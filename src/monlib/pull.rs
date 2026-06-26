extern crate reqwest;

use reqwest::Response;
use serde_json::from_str;

use std::{
    fs,
    error::Error,
};

use crate::{
    args_cli::Flags,
    cmd::bundle::Bundle,
    consts::addons::Addons,
    handlers::monlib_errors::*,
    monlib::request::MonlibRequest,

    ui::{
        ui_base::UI,
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },
};

pub struct MonlibPull;

impl MonlibPull {

    pub async fn pull(&self, run: &str, flags: &Flags) -> Result<String, Box<dyn Error>> {
        UI::section_header("Pulling", "normal");

        let (package, version) = match run.split_once('@') {
            Some((package, version)) => (package, Some(version)),
            None => (run, None),
        };

        let mut url = Addons::MONLIB_API_REQUEST.to_owned();
        url.push_str("packages/");
        url.push_str(package);

        if let Some(version) = version {
            url.push('/');
            url.push_str(version);
        }

        url.push_str("/download");
        let response = MonlibRequest::new().get(&url).await?;
        let status = response.status();

        if !status.is_success() {
            let code = status.as_u16() as i32;
            let text = response.text().await.unwrap_or_default();

            let message = from_str::<ErrorResponse>(&text)
                .map(|error| error.message)
                .unwrap_or_else(|_| "Error: internal server error".to_string());

            ErrorsAlerts::monlib(code, &message);
            return Ok(message);
        }

        let filename = Self::filename(&response).unwrap_or_else(|| match version {
            Some(version) => format!("{}-{}.scpkg", package, version),
            None => format!("{}.scpkg", package),
        });

        let bytes = response.bytes().await?;
        fs::write(&filename, &bytes)?;

        SuccessAlerts::pulled(&filename);
        let result = Bundle.run(&filename, flags).await;
        let _ = fs::remove_file(&filename);
        result?;

        Ok(filename)
    }

    fn filename(response: &Response) -> Option<String> {
        let value = response.headers()
            .get("content-disposition")?
            .to_str()
            .ok()?;

        let raw = value.split("filename=").nth(1)?;
        let name = raw.split(';').next()?.trim().trim_matches('"');

        if name.is_empty() {
            None
        } else {
            Some(name.to_string())
        }
    }

}
