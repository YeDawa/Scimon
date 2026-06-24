use serde_yaml::Value;

use std::{
    fs,
    path::Path,
    sync::RwLock,
};

use crate::consts::{
    global::Global,
    bundler::Bundler,
};

static AUTHOR: RwLock<Option<String>> = RwLock::new(None);

pub struct Package;

impl Package {

    pub fn load(&self, mon_path: &str) {
        if let Ok(mut guard) = AUTHOR.write() {
            *guard = Self::read_author(mon_path);
        }
    }

    pub fn has_metadata(&self, mon_path: &str) -> bool {
        let Some(data) = Self::document(mon_path) else {
            return false;
        };

        Bundler::MONLIB_PACKAGE_MANAGER_KEYS.iter().any(|key| Self::read(&data[*key]).is_some())
            || Self::read(&data["authors"]).is_some()
    }

    pub fn author(&self) -> String {
        AUTHOR.read()
            .ok()
            .and_then(|guard| guard.clone())
            .unwrap_or_else(|| Global::APP_NAME.to_string())
    }

    fn read_author(mon_path: &str) -> Option<String> {
        let data = Self::document(mon_path)?;
        Self::read(&data["author"]).or_else(|| Self::read(&data["authors"]))
    }

    // The package name from `package.yml`, used to name the distributable bundle.
    pub fn name(&self, mon_path: &str) -> Option<String> {
        Self::document(mon_path).and_then(|data| Self::read(&data["name"]))
    }

    // The optional package version from `package.yml`.
    pub fn version(&self, mon_path: &str) -> Option<String> {
        Self::document(mon_path).and_then(|data| Self::read(&data["version"]))
    }

    fn document(mon_path: &str) -> Option<Value> {
        let dir = Path::new(mon_path).parent().unwrap_or_else(|| Path::new("."));
        let text = fs::read_to_string(dir.join("package.yml")).ok()?;
        serde_yaml::from_str(&text).ok()
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
