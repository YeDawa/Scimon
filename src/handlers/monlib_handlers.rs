use regex::Regex;

use crate::{
    configs::package::Package,
    regexp::regex_blocks::BlocksRegExp,
};

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
        Package.has_metadata(run)
    }

}
