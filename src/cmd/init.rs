use std::{
    fs,
    path::Path,
    error::Error,
};

use crate::{
    consts::bundler::Bundler,

    ui::{
        ui_base::UI,
        success_alerts::SuccessAlerts,
    },
};

pub struct Init;

impl Init {

    pub fn create(&self) -> Result<(), Box<dyn Error>> {
        UI::section_header("Init", "normal");

        Self::write("package.yml", Bundler::MANIFEST)?;
        Self::write("main.mon", Bundler::ENTRY_PACKAGE)?;

        Ok(())
    }

    fn write(name: &str, content: &str) -> Result<(), Box<dyn Error>> {
        if Path::new(name).exists() {
            SuccessAlerts::skipped(name);
            return Ok(());
        }

        fs::write(name, content)?;
        SuccessAlerts::created(name);

        Ok(())
    }

}
