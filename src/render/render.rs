use minify::html::minify;

use std::{
    error::Error,
    thread::JoinHandle,
    collections::HashMap,

    io::{
        Read, 
        Write
    },

    net::{
        TcpListener, 
        TcpStream
    },

    sync::{
        Arc,

        atomic::{
            Ordering,
            AtomicBool, 
        },
    },
};

use chromiumoxide::{
    cdp::browser_protocol::page::PrintToPdfParams,

    browser::{
        Browser, 
        BrowserConfig
    },
};
use futures::StreamExt;

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
        let config = BrowserConfig::builder()
            .arg("--headless=new")
            .arg("--window-position=-32000,-32000")
            .arg("--disable-gpu")
            .arg("--no-sandbox")
            .build()
            .map_err(|e| format!("Failed to build browser config: {:?}", e))?;

        let (mut browser, mut handler) = Browser::launch(config).await?;

        let browser_handle = tokio::task::spawn(async move {
            while let Some(event) = handler.next().await {
                if event.is_err() {
                    break;
                }
            }
        });

        let page = browser.new_page("about:blank").await?;

        let (port, stop, server) = Self::serve_content(content.to_string())?;

        page.goto(&format!("http://127.0.0.1:{}/document.html", port)).await?;
        page.wait_for_navigation().await?;

        // Wait for MathJax to finish typesetting
        page.evaluate(r#"
            new Promise(function(resolve) {
                if (typeof MathJax === 'undefined' || typeof MathJax.startup === 'undefined') {
                    resolve();
                    return;
                }
                MathJax.startup.promise.then(resolve).catch(resolve);
                setTimeout(resolve, 5000);
            })
        "#).await?;

        // Wait for web fonts
        page.evaluate(r#"
            new Promise(function(resolve) {
                if (document.fonts && document.fonts.ready) {
                    document.fonts.ready.then(function() { resolve(); }).catch(resolve);
                } else {
                    resolve();
                }
                setTimeout(resolve, 3000);
            })
        "#).await?;

        // Force lazy images to load and wait for them to decode before printing
        page.evaluate(r#"
            new Promise(function(resolve) {
                var imgs = Array.prototype.slice.call(document.querySelectorAll('img'));
                Promise.all(imgs.map(function(img) {
                    img.loading = 'eager';
                    if (img.complete && img.naturalWidth > 0) {
                        return Promise.resolve();
                    }
                    if (img.decode) {
                        return img.decode().catch(function() {});
                    }
                    return new Promise(function(done) {
                        img.addEventListener('load', done);
                        img.addEventListener('error', done);
                    });
                })).then(resolve).catch(resolve);
                setTimeout(resolve, 10000);
            })
        "#).await?;

        // Resolve \pageref{} placeholders
        page.evaluate(r#"
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
                        page += Math.floor((breaks[i].y - segStart) / PAGE_H);
                        page += 1;
                        if (breaks[i].right && page % 2 === 0) page += 1;
                        segStart = breaks[i].y;
                    }
                    return page + Math.floor((targetY - segStart) / PAGE_H);
                }

                function findTarget(id) {
                    var el = document.getElementById(id);
                    if (el) return el;
                    if (id.startsWith('item-')) return document.getElementById('label-' + id.slice(5));
                    if (id.startsWith('label-')) return document.getElementById('item-' + id.slice(6));
                    return null;
                }

                document.querySelectorAll('[data-ref]').forEach(function(ref) {
                    var target = findTarget(ref.getAttribute('data-ref'));
                    if (target) {
                        ref.textContent = String(pageOf(offsetTop(target)));
                    }
                });
            })()
        "#).await?;

        // Configuração nativa de PDF do chromiumoxide
        let pdf_options = PrintToPdfParams::builder()
            .print_background(true)
            .build();

        let mut contents = page.pdf(pdf_options.clone()).await?;

        if content.contains("data-ref") {
            for _ in 0..3 {
                let destinations = Self::destination_pages(&contents);
                if destinations.is_empty() {
                    break;
                }

                let pages_json = serde_json::to_string(&destinations)?;
                
                // O evaluate do chromiumoxide permite extrair o valor retornado
                let eval_result = page.evaluate(format!(r#"
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
                "#, pages_json)).await?;

                // Converte o retorno do JS para um bool no Rust
                let changed = eval_result.into_value::<bool>().unwrap_or(false);

                if !changed {
                    break;
                }
                
                // Gera o PDF novamente se houve mudança
                contents = page.pdf(pdf_options.clone()).await?;
            }
        }

        // Limpeza (desliga o servidor local e o browser)
        stop.store(true, Ordering::SeqCst);
        let _ = TcpStream::connect(("127.0.0.1", port));
        let _ = server.join();
        
        browser.close().await?;
        let _ = browser_handle.await;

        Ok(contents)
    }

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