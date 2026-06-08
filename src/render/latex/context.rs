// context.rs  –  render state threaded through the HTML pass
//
// All counters and accumulated state that LatexNode::to_html() needs.

use std::collections::HashMap;
use crate::render::latex::bibtex::BibEntry;

pub struct RenderContext {
    // -----------------------------------------------------------------------
    // Document metadata (set by \title, \author, \date)
    // -----------------------------------------------------------------------
    pub doc_title:  String,
    pub doc_author: String,
    pub doc_date:   String,

    // -----------------------------------------------------------------------
    // Structural counters
    // -----------------------------------------------------------------------
    pub chap_num:       usize,
    pub sec_num:        usize,
    pub subsec_num:     usize,
    pub subsubsec_num:  usize,
    pub eq_num:         usize,
    pub fig_num:        usize,
    pub tab_num:        usize,

    /// Holds the last figure/table counter string so \caption can refer to it.
    pub last_counter: String,

    // -----------------------------------------------------------------------
    // Table of contents
    //   (heading_level, number_string, title_text)
    // -----------------------------------------------------------------------
    pub toc: Vec<(usize, String, String)>,

    // -----------------------------------------------------------------------
    // Labels  label_name → number_string
    // -----------------------------------------------------------------------
    pub labels: HashMap<String, String>,

    // -----------------------------------------------------------------------
    // Citations
    //   citation_order : keys in the order they were first cited
    //   citation_map   : key → citation number (1-based)
    // -----------------------------------------------------------------------
    pub citation_order: Vec<String>,
    pub citation_map:   HashMap<String, usize>,

    // -----------------------------------------------------------------------
    // Bibliography database (populated when \bibliography is rendered)
    // -----------------------------------------------------------------------
    pub bib_database: HashMap<String, BibEntry>,

    // -----------------------------------------------------------------------
    // Footnotes
    //   footnote_num      : running counter
    //   pending_footnotes : (number, rendered_html) collected during the pass;
    //                       the caller flushes them at the end of the document.
    // -----------------------------------------------------------------------
    pub footnote_num:      usize,
    pub pending_footnotes: Vec<(usize, String)>,
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
        }
    }

    // -----------------------------------------------------------------------
    // Citation helpers
    // -----------------------------------------------------------------------

    /// Register a citation key and return its 1-based number.
    pub fn register_citation(&mut self, key: &str) -> usize {
        if let Some(&n) = self.citation_map.get(key) {
            return n;
        }
        let n = self.citation_order.len() + 1;
        self.citation_order.push(key.to_string());
        self.citation_map.insert(key.to_string(), n);
        n
    }

    // -----------------------------------------------------------------------
    // Footnote flush
    //
    // Call this once at the end of the document render to emit all collected
    // footnotes as an <ol> block.
    // -----------------------------------------------------------------------
    pub fn flush_footnotes(&mut self) -> String {
        if self.pending_footnotes.is_empty() {
            return String::new();
        }
        let mut html = String::from(
            "<hr class=\"footnote-rule\"/><ol class=\"footnote-list\">"
        );
        // Drain in insertion order
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