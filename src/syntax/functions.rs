use regex::Regex;
use std::collections::HashMap;

use crate::regexp::regex_blocks::BlocksRegExp;

struct FnDef {
    params: Vec<String>,
    body: String,
}

pub struct Functions;

impl Functions {

    // Expands user-defined template functions: collects every
    // `@fn name(params) { ... }` definition, removes those blocks, and replaces
    // each `@name(args)` call with the function body, substituting `${param}`
    // with the matching argument. Runs before variable interpolation so a body
    // may also reference global `@var`s. Unknown calls are left untouched.
    pub fn expand(&self, contents: &str) -> String {
        let (defs, mut body) = self.collect(contents);

        if defs.is_empty() {
            return contents.to_string();
        }

        let call = Regex::new(BlocksRegExp::GET_FN_CALL).unwrap();

        // Functions can call functions, so expand repeatedly until stable.
        // The bound keeps a self-referential definition from looping forever.
        for _ in 0..10 {
            let next = call.replace_all(&body, |caps: &regex::Captures| {
                match defs.get(&caps[1]) {
                    Some(def) => self.apply(def, caps.get(2).map_or("", |m| m.as_str())),
                    None => caps[0].to_string(),
                }
            }).into_owned();

            if next == body {
                break;
            }

            body = next;
        }

        body
    }

    // Returns the parsed definitions and the source with the `@fn` blocks removed.
    fn collect(&self, contents: &str) -> (HashMap<String, FnDef>, String) {
        let header = Regex::new(BlocksRegExp::GET_FN_DEF).unwrap();

        let mut defs = HashMap::new();
        let mut stripped = String::new();
        let mut last = 0;

        for caps in header.captures_iter(contents) {
            let whole = caps.get(0).unwrap();
            let body_start = whole.end();

            // Match braces to find the end of the body.
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
                continue;
            }

            let name = caps[1].to_string();
            let params = caps[2].split(',')
                .map(|param| param.trim().to_string())
                .filter(|param| !param.is_empty())
                .collect();

            defs.insert(name, FnDef {
                params,
                body: contents[body_start..body_end].to_string(),
            });

            stripped.push_str(&contents[last..whole.start()]);
            last = close_end;
        }

        stripped.push_str(&contents[last..]);

        (defs, stripped)
    }

    fn apply(&self, def: &FnDef, args_raw: &str) -> String {
        let args = self.split_args(args_raw);

        // Normalize the body: drop the surrounding blank lines and per-line
        // indentation so it drops cleanly into the call site.
        let mut body = def.body
            .lines()
            .map(|line| line.trim())
            .collect::<Vec<_>>()
            .join("\n")
            .trim_matches('\n')
            .to_string();

        for (index, param) in def.params.iter().enumerate() {
            let value = args.get(index).cloned().unwrap_or_default();
            body = body.replace(&format!("${{{}}}", param), &value);
        }

        body
    }

    // Splits a call's argument list on commas, ignoring commas inside quotes and
    // stripping the surrounding double quotes from each argument.
    fn split_args(&self, raw: &str) -> Vec<String> {
        if raw.trim().is_empty() {
            return Vec::new();
        }

        let mut args = Vec::new();
        let mut current = String::new();
        let mut in_quote = false;

        for c in raw.chars() {
            match c {
                '"' => in_quote = !in_quote,
                ',' if !in_quote => {
                    args.push(current.trim().to_string());
                    current.clear();
                }
                _ => current.push(c),
            }
        }

        args.push(current.trim().to_string());
        args
    }

}
