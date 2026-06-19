extern crate colored;

use colored::*;

use crate::ui::ui_base::UI;

pub struct SyntaxAlerts;

impl SyntaxAlerts {

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
