use regex::Regex;

use std::{
    fs,
    path::PathBuf,
};

use crate::{
    syntax::comments::Comments,
    regexp::regex_blocks::BlocksRegExp,
};

pub struct Includes;

impl Includes {

    // Splices local `.mon` files referenced with `include "path"` into the
    // source, so large lists can be broken into smaller files. Runs as a
    // preprocessing pass (before functions/variables), so an included file's
    // `@fn`/`@var` are shared with the rest of the document.
    pub fn expand(&self, contents: &str) -> String {
        self.expand_inner(contents, &mut Vec::new(), 0)
    }

    fn expand_inner(&self, contents: &str, stack: &mut Vec<PathBuf>, depth: usize) -> String {
        // Bail out on runaway nesting (e.g. an accidental cycle the canonical
        // check below somehow misses).
        if depth > 20 {
            return contents.to_string();
        }

        let pattern = Regex::new(BlocksRegExp::GET_INCLUDE_VAR).unwrap();
        let mut out: Vec<String> = Vec::new();

        for line in contents.lines() {
            let Some(caps) = pattern.captures(line) else {
                out.push(line.to_string());
                continue;
            };

            let path = caps.get(1).unwrap().as_str();
            let canonical = fs::canonicalize(path).ok();

            // Skip a file already on the include stack to avoid a cycle.
            if let Some(canonical) = &canonical {
                if stack.contains(canonical) {
                    continue;
                }
            }

            match fs::read_to_string(path) {
                Ok(raw) => {
                    let stripped = Comments.strip(&raw);

                    if let Some(canonical) = canonical {
                        stack.push(canonical);
                        out.push(self.expand_inner(&stripped, stack, depth + 1));
                        stack.pop();
                    } else {
                        out.push(self.expand_inner(&stripped, stack, depth + 1));
                    }
                }

                // A missing include is dropped rather than left as a dangling line.
                Err(_) => {}
            }
        }

        out.join("\n")
    }

}
