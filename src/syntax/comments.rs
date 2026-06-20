use regex::Regex;

use crate::regexp::regex_blocks::BlocksRegExp;

pub struct Comments;

impl Comments {

    pub fn strip(&self, contents: &str) -> String {
        let readme_open = Regex::new(BlocksRegExp::GET_README_BLOCK[0]).unwrap();

        let mut out: Vec<String> = Vec::new();
        let mut in_comment = false;
        let mut readme_depth: i32 = 0;

        for line in contents.lines() {
            if readme_depth > 0 {
                out.push(line.to_string());
                readme_depth += Self::brace_delta(line);
                continue;
            }

            let (clean, still_in_comment) = self.strip_line(line, in_comment);
            in_comment = still_in_comment;

            if !in_comment && readme_open.is_match(&clean) {
                readme_depth = Self::brace_delta(&clean);
            }

            out.push(clean);
        }

        out.join("\n")
    }

    fn brace_delta(line: &str) -> i32 {
        let mut delta = 0;

        for c in line.chars() {
            match c {
                '{' => delta += 1,
                '}' => delta -= 1,
                _ => {}
            }
        }

        delta
    }

    fn strip_line(&self, line: &str, mut in_comment: bool) -> (String, bool) {
        let chars: Vec<char> = line.chars().collect();
        let mut out = String::new();
        let mut in_string = false;
        let mut prev_ws = true;
        let mut i = 0;

        while i < chars.len() {
            let c = chars[i];

            if in_comment {
                if c == '*' && chars.get(i + 1) == Some(&'/') {
                    in_comment = false;
                    prev_ws = true;
                    i += 2;
                } else {
                    i += 1;
                }

                continue;
            }

            if in_string {
                out.push(c);
                if c == '"' {
                    in_string = false;
                }

                prev_ws = false;
                i += 1;
                continue;
            }

            if prev_ws && c == '/' && chars.get(i + 1) == Some(&'*') {
                in_comment = true;
                i += 2;
                continue;
            }

            if prev_ws && c == '/' && chars.get(i + 1) == Some(&'/') {
                break;
            }

            if c == '"' {
                in_string = true;
            }

            out.push(c);
            prev_ws = c.is_whitespace();
            i += 1;
        }

        (out.trim_end().to_string(), in_comment)
    }

}
