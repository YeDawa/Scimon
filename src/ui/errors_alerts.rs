extern crate colored;

use colored::*;
use::std::error::Error;

use crate::system::general::General;

pub struct ErrorsAlerts;

impl ErrorsAlerts {

    pub fn env(e: &str) {
        let current_datetime = General.date_time();

        eprintln!(
            "{} Failed to download the file: {}", 
            current_datetime.red().bold(), 
            e.red()
        );
    }

    pub fn generic(e: &str) {
        let current_datetime = General.date_time();

        eprintln!(
            "{} {}", 
            current_datetime.red().bold(), 
            e.red()
        );
    }

    pub fn monlib(status: i32, e: &str) {
        eprintln!(
            "Status: {}. Message: {}",
            status.to_string().yellow(),
            e.red()
        );
    }

    pub fn download(e: Box<dyn Error>, url: &str) {
        let e = e.to_string();
        let current_datetime = General.date_time();

        eprintln!(
            "{} {} (from {})", 
            current_datetime.red().bold(), 
            e.red(), url.cyan()
        );
    }

    pub fn math(file: &str) {
        let current_datetime = General.date_time();

        eprintln!(
            "{} Failed to render the math expression. Format invalid. ({})", 
            current_datetime.red().bold(),
            file.cyan()
        );
    }

    pub fn unsupported_file_type() {
        let current_datetime = General.date_time();

        eprintln!(
            "{} Unsupported file type for compilation.", 
            current_datetime.red().bold()
        );
    }

}
