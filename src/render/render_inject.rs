use minify::html;
use std::error::Error;

use crate::{
    syntax::vars::Vars,
    utils::remote::Remote,
    consts::addons::Addons,
    templates::generic::TemplateGeneric,
};

pub struct RenderInject;

impl RenderInject {

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
