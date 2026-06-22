use regex::Regex;
use serde_json::{json, Value};

use crate::{
    syntax::vars::Vars,
    syntax::extended::Extended,
    syntax::comments::Comments,
    syntax::macro_handler::MacroHandler,
    regexp::regex_blocks::BlocksRegExp,
};

pub struct Parser;

impl Parser {

    pub fn to_json(&self, contents: &str) -> String {
        serde_json::to_string_pretty(&self.parse(contents))
            .unwrap_or_else(|_| "{}".to_string())
    }

    pub fn parse(&self, raw: &str) -> Value {
        // The loaded server source is already comment-stripped, but strip again
        // so the parser is correct for any input handed to it directly.
        let contents = Comments.strip(raw);

        let ast = json!({
            "metadata": self.metadata(&contents),
            "directives": self.directives(&contents),
            "blocks": self.blocks(&contents),
        });

        // Absent directives and empty blocks are modelled as null/empty while
        // building the AST; prune them here so the output only carries what the
        // script actually declares.
        Self::prune(ast)
    }

    fn prune(value: Value) -> Value {
        match value {
            Value::Object(map) => Value::Object(
                map.into_iter()
                    .map(|(k, v)| (k, Self::prune(v)))
                    .filter(|(_, v)| !Self::is_empty(v))
                    .collect()
            ),

            Value::Array(items) => Value::Array(
                items.into_iter()
                    .map(Self::prune)
                    .filter(|v| !Self::is_empty(v))
                    .collect()
            ),

            other => other,
        }
    }

    fn is_empty(value: &Value) -> bool {
        match value {
            Value::Null => true,
            Value::String(s) => s.is_empty(),
            Value::Array(items) => items.is_empty(),
            Value::Object(map) => map.is_empty(),
            _ => false,
        }
    }

    fn metadata(&self, contents: &str) -> Value {
        let keys = ["name", "version", "description", "author", "license", "privacy", "homepage"];
        let mut map = serde_json::Map::new();

        for key in keys {
            let value = match Vars.get_metadata(contents, key) {
                Some(v) => Value::String(v),
                None => Value::Null,
            };

            map.insert(key.to_string(), value);
        }

        Value::Object(map)
    }

    fn directives(&self, contents: &str) -> Value {
        let math: Vec<Value> = Vars.get_all_math(contents)
            .into_iter()
            .map(|(expression, output)| json!({ "expression": expression, "output": output }))
            .collect();

        let merge: Vec<Value> = Vars.get_all_merge(contents)
            .into_iter()
            .map(|(pattern, output)| json!({ "pattern": pattern, "output": output }))
            .collect();

        let convert: Vec<Value> = Vars.get_all_convert(contents)
            .into_iter()
            .map(|(input, output)| json!({ "input": input, "output": output }))
            .collect();

        json!({
            "path": self.capture(contents, BlocksRegExp::GET_PATH_VAR),
            "open": self.capture(contents, BlocksRegExp::GET_OPEN_VAR),
            "compress": self.capture(contents, BlocksRegExp::GET_COMPRESS_VAR),
            "copy": self.capture(contents, BlocksRegExp::GET_COPY_VAR),
            "covers": self.capture(contents, BlocksRegExp::GET_COVERS_VAR),
            "qrcode": self.capture(contents, BlocksRegExp::GET_QRCODE_VAR),
            "style": self.capture(contents, BlocksRegExp::GET_STYLE_VAR),
            "print": self.capture(contents, BlocksRegExp::GET_PRINT_VAR),
            "server": self.capture(contents, BlocksRegExp::GET_SERVER_VAR),
            "readme": self.capture(contents, BlocksRegExp::GET_README_VAR),
            "imports": Vars.get_imports(contents),
            "math": math,
            "merge": merge,
            "convert": convert,
        })
    }

    fn blocks(&self, contents: &str) -> Value {
        json!({
            "downloads": self.downloads(contents),
            "commands": self.commands(contents),
            "ai": self.ai(contents),
            "readme": self.readme(contents),
        })
    }

