use regex::Regex;

use crate::regexp::regex_blocks::BlocksRegExp;

pub struct Conditionals;

impl Conditionals {

    pub fn expand(&self, contents: &str) -> String {
        let mut text = contents.to_string();

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
        let header = Regex::new(BlocksRegExp::GET_IF_HEADER).unwrap();

        let Some(caps) = header.captures(contents) else {
            return (contents.to_string(), false);
        };

        let whole = caps.get(0).unwrap();
        let start = whole.start();
        let body_start = whole.end();

        // Brace-match the if body.
        let mut depth = 1;
        let mut if_body_end = body_start;
        let mut if_close_end = body_start;

        for (i, c) in contents[body_start..].char_indices() {
            match c {
                '{' => depth += 1,
                '}' => depth -= 1,
                _ => {}
            }

            if depth == 0 {
                if_body_end = body_start + i;
                if_close_end = body_start + i + c.len_utf8();
                break;
            }
        }

        if depth != 0 {
            return (contents.to_string(), false);
        }

        let remaining = &contents[if_close_end..];
        let mut else_body_start = None;
        let mut else_close_end = None;
        let mut else_body_end = None;

        let else_regex = Regex::new(r"^[ \t]*else\s*\{").unwrap();
        if let Some(else_match) = else_regex.find(remaining) {
            let else_header_len = else_match.end();
            let start_of_else_body = if_close_end + else_header_len;
            
            let mut else_depth = 1;
            let mut end_of_else_body = start_of_else_body;
            let mut end_of_else_close = start_of_else_body;

            for (i, c) in contents[start_of_else_body..].char_indices() {
                match c {
                    '{' => else_depth += 1,
                    '}' => else_depth -= 1,
                    _ => {}
                }

                if else_depth == 0 {
                    end_of_else_body = start_of_else_body + i;
                    end_of_else_close = start_of_else_body + i + c.len_utf8();
                    break;
                }
            }

            if else_depth == 0 {
                else_body_start = Some(start_of_else_body);
                else_body_end = Some(end_of_else_body);
                else_close_end = Some(end_of_else_close);
            }
        }

        let operand1 = caps.get(1).unwrap().as_str().trim().trim_matches('"').trim_matches('\'');
        let operator = caps.get(2).unwrap().as_str().trim();
        let operand2 = caps.get(3).unwrap().as_str().trim().trim_matches('"').trim_matches('\'');

        let is_true = match operator {
            "==" => operand1 == operand2,
            "!=" => operand1 != operand2,
            _ => false,
        };

        let mut result = String::new();
        result.push_str(&contents[..start]);

        if is_true {
            let if_body = &contents[body_start..if_body_end];
            result.push_str(if_body);
        } else if let Some(else_start) = else_body_start {
            let else_body = &contents[else_start..else_body_end.unwrap()];
            result.push_str(else_body);
        }

        let end_of_block = else_close_end.unwrap_or(if_close_end);
        result.push_str(&contents[end_of_block..]);

        (result, true)
    }

}
