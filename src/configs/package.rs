use regex::Regex;
use serde_yaml::Value;

use std::{
    fs,
    path::Path,
};

pub struct Package;

impl Package {

    const KEYS: [&'static str; 6] = [
        "name", "description", "author", "license", "privacy", "homepage",
    ];

    pub fn strip_inline(&self, contents: &str) -> String {
        let pattern = Regex::new(
            r"(?i)^\s*@(?:name|description|author|license|privacy|homepage)\b"
        ).unwrap();

        contents.lines()
            .filter(|line| !pattern.is_match(line))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn metadata_block(&self, mon_path: &str) -> String {
        let dir = Path::new(mon_path).parent().unwrap_or_else(|| Path::new("."));
        let package_path = dir.join("package.yml");

        let Ok(text) = fs::read_to_string(&package_path) else {
            return String::new();
        };

        let Ok(data) = serde_yaml::from_str::<Value>(&text) else {
            return String::new();
        };

        let mut block = String::new();

        for key in Self::KEYS {
            let value = if key == "author" {
                Self::read(&data["author"]).or_else(|| Self::read(&data["authors"]))
            } else {
                Self::read(&data[key])
            };

            if let Some(value) = value {
                block.push_str(&format!("@{} \"{}\"\n", key, value.replace('"', "'")));
            }
        }

        block
    }

    fn read(value: &Value) -> Option<String> {
        if let Some(text) = value.as_str() {
            return Some(text.to_string());
        }

        if let Some(items) = value.as_sequence() {
            let joined: Vec<String> = items.iter()
                .filter_map(|item| item.as_str().map(|s| s.to_string()))
                .collect();

            if !joined.is_empty() {
                return Some(joined.join(", "));
            }
        }

        None
    }

}
