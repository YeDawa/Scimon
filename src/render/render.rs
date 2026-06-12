use minify::html::minify;

use std::{
    ffi::OsStr,
    error::Error,
    thread::JoinHandle,
    collections::HashMap,

    io::{
        Read,
        Write,
    },

    net::{
        TcpListener,
        TcpStream,
    },

    sync::{
        Arc,

        atomic::{
            AtomicBool,
            Ordering,
        },
    },
};

use headless_chrome::{
    Browser,
    LaunchOptionsBuilder,
    types::PrintToPdfOptions,
};

use crate::{
    consts::addons::Addons,
    configs::settings::Settings,

    render::{
        render_images::RenderImages,
        render_inject::RenderInject,
    },

    utils::remote::Remote,
};

pub struct Render;

impl Render {

    pub async fn render_content(&self, file: &str, md_content: String) -> Result<String, Box<dyn Error>> {
        let minify_prop = Settings.get("render_markdown.minify_html", "BOOLEAN");
        let template_content = Remote.content(Addons::README_TEMPLATE_LINK).await?;
        let content = RenderInject.content(&file, template_content, md_content);
        let content = RenderImages::new(content).render().await?;

        let output = if minify_prop == true {
            minify(&content)
        } else {
            content
        };

        Ok(output)
    }

