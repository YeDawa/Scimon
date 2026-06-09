use minify::html::minify;

use std::error::Error;

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

    utils::{
        base64::Base64,
        remote::Remote,
    },
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
        let browser = Browser::new(
            LaunchOptionsBuilder::default().build().expect(""),
        )?;

        let tab = browser.new_tab()?;

        tab.navigate_to(&Base64::encode_html(content))?
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
        // PAGE_H = 871px is calibrated from: scrollHeight=6098, total_pages=7
        // for this specific template. Adjust if the template changes significantly.
        tab.evaluate(r#"
            (function() {
                var PAGE_H = 697;

                function offsetTop(el) {
                    var top = 0;
                    while (el) { top += el.offsetTop || 0; el = el.offsetParent; }
                    return top;
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
                        ref.textContent = String(Math.floor(offsetTop(target) / PAGE_H) + 1);
                    }
                });
            })()
        "#, false)?;

        let pdf_options: Option<PrintToPdfOptions> = Some(PrintToPdfOptions {
            print_background: Some(true),
            ..Default::default()
        });

        let contents = tab.print_to_pdf(pdf_options)?;
        Ok(contents)
    }

}