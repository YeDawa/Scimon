use serde_json::from_str;
use std::{
    fs,
    error::Error,
};

use crate::{
    cmd::bundle::Bundle,
    consts::addons::Addons,
    handlers::monlib_errors::*,
    monlib::request::MonlibRequest,
    handlers::monlib_handlers::MonlibHandlers,

    ui::{
        ui_base::UI,
        panic_alerts::PanicAlerts,
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },
};

pub struct MonlibPush;

impl MonlibPush {

    pub async fn push(&self) -> Result<(), Box<dyn Error>> {
        if !MonlibHandlers.validator_file("main.mon") {
            PanicAlerts::monlib_invalid_lib();
            return Ok(());
        }

        let bundle = Bundle.pack()?;
        let filename = bundle.file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| "package.scpkg".to_string());

        let bytes = fs::read(&bundle)?;
        let mut url = Addons::MONLIB_API_REQUEST.to_owned();
        url.push_str("packages/upload");

        UI::section_header("Publishing", "normal");
        let response = MonlibRequest::new().upload(&url, "file", &filename, bytes).await?;
        let status = response.status();

        if status.is_success() {
            SuccessAlerts::pushed(&filename);
            let _ = fs::remove_file(&bundle);
        } else {
            let code = status.as_u16() as i32;
            let text = response.text().await.unwrap_or_default();

            let message = from_str::<ErrorResponse>(&text)
                .map(|error| error.message)
                .unwrap_or_else(|_| "Error: internal server error".to_string());

            ErrorsAlerts::monlib(code, &message);
        }

        Ok(())
    }

}
