use regex::Regex;

use crate::{
    syntax::ranges::Ranges,
    regexp::regex_blocks::BlocksRegExp,
};

pub struct Loops;

impl Loops {

    // Expands `for name in [...] { ... }` (or `for name in {A..B} { ... }`) by
    // repeating the body once per item, substituting `${name}` with each value.
    // A preprocessing pass like functions/includes, so the body may reference
    // global `@var`s and call `@fn`s (expanded afterwards).
    pub fn expand(&self, contents: &str) -> String {
        let mut text = contents.to_string();

        // One loop is expanded per pass; repeating handles nested and sequential
        // loops, and the bound stops a pathological input from looping forever.
        for _ in 0..50 {
            let (next, changed) = self.expand_once(&text);
            text = next;

            if !changed {
                break;
            }
        }

        text
    }

    fn expand_once(&self, contents: &str) -> (String, bool) {
        let header = Regex::new(BlocksRegExp::GET_FOR_LOOP).unwrap();

        let Some(caps) = header.captures(contents) else {
            return (contents.to_string(), false);
        };

        let whole = caps.get(0).unwrap();
        let start = whole.start();
        let body_start = whole.end();

        // Brace-match the body.
        let mut depth = 1;
        let mut body_end = body_start;
        let mut close_end = body_start;

        for (i, c) in contents[body_start..].char_indices() {
            match c {
                '{' => depth += 1,
                '}' => depth -= 1,
                _ => {}
            }

            if depth == 0 {
                body_end = body_start + i;
                close_end = body_start + i + c.len_utf8();
                break;
            }
        }

        if depth != 0 {
            return (contents.to_string(), false);
        }

        let var = caps.get(1).unwrap().as_str();
        let source = caps.get(2).unwrap().as_str();
        let body = &contents[body_start..body_end];

        let items = self.items(source);
        let expanded = items.iter()
            .map(|item| self.apply(body, var, item))
            .collect::<Vec<_>>()
            .join("\n");

        let mut result = String::new();
        result.push_str(&contents[..start]);
        result.push_str(&expanded);
        result.push_str(&contents[close_end..]);

        (result, true)
    }

    fn items(&self, source: &str) -> Vec<String> {
        if let Some(inner) = source.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            self.split_items(inner)
        } else {
            // `{A..B}` range form.
            Ranges.list(source)
        }
    }

    fn apply(&self, body: &str, var: &str, item: &str) -> String {
        let body = body.lines()
            .map(|line| line.trim())
            .collect::<Vec<_>>()
            .join("\n")
            .trim_matches('\n')
            .to_string();

        body.replace(&format!("${{{}}}", var), item)
    }

    // Splits a `[...]` list on commas, ignoring commas inside quotes and
    // stripping the surrounding double quotes from each item.
    fn split_items(&self, raw: &str) -> Vec<String> {
        if raw.trim().is_empty() {
            return Vec::new();
        }

        let mut items = Vec::new();
        let mut current = String::new();
        let mut in_quote = false;

        for c in raw.chars() {
            match c {
                '"' => in_quote = !in_quote,
                ',' if !in_quote => {
                    items.push(current.trim().to_string());
                    current.clear();
                }
                _ => current.push(c),
            }
        }

        items.push(current.trim().to_string());
        items.into_iter().filter(|item| !item.is_empty()).collect()
    }

}
