use regex::Regex;

use crate::{
    utils::url::UrlMisc,
    ui::macros_alerts::MacrosAlerts,
    regexp::regex_macros::MacrosRegExp,
};

pub struct MacroHandler;

impl MacroHandler {

    pub fn remove_macros(input: &str) -> String {
        let re = Regex::new(MacrosRegExp::GET_MACROS).unwrap();
        re.replace_all(input, "").to_string()
    }

    pub fn handle_check_macro_line(line: &str, word: &str) -> bool {
        let get_macro = format!(
            "!{}", word.to_lowercase()
        );

        line.contains(&get_macro)
    }

    // True when any line in `content` carries the `!word` macro. Useful for
    // whole-block modes such as `!only`, where one tagged line changes how the
    // rest of the block is processed.
    pub fn any(content: &str, word: &str) -> bool {
        content.lines().any(|line| Self::handle_check_macro_line(line, word))
    }

    // Reads the argument of a parametrized macro, e.g. `!retry(3)` -> "3".
    pub fn macro_arg(line: &str, word: &str) -> Option<String> {
        let re = Regex::new(&format!(r"(?i)!{}\(([^)]*)\)", regex::escape(word))).unwrap();

        re.captures(line)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().trim().to_string())
    }

    // Number of extra attempts requested via `!retry(N)`; 0 when absent or invalid.
    pub fn retry_count(line: &str) -> u32 {
        Self::macro_arg(line, "retry")
            .and_then(|arg| arg.parse::<u32>().ok())
            .unwrap_or(0)
    }
  
    pub fn handle_ignore_macro_flag(line: &str, no_ignore: bool) -> Result<String, &'static str> {
        if !no_ignore && Self::handle_check_macro_line(line, "ignore") {
            MacrosAlerts::ignore(line);
            return Err("Line contains the '!ignore' directive.");
        }
    
        Ok(
            UrlMisc::extract_url(line)
        )
    }

}
