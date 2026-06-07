extern crate reqwest;

use reqwest::{
    Error,
    Client,
    Response,
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
        Self { api_key }
    }

    pub async fn get(&self, url: &str) -> Result<Response, Error> {
        let client = Client::builder().danger_accept_invalid_certs(true).build().unwrap();

        let response = client
            .get(url)
            .header("API-Key", &self.api_key)
        .send()
        .await?;

        Ok(response)
    }

}
