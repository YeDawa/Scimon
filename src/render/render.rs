use minify::html::minify;

use std::error::Error;
use serde_json;

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

        // Resolve \pageref{} using real DOM positions before paginating.
        // We compute offsetTop of each target and divide by the A4 content
        // height that Chrome uses when printing (794px at 96dpi minus margins).
        // Step 1: inject a probe element that uses CSS counter to get page count,
        // then compute the real screen-px-per-page ratio from scrollHeight.
        let page_count_result = tab.evaluate(r#"
            (function() {
                // Force Chrome to compute print layout by checking how many
                // A4 pages the content spans. We inject a style that sets
                // @page size to A4, then measure scrollHeight vs A4 height.
                // A4 at 96dpi = 1122px. Chrome default print margins = 0.4in
                // top+bottom = ~77px. Content per page = 1045px.
                // We use scrollHeight / 1045 rounded up as page estimate.
                var scrollH = document.body.scrollHeight;
                var pageH = 1045;
                var pages = Math.ceil(scrollH / pageH);
                return JSON.stringify({ scrollH: scrollH, pages: pages, pageH: pageH });
            })()
        "#, false)?;

        // Parse page height from result
        let page_h: f64 = if let Some(serde_json::Value::String(s)) = page_count_result.value
            .as_ref()
            .and_then(|v| Some(v.clone()))
        {
            if let Ok(obj) = serde_json::from_str::<serde_json::Value>(&s) {
                let scroll_h = obj["scrollH"].as_f64().unwrap_or(6000.0);
                let pages = obj["pages"].as_f64().unwrap_or(7.0);
                scroll_h / pages
            } else {
                871.0
            }
        } else {
            871.0
        };

        // Step 2: resolve all [data-ref] using the calibrated page height
        let resolve_script = format!(r#"
            (function() {{
                var PAGE_H = {:.1};

                function offsetTop(el) {{
                    var top = 0;
                    while (el) {{ top += el.offsetTop || 0; el = el.offsetParent; }}
                    return top;
                }}

                function findTarget(id) {{
                    var el = document.getElementById(id);
                    if (el) return el;
                    if (id.startsWith('item-'))
                        return document.getElementById('label-' + id.slice(5));
                    if (id.startsWith('label-'))
                        return document.getElementById('item-' + id.slice(6));
                    return null;
                }}

                document.querySelectorAll('[data-ref]').forEach(function(ref) {{
                    var target = findTarget(ref.getAttribute('data-ref'));
                    if (target) {{
                        ref.textContent = String(Math.floor(offsetTop(target) / PAGE_H) + 1);
                    }}
                }});
            }})()
        "#, page_h);

        tab.evaluate(&resolve_script, false)?;

        let pdf_options: Option<PrintToPdfOptions> = Some(PrintToPdfOptions {
            print_background: Some(true),
            ..Default::default()
        });

        let contents = tab.print_to_pdf(pdf_options)?;
        Ok(contents)
    }

}