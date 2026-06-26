use crate::{
    utils::remote::Remote,
    handlers::static_files::StaticFiles,
};

pub struct RenderInjectFiles;

impl RenderInjectFiles {

    pub async fn css_style(&self) -> String {
        let css_cdn = StaticFiles.get_default_css_style().to_string();
        Remote.content(&css_cdn).await.unwrap_or_default()
    }

    pub async fn latex_css_style(&self) -> String {
        let css_cdn = StaticFiles.get_default_latex_css_style().to_string();
        Remote.content(&css_cdn).await.unwrap_or_default()
    }

    pub async fn latex_js_script(&self) -> String {
        let js_cdn = StaticFiles.get_default_latex_js_script().to_string();
        Remote.content(&js_cdn).await.unwrap_or_default()
    }

}