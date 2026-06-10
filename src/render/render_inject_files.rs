use crate::{
    utils::remote::Remote,
    consts::addons::Addons,
};

pub struct RenderInjectFiles;

impl RenderInjectFiles {

    pub async fn css_style(&self) -> String {
        let css_cdn = Addons::DEFAULT_CSS_STYLE.to_string();
        Remote.content(&css_cdn).await.unwrap_or_default()
    }

    pub async fn latex_css_style(&self) -> String {
        let css_cdn = Addons::DEFAULT_LATEX_CSS_STYLE.to_string();
        Remote.content(&css_cdn).await.unwrap_or_default()
    }

    pub async fn latex_js_script(&self) -> String {
        let js_cdn = Addons::DEFAULT_LATEX_JS_SCRIPT.to_string();
        Remote.content(&js_cdn).await.unwrap_or_default()
    }

}