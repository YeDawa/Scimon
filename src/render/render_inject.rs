use minify::html;
use std::error::Error;

use crate::{
    syntax::vars::Vars,
    templates::generic::TemplateGeneric,

    utils::{
        str::StrUtils,
        remote::Remote,
    },

    consts::{
        global::Global,
        addons::Addons,
    },
};

pub struct RenderInject;

impl RenderInject {

    pub fn content(&self, file: &str, contents: String, markdown_html: String) -> String {
        let title = format!(
            "{}: {}: README", StrUtils.capitalize(Global::APP_NAME), &file.replace(
                ".md", ""
            )
        );

        let bundle_js_link = format!("{}static/dist/bundle.js", Addons::README_TEMPLATE_LINK);
        let bundle_css_link = format!("{}static/dist/bundle.css", Addons::README_TEMPLATE_LINK);
        
        contents.replace(
            "{{ page_title }}", &title
        ).replace(
            "{{ dist_bundle_css }}", &bundle_css_link
        ).replace(
            "{{ dist_bundle_js }}", &bundle_js_link
        ).replace(
            "{{ markdown_content }}", &markdown_html
        )
    }

    pub async fn html_content(&self, contents: &str, html_content: String) -> Result<String, Box<dyn Error>> {
        let css_cdn = if let Some(url) = Vars.get_style(contents) {
            url
        } else {
            Addons::DEFAULT_CSS_STYLE.to_string()
        };

        let css_style = Remote.content(&css_cdn).await?;
        let html = TemplateGeneric.base(&css_style, &html_content);
        let html = html::minify(html.as_str());

        Ok(html)
    }
    
}
