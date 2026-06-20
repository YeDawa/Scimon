extern crate colored;

use colored::*;

use crate::{
    ui::ui_base::UI,
    system::general::General,
};

pub struct SyntaxAlerts;

impl SyntaxAlerts {

    pub fn valid(file: &str) {
        let current_datetime = General.date_time();

        UI::section_header("syntax check", "success");

        println!(
            "{} No issues found in {}",
            current_datetime.green().bold(),
            file.cyan()
        );
    }

    pub fn error(line: usize, content: &str, message: &str) {
        UI::section_header("syntax error", "error");

        eprintln!(
            "{} {}",
            format!("Line {}:", line).red().bold(),
            content.trim()
        );

        eprintln!("  {} {}", "→".red().bold(), message.yellow());
    }

}
