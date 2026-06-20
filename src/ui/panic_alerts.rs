extern crate colored;

use colored::*;
use::std::process::exit;

use crate::ui::ui_base::UI;

pub struct PanicAlerts;

impl PanicAlerts {

    fn message(message: &str) {
        eprintln!(
            "{}: {}",
            "Panic Error".bold().red(),
            message.bold()
        );

        exit(1);
    }

    pub fn compress_level() {
        UI::section_header("compress level invalid", "error");
        Self::message("The compresss level set is invalid.");
    }

    pub fn monlib_invalid_lib() {
        UI::section_header("monlib invalid library", "error");
        Self::message("The library set is invalid.");
    }

}
