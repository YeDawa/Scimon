use serde_yaml::Value;

use std::{
    fs,
    sync::RwLock,
    path::Path,
};

use crate::consts::global::Global;

// Cache for the `author` read from the list's `package.yml`. It is loaded once
// when the list is read and reused by every EPUB generated during the run.
static AUTHOR: RwLock<Option<String>> = RwLock::new(None);

pub struct Package;

impl Package {

    // Loads `author` (or its `authors` alias) from the `package.yml` sitting next
    // to the list file, caching it for the rest of the run.
    pub fn load(&self, mon_path: &str) {
        if let Ok(mut guard) = AUTHOR.write() {
            *guard = Self::read_author(mon_path);
        }
    }

    // The author for generated EPUBs: from `package.yml` when present, otherwise
    // the application name.
    pub fn author(&self) -> String {
        AUTHOR.read()
            .ok()
            .and_then(|guard| guard.clone())
            .unwrap_or_else(|| Global::APP_NAME.to_string())
    }

    fn read_author(mon_path: &str) -> Option<String> {
        let dir = Path::new(mon_path).parent().unwrap_or_else(|| Path::new("."));
        let text = fs::read_to_string(dir.join("package.yml")).ok()?;
        let data: Value = serde_yaml::from_str(&text).ok()?;

        Self::read(&data["author"]).or_else(|| Self::read(&data["authors"]))
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
