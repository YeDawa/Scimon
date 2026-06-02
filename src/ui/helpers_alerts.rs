extern crate colored;

use colored::*;

use crate::system::general::General;

pub struct HelpersAlerts;

impl HelpersAlerts {

    pub fn pdfium_not_found() {
        let current_datetime = General.date_time();
    
        println!(
            "{} PDFium engine not found. Attempting to download... (This only happens the first time)", 
            current_datetime.yellow().bold(), 
        );
    }

    pub fn pdfium_downloaded() {
        let current_datetime = General.date_time();
    
        println!(
            "{} PDFium engine successfully downloaded and installed.", 
            current_datetime.green().bold(), 
        );
    }

    pub fn pdfium_download_failed(error: &str) {
        let current_datetime = General.date_time();
    
        eprintln!(
            "{} Failed to download the PDF engine: {}", 
            current_datetime.red().bold(), 
            error
        );
    }

}
