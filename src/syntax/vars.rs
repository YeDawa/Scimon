extern crate open;

use regex::Regex;
use std::collections::HashMap;

use crate::{
    consts::addons::Addons,
    configs::settings::Settings,
    regexp::regex_blocks::BlocksRegExp,

    ui::ui_base::UI,
};

pub struct Vars;

impl Vars {

    // Resolves user-defined variables: collects every `@var name "value"`
    // declaration, drops those lines, and expands `${name}` everywhere else.
    // Unknown names are left untouched so a stray `${x}` stays visible.
    pub fn interpolate(&self, contents: &str) -> String {
        let def = Regex::new(BlocksRegExp::GET_VAR_DEF).unwrap();

        let mut map: HashMap<String, String> = HashMap::new();
        for caps in def.captures_iter(contents) {
            map.insert(caps[1].to_string(), caps[2].to_string());
        }

        if map.is_empty() {
            return contents.to_string();
        }

        let body = contents.lines()
            .filter(|line| !def.is_match(line))
            .collect::<Vec<_>>()
            .join("\n");

        let usage = Regex::new(BlocksRegExp::GET_VAR_USAGE).unwrap();

        usage.replace_all(&body, |caps: &regex::Captures| {
            map.get(&caps[1]).cloned().unwrap_or_else(|| caps[0].to_string())
        }).into_owned()
    }

    pub fn get_path(&self, contents: &str) -> String {
        let path_pattern = Regex::new(BlocksRegExp::GET_PATH_VAR).unwrap();

        // `path` is optional: when it is absent the current directory is used,
        // so lists that don't download anything don't have to declare it.
        path_pattern.captures(contents)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
            .unwrap_or_else(|| ".".to_string())
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

    pub fn get_metadata(&self, contents: &str, key: &str) -> Option<String> {
        let pattern = Regex::new(&format!(r#"(?im)^\s*@{}\s+"([^"]+)""#, key)).unwrap();

        pattern.captures(contents)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
    }

    pub fn get_imports(&self, contents: &str) -> Vec<String> {
        let import_pattern = Regex::new(BlocksRegExp::GET_IMPORT_VAR).unwrap();

        contents.lines()
            .filter_map(|line| {
                import_pattern.captures(line)
                    .and_then(|caps| caps.get(1))
                    .map(|m| m.as_str().to_string())
            })
            .collect()
    }

    pub fn get_copy(&self, contents: &str) -> Option<String> {
        let copy_pattern = Regex::new(BlocksRegExp::GET_COPY_VAR).unwrap();

        if let Some(caps) = copy_pattern.captures(contents) {
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
    
    pub fn get_all_merge(&self, contents: &str) -> Vec<(String, String)> {
        let merge_pattern = Regex::new(BlocksRegExp::GET_MERGE_VAR).unwrap();
        let mut results = Vec::new();

        for line in contents.lines() {
            if let Some(caps) = merge_pattern.captures(line) {
                let glob = caps.get(1).map(|m| m.as_str().to_string()).unwrap();
                let output = caps.get(2).map(|m| m.as_str().to_string()).unwrap();
                results.push((glob, output));
            }
        }

        results
    }

    pub fn get_all_convert(&self, contents: &str) -> Vec<(String, String)> {
        let convert_pattern = Regex::new(BlocksRegExp::GET_CONVERT_VAR).unwrap();
        let mut results = Vec::new();

        for line in contents.lines() {
            if let Some(caps) = convert_pattern.captures(line) {
                let input = caps.get(1).map(|m| m.as_str().to_string()).unwrap();
                let output = caps.get(2).map(|m| m.as_str().to_string()).unwrap();
                results.push((input, output));
            }
        }

        results
    }

    pub fn get_all_split(&self, contents: &str) -> Vec<(String, String)> {
        let split_pattern = Regex::new(BlocksRegExp::GET_SPLIT_VAR).unwrap();
        let mut results = Vec::new();

        for line in contents.lines() {
            if let Some(caps) = split_pattern.captures(line) {
                let input = caps.get(1).map(|m| m.as_str().to_string()).unwrap();
                let output = caps.get(2).map(|m| m.as_str().to_string()).unwrap();
                results.push((input, output));
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
