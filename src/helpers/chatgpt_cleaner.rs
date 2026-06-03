use regex::Regex;

use crate::consts::addons::Addons;

pub struct ChatGPTCleaner;

impl ChatGPTCleaner {

    pub fn strip_html_header(&self, text: &str) -> String {
        if let Some((_, content)) = text.split_once(Addons::CHATGPT_CONTENT_H4_CLASS) {
            return content.trim().to_string();
        }
        
        text.trim().to_string()
    }

    pub fn strip_reasoning_text(&self, text: &str) -> String {
        let re = Regex::new(Addons::CHATGPT_CONTENT_CLASS_ALT).unwrap();
        let clean_text = re.replace_all(text, "");
        clean_text.trim().to_string()
    }

}