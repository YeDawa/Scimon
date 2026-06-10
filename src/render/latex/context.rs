use std::collections::HashMap;
use crate::render::latex::bibtex::BibEntry;

pub struct RenderContext {
    pub doc_title:  String,
    pub doc_author: String,
    pub doc_date:   String,

    pub part_num:       usize,
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
    /// true when \nocite{*} was seen — include every .bib entry at render time
    pub nocite_all:     bool,
    /// User-defined colors from \definecolor
    pub color_defs:     HashMap<String, String>,

    pub footnote_num:      usize,
    pub pending_footnotes: Vec<(usize, String)>,
    pub phantom_id:        usize,

    pub in_float: bool,

    // fancyhdr — header/footer slots (rendered HTML strings)
    pub header_left:   String,
    pub header_center: String,
    pub header_right:  String,
    pub footer_left:   String,
    pub footer_center: String,
    pub footer_right:  String,
    pub has_fancy:     bool,
}

impl RenderContext {

    pub fn new(labels: HashMap<String, String>) -> Self {
        RenderContext {
            doc_title:  String::new(),
            doc_author: String::new(),
            doc_date:   String::new(),

            part_num:      0,
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
            nocite_all:     false,
            color_defs:     HashMap::new(),

            footnote_num:      0,
            pending_footnotes: Vec::new(),
            phantom_id:        0,

            in_float: false,

            header_left:   String::new(),
            header_center: String::new(),
            header_right:  String::new(),
            footer_left:   String::new(),
            footer_center: String::new(),
            footer_right:  String::new(),
            has_fancy:     false,
        }
    }

    /// Resolve a LaTeX color name to a CSS value.
    /// Checks user-defined colors first, then standard named colors.
    pub fn resolve_color(&self, name: &str) -> String {
        if let Some(css) = self.color_defs.get(name) {
            return css.clone();
        }
        match name {
            "black"   => "#000000",
            "white"   => "#ffffff",
            "red"     => "#ff0000",
            "green"   => "#00aa00",
            "blue"    => "#0000ff",
            "cyan"    => "#00cccc",
            "magenta" => "#cc00cc",
            "yellow"  => "#cccc00",
            "gray"    => "#808080",
            "grey"    => "#808080",
            "orange"  => "#ff8800",
            "violet"  => "#8800ff",
            "purple"  => "#880088",
            "brown"   => "#8b4513",
            "pink"    => "#ff69b4",
            "teal"    => "#008080",
            "lime"    => "#32cd32",
            "olive"   => "#808000",
            "navy"    => "#000080",
            _         => name,  // pass through (may already be a CSS value)
        }.to_string()
    }

    /// Return the current value of a named counter.
    pub fn counter_value(&self, name: &str) -> usize {
        match name {
            "section"      | "section*"      => self.sec_num,
            "subsection"   | "subsection*"   => self.subsec_num,
            "subsubsection"                  => self.subsubsec_num,
            "chapter"                        => self.chap_num,
            "equation"                       => self.eq_num,
            "figure"                         => self.fig_num,
            "table"                          => self.tab_num,
            "footnote"                       => self.footnote_num,
            _                                => 0,
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