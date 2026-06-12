use minify::html::minify;

use std::{error::Error, ffi::OsStr};

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

        // Navigate via a temp file instead of a data: URL — fragment hrefs
        // ("#label-x") do not resolve against data: URLs, which makes Chrome
        // drop every internal link annotation from the printed PDF.
        let temp_path = std::env::temp_dir()
            .join(format!("scimon-render-{}.html", std::process::id()));
        std::fs::write(&temp_path, content)?;
        let url = format!(
            "file:///{}",
            temp_path.display().to_string().replace('\\', "/")
        );

        tab.navigate_to(&url)?
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

        let pdf_options: Option<PrintToPdfOptions> = Some(PrintToPdfOptions {
            print_background: Some(true),
            ..Default::default()
        });

        let contents = tab.print_to_pdf(pdf_options)?;
        let _ = std::fs::remove_file(&temp_path);
        Ok(contents)
    }

}