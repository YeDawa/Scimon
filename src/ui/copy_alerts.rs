extern crate colored;

use colored::*;

use crate::system::general::General;

pub struct CopyAlerts;

impl CopyAlerts {

    pub fn copied(file: &str, destination: &str) {
        let current_datetime = General.date_time();

        println!(
            "{} Copied {} to: {}",
            current_datetime.green().bold(),
            file.blue(),
            destination.cyan(),
        );
    }

    pub fn completed(destination: &str) {
        let current_datetime = General.date_time();

        println!(
            "{} All files in the folder have been copied to {}.",
            current_datetime.green().bold(),
            destination.blue(),
        );
    }

}
