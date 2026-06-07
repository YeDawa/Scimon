extern crate colored;

use colored::*;

use crate::system::general::General;

pub struct SecurityAlerts;

impl SecurityAlerts {

    pub fn secured(source: &str) {
        let current_datetime = General.date_time();

        eprintln!(
            "{} {}: SAFE. No threats detected", 
            current_datetime.red().bold(), 
            source.green().bold()
        );
    }

    pub fn blocked(source: &str) {
        let current_datetime = General.date_time();
        
        eprintln!(
            "{} {}: UNSAFE. Threats detected", 
            current_datetime.red().bold(), 
            source.red().bold()
        );
    }

    pub fn rule(rule: &str) {
        println!("   -> {}", rule.red().bold());
    }

    pub fn error_in_script(source: &str, err: &str) {
        let current_datetime = General.date_time();

        eprintln!(
            "{} Failed to scan the script '{}': {}", 
            current_datetime.red().bold(), 
            source.cyan(), 
            err.red()
        );
    }

    pub fn high_entropy(source: &str, entropy: f64) {
        let current_datetime = General.date_time();

        eprintln!(
            "{} {}: HIGH ENTROPY ({:.2}). Potentially unsafe", 
            current_datetime.red().bold(), 
            source.yellow().bold(), 
            entropy
        );
    }

}