    fn downloads(&self, contents: &str) -> Vec<Value> {
        let Some(block) = self.extract_block(contents, "downloads") else {
            return Vec::new();
        };

        block.lines()
            .filter_map(|line| {
                let trimmed = line.trim();

                // A line may list `||`-separated fallback URLs; the first is the
                // primary, the rest are mirrors tried in order on failure.
                let candidates: Vec<&str> = trimmed
                    .split("||")
                    .filter_map(|segment| segment.trim().split_whitespace().next())
                    .collect();

                let url = *candidates.first()?;

                if !url.starts_with("http") {
                    return None;
                }

                let retries = MacroHandler::retry_count(trimmed);
                let fallbacks = &candidates[1..];

                Some(json!({
                    "url": url,
                    "fallbacks": if fallbacks.is_empty() { Value::Null } else { json!(fallbacks) },
                    "name": Extended.rename_on_the_fly(trimmed),
                    "ignore": MacroHandler::handle_check_macro_line(trimmed, "ignore"),
                    "retry": if retries > 0 { json!(retries) } else { Value::Null },
                }))
            })
            .collect()
    }

    fn commands(&self, contents: &str) -> Vec<Value> {
        let Some(block) = self.extract_block(contents, "commands") else {
            return Vec::new();
        };

        block.lines()
            .filter_map(|line| {
                let trimmed = line.trim();
                let url = trimmed.split_whitespace().next().unwrap_or("");

                if !url.starts_with("http") {
                    return None;
                }

                Some(json!({
                    "url": url,
                    "ignore": MacroHandler::handle_check_macro_line(trimmed, "ignore"),
                }))
            })
            .collect()
    }

    fn ai(&self, contents: &str) -> Vec<Value> {
        let Some(block) = self.extract_block(contents, "ai") else {
            return Vec::new();
        };

        let entry_pattern = Regex::new(BlocksRegExp::GET_AI_ENTRY).unwrap();

        block.lines()
            .filter_map(|line| {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    return None;
                }

                let caps = entry_pattern.captures(trimmed)?;

                let prompt = caps.name("prompt")?
                    .as_str()
                    .replace("\\\"", "\"")
                    .trim()
                    .to_string();

                let file = caps.name("file")?.as_str().trim().to_string();
                let model = caps.name("model").map(|m| m.as_str().trim().to_string());

                Some(json!({
                    "prompt": prompt,
                    "file": file,
                    "model": model,
                    "ignore": MacroHandler::handle_check_macro_line(trimmed, "ignore"),
                }))
            })
            .collect()
    }

    fn readme(&self, contents: &str) -> Value {
        match self.extract_block(contents, "readme") {
            Some(block) => Value::String(block.trim().to_string()),
            None => Value::Null,
        }
    }

    // Captures the first quoted argument of a single-line directive (e.g. `path`,
    // `open`, `server`), returning JSON null when the directive is absent.
    fn capture(&self, contents: &str, pattern: &str) -> Value {
        Regex::new(pattern).unwrap()
            .captures(contents)
            .and_then(|caps| caps.get(1))
            .map(|m| Value::String(m.as_str().to_string()))
            .unwrap_or(Value::Null)
    }

    // Returns the inner content of a `keyword { ... }` block using brace matching
    // so nested braces don't truncate the block early.
    fn extract_block(&self, contents: &str, keyword: &str) -> Option<String> {
        let pattern = Regex::new(&format!(r"(?im)^\s*{}\s*\{{", keyword)).unwrap();
        let start = pattern.find(contents)?.end();

        let mut depth = 1;
        for (i, c) in contents[start..].char_indices() {
            match c {
                '{' => depth += 1,
                '}' => depth -= 1,
                _ => {}
            }

            if depth == 0 {
                return Some(contents[start..start + i].to_string());
            }
        }

        None
    }

}
