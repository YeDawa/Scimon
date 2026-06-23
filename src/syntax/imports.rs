use regex::Regex;

use std::{
    env,
    fs,
    pin::Pin,
    path::Path,
    future::Future,
};

use crate::{
    syntax::comments::Comments,
    monlib::request::MonlibRequest,
    regexp::regex_blocks::BlocksRegExp,

    consts::{
        addons::Addons,
        folders::Folders,
    },
};

pub struct Imports;

impl Imports {

    pub async fn expand(&self, contents: &str) -> String {
        Self::expand_inner(contents.to_string(), Vec::new(), 0).await
    }

    fn expand_inner(contents: String, stack: Vec<String>, depth: usize) -> Pin<Box<dyn Future<Output = String>>> {
        Box::pin(async move {
            if depth > 20 {
                return contents;
            }

            let pattern = Regex::new(BlocksRegExp::GET_IMPORT_LINE).unwrap();
            let mut out: Vec<String> = Vec::new();

            for line in contents.lines() {
                let Some(caps) = pattern.captures(line) else {
                    out.push(line.to_string());
                    continue;
                };

                let source = caps.get(1).unwrap().as_str().to_string();
                let key = Self::cycle_key(&source);

                if stack.contains(&key) {
                    continue;
                }

                if let Some(raw) = Self::load(&source).await {
                    let stripped = Comments.strip(&raw);

                    let mut next = stack.clone();
                    next.push(key);

                    out.push(Self::expand_inner(stripped, next, depth + 1).await);
                }
            }

            out.join("\n")
        })
    }

    // Resolves a source to its `.mon` contents: a URL is fetched, an existing
    // path is read from disk, and anything else is treated as a Monlib package.
    async fn load(source: &str) -> Option<String> {
        if source.starts_with("http://") || source.starts_with("https://") {
            return reqwest::get(source).await.ok()?.text().await.ok();
        }

        if Path::new(source).is_file() {
            return fs::read_to_string(source).ok();
        }

        if !Self::has_api_key() {
            return None;
        }

        let url = format!("{}packages/{}/raw", Addons::MONLIB_API_REQUEST, source);
        let body = MonlibRequest::new().get(&url).await.ok()?.text().await.ok()?;

        // A JSON body is an API error (e.g. package not found), not a list.
        if body.trim_start().starts_with('{') {
            return None;
        }

        Some(body)
    }

    fn cycle_key(source: &str) -> String {
        if source.starts_with("http://") || source.starts_with("https://") {
            source.to_string()
        } else if Path::new(source).is_file() {
            fs::canonicalize(source)
                .map(|path| path.to_string_lossy().to_string())
                .unwrap_or_else(|_| source.to_string())
        } else {
            format!("monlib:{}", source)
        }
    }

    fn has_api_key() -> bool {
        dotenvy::from_path(Folders::APP_FOLDER.join(".env")).ok();

        env::var(Addons::MONLIB_API_ENV)
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false)
    }

}
