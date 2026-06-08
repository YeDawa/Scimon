use std::collections::HashMap;
use crate::render::latex::bibtex::BibEntry;

pub struct RenderContext {
    pub doc_title:  String,
    pub doc_author: String,
    pub doc_date:   String,

    pub chap_num:       usize,
    pub sec_num:        usize,
    pub subsec_num:     usize,
    pub subsubsec_num:  usize,
    pub eq_num:         usize,
    pub fig_num:        usize,
    pub tab_num:        usize,
    pub last_counter:   String,

    pub toc:    Vec<(usize, String, String)>,
    pub labels: HashMap<String, String>,

    pub citation_order: Vec<String>,
    pub citation_map:   HashMap<String, usize>,
    pub bib_database:   HashMap<String, BibEntry>,

    pub footnote_num:      usize,
    pub pending_footnotes: Vec<(usize, String)>,

    pub in_float: bool,
}

impl RenderContext {

    pub fn new(labels: HashMap<String, String>) -> Self {
        RenderContext {
            doc_title:  String::new(),
            doc_author: String::new(),
            doc_date:   String::new(),

            chap_num:      0,
            sec_num:       0,
            subsec_num:    0,
            subsubsec_num: 0,
            eq_num:        0,
            fig_num:       0,
            tab_num:       0,
            last_counter:  String::from("0"),

            toc:    Vec::new(),
            labels,

            citation_order: Vec::new(),
            citation_map:   HashMap::new(),
            bib_database:   HashMap::new(),

            footnote_num:      0,
            pending_footnotes: Vec::new(),

            in_float: false,
        }
    }

    pub fn register_citation(&mut self, key: &str) -> usize {
        if let Some(&n) = self.citation_map.get(key) {
            return n;
        }

        let n = self.citation_order.len() + 1;
        self.citation_order.push(key.to_string());
        self.citation_map.insert(key.to_string(), n);

        n
    }

    pub fn flush_footnotes(&mut self) -> String {
        if self.pending_footnotes.is_empty() {
            return String::new();
        }

        let mut html = String::from(
            "<hr class=\"footnote-rule\"/><ol class=\"footnote-list\">"
        );

        let footnotes = std::mem::take(&mut self.pending_footnotes);
        for (num, content) in footnotes {
            html.push_str(&format!(
                "<li id=\"fn-{}\">{} <a href=\"#fnref-{}\" class=\"footnote-back\">↩</a></li>",
                num, content, num
            ));
        }
        
        html.push_str("</ol>");
        html
    }

}