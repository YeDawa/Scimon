use std::{
    fs::write,
    error::Error,
};

use crate::{
    consts::ai::AI,
    render::render::Render,
    utils::scraping::Scraping,
    ui::success_alerts::SuccessAlerts,
    render::render_images::RenderImages,
    templates::generic::TemplateGeneric,
    helpers::chatgpt_cleaner::ChatGPTCleaner,
    render::render_inject_files::RenderInjectFiles,
};

pub struct ChatGPT {
    url: String,
    path: String,
    custom_name: Option<String>,
}

impl ChatGPT {

    pub fn new(url: &str, path: &str, custom_name: Option<&str>) -> Self {
        Self {
            url: url.to_string(),
            path: path.to_string(),
            custom_name: custom_name.map(|s| s.to_string()),
        }
    }

    async fn get_content(&self) -> Result<(String, String), Box<dyn Error>> {
        let scraping = Scraping::new(&self.url);
        let content = scraping.get_html().await?;

        let title = scraping.title(&content);
        let html_content = scraping.content(&content, AI::CHATGPT_CONTENT_CLASS);

        let stripped_html_header = ChatGPTCleaner.strip_html_header(&html_content);
        let stripped_reasoning_text = ChatGPTCleaner.strip_reasoning_text(&stripped_html_header);

        Ok((title, stripped_reasoning_text))
    }

    pub async fn title(&self) -> Result<String, Box<dyn Error>> {
        let (title, _) = self.get_content().await?;
        Ok(title)
    }

    pub fn custom_name(&self, file_name: &str) -> String {
        if let Some(custom_name) = &self.custom_name {
            custom_name.replace(" ", "_").to_string()
        } else {
            file_name.replace(" ", "_").to_string()
        }
    }

    pub async fn convert(&self) -> Result<String, Box<dyn Error>> {
        let (file_name, html_content) = self.get_content().await?;
        
        let css_style = RenderInjectFiles.css_style().await;
        let styled_html = TemplateGeneric.base(&css_style, &html_content);
        let styled_html = RenderImages::new(styled_html).render().await?;

        let file = self.custom_name(&file_name);
        let path = format!("{}{}", &self.path, &file);

        let pdf_contents = Render.connect_to_browser(&styled_html).await?;
        write(&path, pdf_contents)?;

        SuccessAlerts::download_and_generated_pdf(&file, &self.url);
        Ok(path)
    }

}