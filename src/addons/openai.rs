use reqwest;

use serde_json::{
    json,
    Value,
};

use std::{
    env,
    error::Error,
};

use crate::consts::{
    ai::AI,
    folders::Folders,
};

pub struct OpenAI {
    model: String,
}

impl OpenAI {

    pub fn new(model: Option<String>) -> Self {
        let model = model
            .filter(|m| !m.trim().is_empty())
            .unwrap_or_else(|| AI::OPENAI_DEFAULT_MODEL.to_string());

        Self { model }
    }

    fn api_key() -> Result<String, Box<dyn Error>> {
        dotenvy::from_path(
            Folders::APP_FOLDER.join(".env")
        ).ok();

        env::var(AI::OPENAI_API_ENV).map_err(|_| {
            format!(
                "{} is not set. Add it to your .env file (scimon options open-env).",
                AI::OPENAI_API_ENV
            ).into()
        })
    }

    pub async fn generate(&self, prompt: &str) -> Result<String, Box<dyn Error>> {
        let api_key = Self::api_key()?;
        let client = reqwest::Client::new();

        let body = json!({
            "model": self.model,
            "messages": [
                {
                    "role": "system",
                    "content": "You are a helpful assistant that writes well-structured documents in Markdown. Reply only with the Markdown body, without wrapping the whole answer in a code fence."
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ]
        });

        let response = client
            .post(AI::OPENAI_API_REQUEST)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        let data: Value = response.json().await?;

        if !status.is_success() {
            let message = data["error"]["message"]
                .as_str()
                .unwrap_or("unknown error");

            return Err(
                format!("OpenAI API error ({}): {}", status, message).into()
            );
        }

        let content = data["choices"][0]["message"]["content"]
            .as_str()
            .ok_or("OpenAI response did not contain any content")?
            .to_string();

        Ok(content)
    }

}
