extern crate open;

use regex::Regex;

use crate::{
    consts::addons::Addons,
    configs::settings::Settings,
    regexp::regex_blocks::BlocksRegExp,

    ui::{
        ui_base::UI,
        panic_alerts::PanicAlerts,
    },
};

pub struct Vars;

impl Vars {

    pub fn get_path(&self, contents: &str) -> String {
        let path_pattern = Regex::new(BlocksRegExp::GET_PATH_VAR).unwrap();

        let path = path_pattern.captures(contents)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str())
            .unwrap_or_else(|| {
                PanicAlerts::path_var();
                ""
            });

        path.to_string()
    }

    pub async fn get_open(&self, contents: &str, no_open: bool) -> Option<String> {
        if !no_open {
            let open_pattern = Regex::new(BlocksRegExp::GET_OPEN_VAR).unwrap();
        
            if let Some(caps) = open_pattern.captures(contents) {
                let url = caps.get(1).map(|m| m.as_str().to_string())?;

                let open_url = if Settings.get("general.urlfilter_open", "BOOLEAN") == true {
                    format!("{}{}", Addons::SCIMON_URLFILTER_API_ENDPOINT, url)
                } else {
                    url
                };

                open::that(&open_url).unwrap();
                
                None
            } else {
                None
            }
        } else {
            None
        }
    }

    pub async fn get_readme(&self, contents: &str) -> Option<String> {
        let readme_pattern = Regex::new(BlocksRegExp::GET_README_VAR).unwrap();
    
        if let Some(caps) = readme_pattern.captures(contents) {
            caps.get(1).map(|m| m.as_str().to_string())
        } else {
            None
        }
    }

    pub fn get_compress(&self, contents: &str) -> Option<String> {
        let readme_pattern = Regex::new(BlocksRegExp::GET_COMPRESS_VAR).unwrap();
    
        if let Some(caps) = readme_pattern.captures(contents) {
            caps.get(1).map(|m| m.as_str().to_string())
        } else {
            None
        }
    }

    pub fn get_covers(&self, contents: &str) -> Option<String> {
        let readme_pattern = Regex::new(BlocksRegExp::GET_COVERS_VAR).unwrap();
    
        if let Some(caps) = readme_pattern.captures(contents) {
            caps.get(1).map(|m| m.as_str().to_string())
        } else {
            None
        }
    }

    pub fn get_print(&self, contents: &str) -> Option<String> {
        let print_pattern = Regex::new(BlocksRegExp::GET_PRINT_VAR).unwrap();
        
        if let Some(caps) = print_pattern.captures(contents) {
            UI::section_header("print", "normal");
    
            let print = caps.get(1).map(|m| m.as_str().to_string())?;
            println!("{}", print);
    
            None
        } else {
            None
        }
    }

    pub fn get_all_math(&self, contents: &str) -> Vec<(String, String)> {
        let math_pattern = Regex::new(BlocksRegExp::GET_MATH_VAR).unwrap();
        let mut results = Vec::new();

        for line in contents.lines() {
            if let Some(caps) = math_pattern.captures(line) {
                let math_expression = caps.get(1).map(|m| m.as_str().to_string()).unwrap();
                let file_name = caps.get(2).map(|m| m.as_str().to_string()).unwrap();
                results.push((math_expression, file_name));
            }
        }

        results
    }
    
    pub fn get_style(&self, contents: &str) -> Option<String> {
        let style_pattern = Regex::new(BlocksRegExp::GET_STYLE_VAR).unwrap();
    
        if let Some(caps) = style_pattern.captures(contents) {
            caps.get(1).map(|m| m.as_str().to_string())
        } else {
            None
        }
    }
    
    pub fn get_qrcode(&self, contents: &str) -> Option<String> {
        let style_pattern = Regex::new(BlocksRegExp::GET_QRCODE_VAR).unwrap();

        if let Some(caps) = style_pattern.captures(contents) {
            caps.get(1).map(|m| m.as_str().to_string())
        } else {
            None
        }
    }

    pub fn get_server(&self, contents: &str) -> Option<String> {
        let server_pattern = Regex::new(BlocksRegExp::GET_SERVER_VAR).unwrap();

        if let Some(caps) = server_pattern.captures(contents) {
            caps.get(1).map(|m| m.as_str().to_string())
        } else {
            None
        }
    }

}
