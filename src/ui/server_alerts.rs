extern crate colored;

use colored::*;

use crate::system::general::General;

pub struct ServerAlerts;

impl ServerAlerts {

    pub fn started(port: u16, addr: &str) {
        let current_datetime = General.date_time();
    
        println!(
            "{} Server started successfully on port {}. You can access it in your browser at {}", 
            current_datetime.green().bold(), 
            port.to_string().blue(),
            addr.blue().underline(),
        );
    }

    pub fn to_quit() {
        println!(
            "{}", "Press Ctrl+C again to quit.".dimmed(), 
        );
    }

}