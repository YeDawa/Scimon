use std::error::Error;

use crate::{
    consts::uris::Uris,

    addons::{
        scihub::SciHub,
        wikipedia::Wikipedia,
    },
    
    utils::{
        url::UrlMisc,
        remote::Remote,
        domain::Domain,
    },
};

pub struct Providers {
    url: String,
}

impl Providers {

    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_owned(),
        }
    }

    pub fn arxiv(&self) -> String {
        let escape_quotes = UrlMisc::escape_quotes(&self.url);
        let domain = Domain::new(&self.url);

        if !domain.check(Uris::PROVIDERS_DOMAINS[1]) {
            escape_quotes.to_owned()
        } else {
            escape_quotes.replace("/abs/", "/pdf/")
        }
    }

    pub fn valid_provider_domain(&self) -> bool {
        let mut valid_domain = false;
        let domain = Domain::new(&self.url);

        for url_domain in &Uris::PROVIDERS_DOMAINS {
            if domain.check(url_domain) {
                valid_domain = true
            }
        }

        valid_domain
    }

    pub async fn generic(&self) -> Result<(String, String), Box<dyn Error>> {
        let request_uri = self.url.to_string();
        let filename = Remote.get_filename(&self.url, true).await?;

        Ok((request_uri, filename))
    }

    pub async fn get_from_provider(&self) -> Result<(String, String), Box<dyn Error>> {
        let domain = Domain::new(&self.url);
        let wikipedia = Wikipedia::new(&self.url);
        let scihub = SciHub::new(&self.url);

        let (request_uri, filename) = if domain.check(Uris::PROVIDERS_DOMAINS[0]) {
            wikipedia.wikipedia()
        } else if domain.check(Uris::PROVIDERS_DOMAINS[2]) {
            wikipedia.wikisource()
        } else if domain.check(Uris::PROVIDERS_DOMAINS[7]) {
            scihub.name()
        } else {
            return self.generic().await;
        };

        Ok((request_uri, filename))
    }

    pub fn check_provider_domain(&self) -> bool {
        let domain = Domain::new(&self.url);

        for url_domain in &Uris::PROVIDERS_DOMAINS {
            if domain.check(url_domain) {
                return true
            }
        }

        false
    }

}
