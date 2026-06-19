use std::error::Error;

use crate::{
    ui::panic_alerts::PanicAlerts,
    handlers::monlib_handlers::MonlibHandlers,
};

pub struct MonlibPush;

impl MonlibPush {

    pub async fn push(&self, run: &str) -> Result<(), Box<dyn Error>> {
        if !&MonlibHandlers.validator_file(run) {
            PanicAlerts::monlib_invalid_lib();
            return Ok(());
        }

        println!("Monlib publish");
        Ok(())
    }

}
