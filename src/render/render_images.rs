use reqwest;

use scraper::{
    Html, 
    Selector
};

use base64::{
    Engine as _,
    engine::general_purpose, 
};

use std::error::Error;

pub struct RenderImages {
    content: String,
}

impl RenderImages {

    pub fn new(content: String) -> Self {
        Self {
            content
        }
    }

    pub async fn render(&mut self) -> Result<String, Box<dyn Error>> {
        // Coleta os `src` antes de qualquer await: os tipos do `scraper`
        // (Html/ElementRef) não são Send e não podem cruzar um ponto de await.
        let sources: Vec<String> = {
            let html = Html::parse_document(&self.content);
            let selector = Selector::parse("img").unwrap();

            html.select(&selector)
                .filter_map(|img| img.value().attr("src"))
                .filter(|src| src.starts_with("http"))
                .map(|src| src.to_string())
                .collect()
        };

        for src in sources {
            let response = reqwest::get(&src).await?;

            let content_type = response
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|ct| ct.to_str().ok())
                .unwrap_or("image/jpeg")
                .to_string();

            let img = response.bytes().await?;
            let img = general_purpose::STANDARD.encode(&img);
            let img = format!("data:{};base64,{}", content_type, img);

            // O `scraper` devolve o src decodificado (ex.: `&s=10`), mas no HTML
            // bruto ele pode estar com entidades (`&amp;s=10`). Substitui ambas as
            // formas para garantir que a imagem seja embutida.
            let encoded_src = src.replace('&', "&amp;");

            self.content = self.content.replace(&src, &img);
            if encoded_src != src {
                self.content = self.content.replace(&encoded_src, &img);
            }
        }

        Ok(self.content.clone())
    }

}