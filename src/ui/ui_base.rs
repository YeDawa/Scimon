extern crate colored;
extern crate figlet_rs;

use colored::*;
use figlet_rs::FIGlet;
use indicatif::ProgressStyle;

use crate::{
    utils::str::StrUtils,
    consts::global::Global,
    system::general::General,
    configs::settings::Settings,
};

pub struct UI;

impl UI {

    pub fn header() {
        if Settings.get("ui.show_header", "BOOLEAN") == true {
            let name = StrUtils.capitalize(Global::APP_NAME);
            let standard_font = FIGlet::standard().unwrap();
            
            if let Some(title) = standard_font.convert(&name) {
                println!("{}", &title.to_string().bold().cyan());
                println!("");
                println!("Version: {}", Global::APP_VERSION.yellow());
                println!("Homepage: {} • {}", Global::APP_HOMEPAGE.blue(), Global::APP_AUTHOR.green());
                println!("Started in: {}", General.date_time().magenta());
            }
        }
    }
    
    pub fn section_header(text: &str, level: &str) {
        let text = text.to_uppercase();

        let message = match level {
            "normal" => text.bold(),
            "info" => text.bold().blue(),
            "warning" => text.bold().yellow(),
            "error" => text.bold().red(),
            "success" => text.bold().green(),
            _ => text.bold(),
        };

        println!("");
        println!("{}", message);
    }

    pub fn pb_template() -> ProgressStyle {
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})"
        ).unwrap().progress_chars("█░")
    }

}
