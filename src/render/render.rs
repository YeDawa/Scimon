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

        tab.evaluate(
    r#"
            (function() {
                var refs = document.querySelectorAll('[data-ref]');
                return JSON.stringify({
                    count: refs.length,
                    first: refs[0] ? refs[0].getAttribute('data-ref') : null,
                    firstText: refs[0] ? refs[0].textContent : null,
                    resolveExists: typeof window.resolvePageRefs
                });
            })()
            "#,
            false,
        )?;

        let pdf_options: Option<PrintToPdfOptions> = Some(PrintToPdfOptions {
            print_background: Some(true),
            ..Default::default()
        });

        let contents = tab.print_to_pdf(pdf_options)?;
        Ok(contents)
    }

}