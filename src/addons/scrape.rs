extern crate reqwest;

use std::error::Error;
use serde::Deserialize;

use crate::{
    args_cli::Flags,
    cmd::tasks::Tasks,
    ui::errors_alerts::ErrorsAlerts,
    
    consts::{
        addons::Addons,
        folders::Folders,
    },
};

#[derive(Debug, Deserialize)]
struct Item {
    url: String,
    encrypted: bool,
}

#[derive(Debug, Deserialize)]
struct Response {
    #[serde(default)]
    total: Option<u32>,

    #[serde(default)]
    list: Vec<Item>,

    #[serde(default)]
    message: String,

    success: Option<bool>
}

pub struct Scrape;

impl Scrape {

    async fn fetch_items(&self, url: &str) -> Result<Response, Box<dyn std::error::Error>> {
        let url_with_param = Addons::SCIMON_SCRAPE_API_ENDPOINT.to_string() + url;
        let response = reqwest::get(&url_with_param).await?;
        let body = response.bytes().await?;
        
        let data = serde_json::from_slice(&body)?;
        Ok(data)
    }

    pub async fn get(&self, flags: &Flags, url: &str) -> Result<(), Box<dyn Error>> {
        match self.fetch_items(url).await {
            Ok(response) => {
                if let Some(success) = response.success {
                    if !success {
                        ErrorsAlerts::generic(&response.message);
                        return Ok(())
                    }
                }

                if let Some(total) = response.total {
                    if total > 0 {
                        if !response.list.is_empty() {
                            for item in &response.list {
                                if !item.encrypted {
                                    let path = Folders::SCRAPE_FOLDER.to_str().unwrap().to_string();
                                    let url = &item.url;

                                    Tasks.download(
                                        None, url, &path, None, flags, 0,
                                    ).await?;
                                }
                            }
                        }
                    } else {
                        ErrorsAlerts::generic(&response.message);
                    }
                }
            }

            Err(e) => ErrorsAlerts::generic(&e.to_string())
        }
    
        Ok(())
    }
    
}
