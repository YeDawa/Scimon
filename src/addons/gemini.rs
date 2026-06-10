use std::error::Error;
use urlencoding::encode;

use crate::{
    consts::addons::Addons,
    utils::scraping::Scraping,
    ui::success_alerts::SuccessAlerts,
    templates::generic::TemplateGeneric,
    render::render_inject_files::RenderInjectFiles,
};

pub struct Gemini {
    url: String,
    path: String,
    custom_name: Option<String>,
}

impl Gemini {

    pub fn new(url: &str, path: &str, custom_name: Option<&str>) -> Self {
        Self {
            url: url.to_string(),
            path: path.to_string(),
            custom_name: custom_name.map(|s| s.to_string()),
        }
    }

    fn get_content(&self) -> Result<(String, String), Box<dyn Error>> {
        let scraping = Scraping::new(&self.url);
        let content = scraping.get_html()?;

        let title = scraping.title(&content);
        let html_content = scraping.content(&content, Addons::GEMINI_CONTENT_CLASS);
        
        Ok((title, html_content))
    }

    pub fn title(&self) -> Result<String, Box<dyn Error>> {
        let (title, _) = self.get_content()?;
        Ok(title)
    }

    pub fn custom_name(&self, file_name: &str) -> String {
        if let Some(custom_name) = &self.custom_name {
            format!("{}", custom_name.replace(" ", "_"))
        } else {
            format!("{}", &file_name.replace(" ", "_"))
        }
    }

    pub async fn convert(&self) -> Result<(), Box<dyn Error>> {
        let (file_name, html_content) = self.get_content()?;
        
        let css_style = RenderInjectFiles.default_css_style().await;
        let styled_html = TemplateGeneric.base(&css_style, &html_content);

        let file = self.custom_name(&file_name);
        let path = format!("{}{}", &self.path, &file);
        let data_url = encode(&styled_html);

        Scraping::new(&data_url).print_pdf(path.as_str())?;
        SuccessAlerts::download_and_generated_pdf(&file, &self.url);
        Ok(())
    }

}