extern crate colored;

use colored::*;

pub struct Logs;

impl Logs {

    pub fn print(&self, method: &str, target: &str, status: u16) {
        let status_str = match status {
            200 => status.to_string().green(),
            400..=499 => status.to_string().yellow(),
            _ => status.to_string().red(),
        };

        println!("{} {} {}", status_str, method.bold(), target);
    }

}