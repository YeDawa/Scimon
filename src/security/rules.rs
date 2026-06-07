use regex::Regex;

pub struct Rules {
    pub name: &'static str,
    pub description: &'static str,
    pub pattern: Regex,
}