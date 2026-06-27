extern crate reqwest;
use reqwest::{
    Error,
    Client,
    Response,

    multipart::{
        Form,
        Part,
    },
};

use crate::{
    configs::env::Env,
    consts::addons::Addons,
};

pub struct MonlibRequest {
    api_key: String
}

impl MonlibRequest {

    pub fn new() -> Self {
        let api_key = Env.env_var(Addons::MONLIB_API_ENV);

        Self {
            api_key
        }
    }

    pub async fn get(&self, url: &str) -> Result<Response, Error> {
        let client = Client::builder().danger_accept_invalid_certs(true).build().unwrap();
        let api_key = "Bearer ".to_string() + &self.api_key;

        let response = client
            .get(url)
            .header("Authorization", api_key)
        .send()
        .await?;

        Ok(response)
    }

    pub async fn upload(&self, url: &str, field: &str, filename: &str, bytes: Vec<u8>) -> Result<Response, Error> {
        let api_key = "Bearer ".to_string() + &self.api_key;
        let client = Client::builder().danger_accept_invalid_certs(true).build().unwrap();

        let part = Part::bytes(bytes)
            .file_name(filename.to_string())
            .mime_str("application/gzip")?;

        let form = Form::new().part(field.to_string(), part);
        let response = client
            .post(url)
            .header("Authorization", api_key)
            .multipart(form)
            .send()
            .await?;

        Ok(response)
    }

}
