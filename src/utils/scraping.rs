use std::error::Error;

use futures::StreamExt;

use chromiumoxide::browser::{
    Browser,
    BrowserConfig,
};

use scraper::{
    Html,
    Selector
};

pub struct Scraping {
    url: String,
}

impl Scraping {

    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    pub async fn get_html(&self) -> Result<String, Box<dyn Error>> {
        let config = BrowserConfig::builder()
            .arg("--headless=new")
            .arg("--window-position=-32000,-32000")
            .arg("--disable-gpu")
            .arg("--no-sandbox")
            .build()
            .map_err(|e| format!("Failed to build launch options: {:?}", e))?;

        let (mut browser, mut handler) = Browser::launch(config).await?;

        let browser_handle = tokio::task::spawn(async move {
            while let Some(event) = handler.next().await {
                if event.is_err() {
                    break;
                }
            }
        });

        let page = browser.new_page(&self.url).await?;
        page.wait_for_navigation().await?;

        // Páginas do ChatGPT/Gemini são renderizadas por JS: espera o documento
        // terminar de carregar antes de capturar o HTML.
        page.evaluate(r#"
            new Promise(function(resolve) {
                if (document.readyState === 'complete') {
                    resolve();
                } else {
                    window.addEventListener('load', function() { resolve(); });
                }
                setTimeout(resolve, 15000);
            })
        "#).await?;

        // Pequena folga para a hidratação do SPA assentar o conteúdo no DOM.
        tokio::time::sleep(std::time::Duration::from_millis(2500)).await;

        let content = page.content().await?;
        browser.close().await?;
        let _ = browser_handle.await;

        Ok(content)
    }

    pub fn title(&self, content: &str) -> String {
        let document = Html::parse_document(&content);

        let title_selector = match Selector::parse("title") {
            Ok(selector) => selector,
            Err(_) => return String::from("Untitled"),
        };
        
        return document
            .select(&title_selector)
            .next()
            .map(|e| e.inner_html())
                .unwrap_or_else(|| String::from("Untitled"));
    }

    pub fn content(&self, content: &str, class: &str) -> String {
        let document = Html::parse_document(&content);

        let selector = match Selector::parse(class) {
            Ok(selector) => selector,
            Err(_) => return String::from(""),
        };

        let mut html_content = String::new();

        for element in document.select(&selector) {
            html_content.push_str(&element.inner_html());
        }

        return html_content;
    }

}