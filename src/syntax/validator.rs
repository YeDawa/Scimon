use regex::Regex;

use crate::regexp::regex_blocks::BlocksRegExp;

pub struct SyntaxError {
    pub line: usize,
    pub content: String,
    pub message: String,
}

pub struct Validator;

impl Validator {

    pub fn check(&self, contents: &str) -> Option<SyntaxError> {
        // Depth tracks block nesting (e.g. `group { ... }` inside `downloads`),
        // so only top-level directive lines are validated.
        let mut depth: i32 = 0;

        for (index, raw) in contents.lines().enumerate() {
            let trimmed = raw.trim();

            if depth > 0 {
                if trimmed.ends_with('{') {
                    depth += 1;
                } else if trimmed.starts_with('}') {
                    depth -= 1;
                }

                continue;
            }

            if trimmed.is_empty() {
                continue;
            }

            if let Some(message) = self.check_line(trimmed) {
                return Some(SyntaxError {
                    line: index + 1,
                    content: raw.to_string(),
                    message,
                });
            }

            if trimmed.ends_with('{') {
                depth += 1;
            }
        }

        None
    }

    fn keyword(line: &str) -> String {
        line.chars()
            .take_while(|c| c.is_ascii_alphabetic() || *c == '@')
            .collect::<String>()
            .to_lowercase()
    }

    fn check_line(&self, line: &str) -> Option<String> {
        let matches = |pattern: &str| {
            Regex::new(pattern).map(|re| re.is_match(line)).unwrap_or(true)
        };

        let kw = Self::keyword(line);

        if kw.starts_with('@') {
            if kw == "@var" {
                return if matches(BlocksRegExp::GET_VAR_DEF) {
                    None
                } else {
                    Some("expected: @var name \"value\"".to_string())
                };
            }

            if kw == "@fn" {
                return if matches(BlocksRegExp::GET_FN_DEF) {
                    None
                } else {
                    Some("expected: @fn name(params) { ... }".to_string())
                };
            }

            return if matches(r#"(?i)^@\w+\s+"[^"]*""#) {
                None
            } else {
                Some(format!("expected metadata in the form: {} \"value\"", kw))
            };
        }

        let var = |pattern: &str, name: &str| {
            if matches(pattern) { None } else { Some(format!("expected: {} \"value\"", name)) }
        };

        match kw.as_str() {
            "include"  => {
                if matches(BlocksRegExp::GET_INCLUDE_VAR) {
                    None
                } else {
                    Some("expected: include \"path.mon\"".to_string())
                }
            }

            "for" => {
                if matches(BlocksRegExp::GET_FOR_LOOP) {
                    None
                } else {
                    Some("expected: for name in [\"a\", \"b\"] { ... }".to_string())
                }
            }

            "path"     => var(BlocksRegExp::GET_PATH_VAR, "path"),
            "open"     => var(BlocksRegExp::GET_OPEN_VAR, "open"),
            "style"    => var(BlocksRegExp::GET_STYLE_VAR, "style"),
            "print"    => var(BlocksRegExp::GET_PRINT_VAR, "print"),
            "covers"   => var(BlocksRegExp::GET_COVERS_VAR, "covers"),
            "qrcode"   => var(BlocksRegExp::GET_QRCODE_VAR, "qrcode"),
            "copy"     => var(BlocksRegExp::GET_COPY_VAR, "copy"),
            "import"   => var(BlocksRegExp::GET_IMPORT_VAR, "import"),
            "compress" => var(BlocksRegExp::GET_COMPRESS_VAR, "compress"),
            "server"   => var(BlocksRegExp::GET_SERVER_VAR, "server"),

            "math" => {
                if matches(BlocksRegExp::GET_MATH_VAR) {
                    None
                } else {
                    Some("expected: math \"expression\" > path/to/output.png".to_string())
                }
            }

            "merge" => {
                if matches(BlocksRegExp::GET_MERGE_VAR) {
                    None
                } else {
                    Some("expected: merge \"pattern/*.pdf\" > \"output.pdf\"".to_string())
                }
            }

            "convert" => {
                if matches(BlocksRegExp::GET_CONVERT_VAR) {
                    None
                } else {
                    Some("expected: convert \"input.md\" > \"output.pdf\"".to_string())
                }
            }

            "split" => {
                if matches(BlocksRegExp::GET_SPLIT_VAR) {
                    None
                } else {
                    Some("expected: split \"input.pdf\" > \"output-{n}.pdf\"".to_string())
                }
            }

            "rotate" => {
                if matches(BlocksRegExp::GET_ROTATE_VAR) {
                    None
                } else {
                    Some("expected: rotate \"input.pdf\" 90 > \"output.pdf\"".to_string())
                }
            }

            "watermark" => {
                if matches(BlocksRegExp::GET_WATERMARK_VAR) {
                    None
                } else {
                    Some("expected: watermark \"input.pdf\" \"TEXT\" > \"output.pdf\" (or: ... image \"logo.png\" > ...)".to_string())
                }
            }

            "readme" => {
                if matches(BlocksRegExp::GET_README_VAR) || matches(r"(?i)^readme\s*\{") {
                    None
                } else {
                    Some("expected: readme \"url\" or readme { ... }".to_string())
                }
            }

            "downloads" => {
                if matches(r"(?i)^downloads\s*\{") {
                    None
                } else {
                    Some("expected: downloads { ... }".to_string())
                }
            }

            "commands" => {
                if matches(r"(?i)^commands\s*\{") {
                    None
                } else {
                    Some("expected: commands { ... }".to_string())
                }
            }

            "ai" => {
                if matches(BlocksRegExp::GET_AI_BLOCK) {
                    None
                } else {
                    Some("expected: ai { ... }".to_string())
                }
            }

            _ => None,
        }
    }

}
