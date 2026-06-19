use std::fs;
use regex::Regex;

use crate::regexp::regex_blocks::BlocksRegExp;

pub struct MonlibHandlers;

impl MonlibHandlers {

    pub fn validator_lib(&self, content: &str) -> bool {
        if content.is_empty() {
            return false;
        }
    
        BlocksRegExp::GET_PATTERNS_MONLIB_VARS.iter().any(|pattern| {
            let re = Regex::new(pattern).expect("Error compiling regex");
            re.is_match(content)
        })
    }

    pub fn validator_file(&self, run: &str) -> bool {
        let content = fs::read_to_string(run).unwrap_or_default();
        if content.is_empty() {
            return false;
        }
    
        BlocksRegExp::GET_PATTERNS_MONLIB_VARS.iter().any(|pattern| {
            let re = Regex::new(pattern).expect("Erro ao compilar a regex");
            re.is_match(&content)
        })
    }

}
