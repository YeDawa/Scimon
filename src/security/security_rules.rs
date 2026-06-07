use reqwest::blocking::get;

use std::{
    fs,
    error::Error,
};

use crate::{
    ui::{
        ui_base::UI,
        security_alerts::SecurityAlerts,
    },

    security::rules::{
        py_rules::*,
        js_rules::*,
        ts_rules::*,
    },
};

pub struct SecurityRules;

impl SecurityRules {

    pub fn read_content(&self, source: &str) -> Result<String, Box<dyn Error>> {
        if source.starts_with("http://") || source.starts_with("https://") {
            let response = get(source)?;
            let text = response.text()?;
            Ok(text)
        } else {
            let content = fs::read_to_string(source)?;
            Ok(content)
        }
    }

    pub fn print_scan_report(&self, source: &str, is_safe: bool, violations: &[String]) {
        UI::section_header("Scan Report", "warning");
        let source = source.replace("\\", "/");
        
        if is_safe {
            SecurityAlerts::secured(&source);
        } else {
            SecurityAlerts::blocked(&source);
            
            for v in violations {
                SecurityAlerts::rule(v);
            }
        }

        println!();
    }

    pub async fn scan_script(&self, source: &str) -> Result<bool, Box<dyn Error>> {
        let language = if source.ends_with(".py") {
            "Python"
        } else if source.ends_with(".js") || source.ends_with(".mjs") || source.ends_with(".cjs") || source.ends_with(".jsx") {
            "JavaScript"
        } else if source.ends_with(".ts") || source.ends_with(".tsx") {
            "TypeScript"
        } else {
            "Unknown"
        };

        match self.read_content(source) {
            Ok(content) => {
                let (violations, is_safe) = if language == "Python" {
                    PyRules.rules(&content)
                } else if language == "JavaScript" {
                    JsRules.rules(&content)
                } else if language == "TypeScript" {
                    TsRules.rules(&content)
                } else {
                    (Vec::new(), true)
                };

                self.print_scan_report(source, is_safe, &violations);
                Ok(is_safe)
            }

            Err(e) => {
                SecurityAlerts::error_in_script(source, &e.to_string());               
                Err(e)
            }
        }
    }

}