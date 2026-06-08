use std::collections::HashMap;

use crate::render::latex::bibtex::BibEntry;

pub struct RenderContext {
    pub bib_database: HashMap<String, BibEntry>,
    pub used_citations: Vec<String>,
    pub doc_title: String,
    pub doc_author: String,
    
    pub sec_num: u32,
    pub subsec_num: u32,
    pub eq_num: u32,
    pub fig_num: u32,
    pub tab_num: u32,
    
    pub last_counter: String,
    pub labels: HashMap<String, String>,
    pub toc: Vec<(u8, String, String)>,
}

impl RenderContext {

    pub fn new() -> Self {
        Self {
            bib_database: HashMap::new(),
            used_citations: Vec::new(),
            doc_title: String::from("Untitled Document"),
            doc_author: String::from("Unknown Author"),
            sec_num: 0, subsec_num: 0, eq_num: 0, fig_num: 0, tab_num: 0,
            last_counter: String::new(),
            labels: HashMap::new(),
            toc: Vec::new(),
        }
    }

    pub fn register_citation(&mut self, key: &str) -> usize {
        let key = key.trim().to_string();

        if let Some(pos) = self.used_citations.iter().position(|c| c == &key) {
            pos + 1
        } else {
            self.used_citations.push(key); 
            self.used_citations.len()
        }
    }

    pub fn reset_counters(&mut self) {
        self.sec_num = 0; 
        self.subsec_num = 0; 
        self.eq_num = 0; 
        self.fig_num = 0; 
        self.tab_num = 0;
    }

}