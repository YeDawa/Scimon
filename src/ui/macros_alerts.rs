extern crate colored;

use colored::*;

use crate::{
    ui::ui_base::UI,
    system::general::General,
};

pub struct MacrosAlerts;

impl MacrosAlerts {

    pub fn ignore(line: &str) {
        let current_datetime = General.date_time();
        let url = line.replace(" !ignore", "");

        println!(
            "{} The url {} was ignored", 
            current_datetime.green().bold(), 
            url.blue()
        );
    }

    pub fn readme(file: &str) {
        let current_datetime = General.date_time();

        UI::section_header("readme", "info");
        println!(
            "{} README file rendered with successfully in {}", 
            current_datetime.blue().bold(), 
            file.cyan()
        );
    }

}