    pub async fn connect_to_browser(&self, content: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        // --headless=new uses the new headless code path that never creates a visible
        // window on Windows, unlike the legacy --headless flag.
        // --window-position moves any residual window fully off-screen as a fallback.
        let extra_args: Vec<&OsStr> = vec![
            OsStr::new("--headless=new"),
            OsStr::new("--window-position=-32000,-32000"),
        ];
        let browser = Browser::new(
            LaunchOptionsBuilder::default()
                .headless(true)
                .args(extra_args)
                .build()
                .expect("failed to build launch options"),
        )?;

        let tab = browser.new_tab()?;

        // Serve the document from an ephemeral loopback socket instead of a
        // data: URL — fragment hrefs ("#label-x") do not resolve against
        // data: URLs, which makes Chrome drop every internal link annotation
        // from the printed PDF.
        let (port, stop, server) = Self::serve_content(content.to_string())?;

        tab.navigate_to(&format!("http://127.0.0.1:{}/document.html", port))?
            .wait_until_navigated()?;

        // Wait for MathJax to finish typesetting (if present)
        tab.evaluate(r#"
            new Promise(function(resolve) {
                if (typeof MathJax === 'undefined' || typeof MathJax.startup === 'undefined') {
                    resolve();
                    return;
                }
                MathJax.startup.promise.then(resolve).catch(resolve);
                // Safety timeout: resolve after 5s regardless
                setTimeout(resolve, 5000);
            })
        "#, true)?;

        // Wait for web fonts — fallback metrics paginate differently, which
        // would desync the two print passes below
        tab.evaluate(r#"
            new Promise(function(resolve) {
                if (document.fonts && document.fonts.ready) {
                    document.fonts.ready.then(function() { resolve(); }).catch(resolve);
                } else {
                    resolve();
                }
                setTimeout(resolve, 3000);
            })
        "#, true)?;

        // Resolve \pageref{} placeholders before printing.
        // PAGE_H = 697px is the printable page height calibrated for this
        // template. Forced breaks (\newpage et al.) have zero height in the
        // measured layout, so pagination is simulated: natural breaks every
        // PAGE_H within a segment, plus one page per forced-break div
        // (\cleardoublepage additionally skips to the next odd page).
        tab.evaluate(r#"
            (function() {
                var PAGE_H = 697;

                function offsetTop(el) {
                    var top = 0;
                    while (el) { top += el.offsetTop || 0; el = el.offsetParent; }
                    return top;
                }

                var breaks = [];
                document.querySelectorAll(
                    '[style*="break-after: page"], [style*="page-break-after: always"], [style*="break-after: right"]'
                ).forEach(function(el) {
                    breaks.push({
                        y: offsetTop(el),
                        right: (el.getAttribute('style') || '').indexOf('break-after: right') !== -1
                    });
                });
                breaks.sort(function(a, b) { return a.y - b.y; });

                function pageOf(targetY) {
                    var page = 1, segStart = 0;
                    for (var i = 0; i < breaks.length && breaks[i].y <= targetY; i++) {
                        page += Math.floor((breaks[i].y - segStart) / PAGE_H); // natural breaks
                        page += 1;                                             // the forced break
                        if (breaks[i].right && page % 2 === 0) page += 1;      // next odd page
                        segStart = breaks[i].y;
                    }
                    return page + Math.floor((targetY - segStart) / PAGE_H);
                }

                function findTarget(id) {
                    var el = document.getElementById(id);
                    if (el) return el;
                    if (id.startsWith('item-'))
                        return document.getElementById('label-' + id.slice(5));
                    if (id.startsWith('label-'))
                        return document.getElementById('item-' + id.slice(6));
                    return null;
                }

                document.querySelectorAll('[data-ref]').forEach(function(ref) {
                    var target = findTarget(ref.getAttribute('data-ref'));
                    if (target) {
                        ref.textContent = String(pageOf(offsetTop(target)));
                    }
                });
            })()
        "#, false)?;

        let pdf_options = || Some(PrintToPdfOptions {
            print_background: Some(true),
            ..Default::default()
        });

        // The first pass prints with the simulated page numbers; the PDF's
        // own named destinations then reveal the exact page of every \label.
        // Iterate to a fixed point: late layout shifts (slow web fonts that
        // beat the wait's timeout, images) re-paginate the document between
        // prints, so keep reprinting until the destinations agree with the
        // numbers shown in the text.
        let mut contents = tab.print_to_pdf(pdf_options())?;

        if content.contains("data-ref") {
            for _ in 0..3 {
                let destinations = Self::destination_pages(&contents);
                if destinations.is_empty() {
                    break;
                }

                let pages = serde_json::to_string(&destinations)?;
                let changed = tab.evaluate(&format!(r#"
                    (function() {{
                        var pages = {};
                        var changed = false;
                        document.querySelectorAll('[data-ref]').forEach(function(ref) {{
                            var page = pages[ref.getAttribute('data-ref')];
                            if (page !== undefined && ref.textContent !== String(page)) {{
                                ref.textContent = String(page);
                                changed = true;
                            }}
                        }});
                        return changed;
                    }})()
                "#, pages), false)?;

                // numbers already match the real pages — `contents` is final
                if changed.value != Some(serde_json::Value::Bool(true)) {
                    break;
                }
                contents = tab.print_to_pdf(pdf_options())?;
            }
        }

        // release the accept loop and let the server thread finish
        stop.store(true, Ordering::SeqCst);
        let _ = TcpStream::connect(("127.0.0.1", port));
        let _ = server.join();

        Ok(contents)
    }

    // Map every named destination ("label-x") in the printed PDF to its
    // 1-based page number — the ground truth for \pageref resolution.
    fn destination_pages(pdf: &[u8]) -> HashMap<String, u32> {
        let mut map = HashMap::new();

        let Ok(doc) = lopdf::Document::load_mem(pdf) else { return map };
        let page_numbers: HashMap<lopdf::ObjectId, u32> = doc
            .get_pages()
            .into_iter()
            .map(|(number, id)| (id, number))
            .collect();

        let dests = doc.trailer.get(b"Root")
            .and_then(|root| root.as_reference())
            .and_then(|id| doc.get_dictionary(id))
            .and_then(|catalog| catalog.get(b"Dests"))
            .and_then(|dests| match dests {
                lopdf::Object::Reference(id) => doc.get_dictionary(*id),
                lopdf::Object::Dictionary(dict) => Ok(dict),
                _ => Err(lopdf::Error::Type),
            });

        let Ok(dests) = dests else { return map };
        for (name, value) in dests.iter() {
            let array = match value {
                lopdf::Object::Array(array) => array.clone(),
                lopdf::Object::Reference(id) => match doc.get_object(*id).and_then(|o| o.as_array()) {
                    Ok(array) => array.clone(),
                    Err(_) => continue,
                },
                _ => continue,
            };

            if let Some(lopdf::Object::Reference(page_id)) = array.first() {
                if let Some(&number) = page_numbers.get(page_id) {
                    map.insert(String::from_utf8_lossy(name).to_string(), number);
                }
            }
        }

        map
    }

    // Serve `content` over HTTP on an ephemeral loopback port, answering
    // every request with the same document until the stop flag is set.
    fn serve_content(
        content: String,
    ) -> Result<(u16, Arc<AtomicBool>, JoinHandle<()>), Box<dyn Error>> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let port = listener.local_addr()?.port();

        let stop = Arc::new(AtomicBool::new(false));
        let flag = stop.clone();

        let handle = std::thread::spawn(move || {
            for stream in listener.incoming() {
                if flag.load(Ordering::SeqCst) {
                    break;
                }
                let Ok(mut stream) = stream else { continue };

                // request line and headers are irrelevant — drain best-effort
                let mut request = [0u8; 2048];
                let _ = stream.read(&mut request);

                let header = format!(
                    "HTTP/1.1 200 OK\r\n\
                     Content-Type: text/html; charset=utf-8\r\n\
                     Content-Length: {}\r\n\
                     Connection: close\r\n\r\n",
                    content.len()
                );
                let _ = stream.write_all(header.as_bytes());
                let _ = stream.write_all(content.as_bytes());
            }
        });

        Ok((port, stop, handle))
    }

}