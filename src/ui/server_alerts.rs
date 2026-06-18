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

    pub fn logs(method: &str, target: &str, status: u16) {
        let status_str = match status {
            200 => status.to_string().green(),
            400..=499 => status.to_string().yellow(),
            _ => status.to_string().red(),
        };

        println!("{} {} {}", status_str, method.bold(), target);
    }

    pub fn to_quit() {
        println!(
            "{}", "Press Ctrl+C again to quit.".dimmed(), 
        );
    }

}