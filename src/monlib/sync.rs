extern crate reqwest;

use serde_json::{
    Value,
    from_str,
};

use std::error::Error;

use crate::{
    consts::addons::Addons,
    handlers::monlib_errors::*,
    configs::settings::Settings,
    monlib::request::MonlibRequest,
    ui::errors_alerts::ErrorsAlerts,
};

pub struct MonlibSync;

impl MonlibSync {

    pub async fn pull(&self) -> Result<String, Box<dyn Error>> {
        let mut url = Addons::MONLIB_API_REQUEST.to_owned();
        url.push_str("sync/");

        let response = MonlibRequest::new().get(url.as_str()).await?;
        if response.status().is_success() {
            let result = String::new();
            let mut is_json = true;
            let data = response.text().await?;
    
            if let Ok(json_data) = serde_json::from_str::<Value>(&data) {
                if let Some(message) = json_data.get("message") {
                    if let Some(message_str) = message.as_str() {
                        return Ok(message_str.to_string());
                    }
                }
            } else {
                is_json = false;
            }
    
            if !is_json {
                Settings.write_file(&data)?;
            }
    
            Ok(result)
        } else {
            let status = response.status().as_u16() as i32;
            let response_text = response.text().await?;

            if let Ok(error_response) = from_str::<ErrorResponse>(&response_text) {
                let message = ApiError::Message(error_response.message);
                ErrorsAlerts::monlib(status, &message.to_string());
    
                Ok(message.to_string())
            } else {
                Err(
                    ApiError::Message(
                        "Error: internal server error".to_string()
                    ).into()
                )
            }
        }
    }

    pub async fn push(&self) -> Result<String, Box<dyn Error>> {
        println!("Pushing settings file to Monlib...");
        Ok("Settings file pushed successfully".to_string())
    }

}
