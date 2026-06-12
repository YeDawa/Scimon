use std::collections::{
    HashMap,
    HashSet,
};

use crate::render::latex::bibtex::{
    BibEntry,
    BibStyle,
};

/// Acronym registered by \newacronym, \DeclareAcronym or \acro
#[derive(Debug, Clone)]
pub struct AcronymInfo {
    pub short:        String,
    pub long:         String,
    /// explicit plural forms; defaults to short/long + "s"
    pub short_plural: Option<String>,
    pub long_plural:  Option<String>,
}

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
    /// Active citation/bibliography style (biblatex style option,
    /// \bibliographystyle, or numeric by default)
    pub bib_style:      BibStyle,
    /// true when \nocite{*} was seen — include every .bib entry at render time
    pub nocite_all:     bool,
    /// User-defined colors from \definecolor
    pub color_defs:     HashMap<String, String>,

    /// Acronyms keyed by label, plus first-use tracking for \ac and \gls
    pub acronyms:      HashMap<String, AcronymInfo>,
    pub acronyms_used: HashSet<String>,

    /// amsthm counters: counter name → (count, parent counter's last value)
    pub theorem_counters: HashMap<String, (usize, usize)>,

    pub footnote_num:      usize,
    pub pending_footnotes: Vec<(usize, String)>,
    /// \footnotemark numbers not yet claimed by a \footnotetext (FIFO)
    pub unresolved_marks:  Vec<usize>,
    /// label anchors already written — keeps ids unique when a float's
    /// anchor is hoisted to its top and \label also appears inside
    pub emitted_labels:    HashSet<String>,
    pub phantom_id:        usize,

    pub in_float: bool,
    /// "Table" or "Figure" while rendering inside the matching float,
    /// so captions can label themselves correctly
    pub float_kind: String,

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
            bib_style:      BibStyle::Numeric,
            nocite_all:     false,
            color_defs:     HashMap::new(),

            acronyms:      HashMap::new(),
            acronyms_used: HashSet::new(),

            theorem_counters: HashMap::new(),

            footnote_num:      0,
            pending_footnotes: Vec::new(),
            unresolved_marks:  Vec::new(),
            emitted_labels:    HashSet::new(),
            phantom_id:        0,

            in_float: false,
            float_kind: String::new(),

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

        // sort by mark number and pin each item's displayed number, so the
        // list matches the superscripts even when \footnotetext arrives out
        // of document order (e.g. after a float)
        let mut footnotes = std::mem::take(&mut self.pending_footnotes);
        footnotes.sort_by_key(|(num, _)| *num);
        for (num, content) in footnotes {
            html.push_str(&format!(
                "<li id=\"fn-{}\" value=\"{}\">{} <a href=\"#fnref-{}\" class=\"footnote-back\">↩</a></li>",
                num, num, content, num
            ));
        }

        html.push_str("</ol>");
        html
    }

}