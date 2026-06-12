use std::{
    fs::read_to_string,
    collections::HashMap,
};

// ---------------------------------------------------------------------------
// Built-in bibliography styles
// ---------------------------------------------------------------------------
// Citation/bibliography style selected via \usepackage[style=...]{biblatex}
// or \bibliographystyle{...}. Unknown styles fall back to Numeric.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BibStyle {
    // [1] citations, reference list in citation order
    Numeric,
    // (Silva & Santos, 2020) citations, alphabetical reference list (APA-like)
    AuthorYear,
    // [Sil20] citations, alphabetical reference list
    Alphabetic,
    // (SILVA; SANTOS, 2020) citations, ABNT NBR 6023-like reference list
    Abnt,
}

impl BibStyle {

    // Map a biblatex style option or a \bibliographystyle name to a
    // built-in style. Returns None for unrecognized names.
    pub fn from_name(name: &str) -> Option<Self> {
        let name = name.trim().to_lowercase();
        match name.as_str() {
            "numeric" | "numeric-comp" | "numeric-verb" | "ieee" | "nature"
            | "science" | "vancouver" | "plain" | "abbrv" | "unsrt"
                => Some(BibStyle::Numeric),

            "authoryear" | "authoryear-comp" | "authoryear-ibid" | "apa"
            | "apa6" | "apalike" | "harvard" | "chicago-authordate"
                => Some(BibStyle::AuthorYear),

            "alphabetic" | "alphabetic-verb" | "alpha"
                => Some(BibStyle::Alphabetic),

            _ if name.starts_with("abnt") => Some(BibStyle::Abnt),
            _ => None,
        }
    }

}

// ---------------------------------------------------------------------------
// Bibliography entry
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Default)]
pub struct BibEntry {
    // All fields, keyed by lowercase field name, values cleaned for HTML
    pub fields: HashMap<String, String>,
}

impl BibEntry {

    pub fn field(&self, name: &str) -> &str {
        self.fields.get(name).map(|s| s.as_str()).unwrap_or("")
    }

    pub fn year(&self) -> String {
        let year = self.field("year");
        if !year.is_empty() {
            return year.to_string();
        }
        // biblatex date = {2020-01-15} — take the year part
        self.field("date").chars().take(4).collect()
    }

    // Authors as (surname, given-names) pairs. Falls back to editor,
    // then organization, when no author field exists.
    pub fn authors(&self) -> Vec<(String, String)> {
        let raw = if !self.field("author").is_empty() {
            self.field("author")
        } else if !self.field("editor").is_empty() {
            self.field("editor")
        } else {
            self.field("organization")
        };

        raw.split(" and ")
            .map(|name| Self::split_name(name.trim()))
            .filter(|(surname, _)| !surname.is_empty())
            .collect()
    }

    // "Silva, Maria" -> ("Silva", "Maria"); "Donald Knuth" -> ("Knuth", "Donald")
    fn split_name(name: &str) -> (String, String) {
        if let Some((surname, given)) = name.split_once(',') {
            (surname.trim().to_string(), given.trim().to_string())
        } else {
            match name.rsplit_once(' ') {
                Some((given, surname)) => (surname.trim().to_string(), given.trim().to_string()),
                None => (name.to_string(), String::new()),
            }
        }
    }

    fn initials(given: &str) -> String {
        given.split_whitespace()
            .filter_map(|part| part.chars().next())
            .map(|c| format!("{}.", c))
            .collect::<Vec<_>>()
            .join(" ")
    }

    // In-text author label: "Silva", "Silva & Santos", "Silva et al."
    // `textual` selects the \textcite form (spelled-out connective);
    // ABNT parenthetical citations use uppercase surnames.
    pub fn cite_authors(&self, style: BibStyle, textual: bool) -> String {
        let authors = self.authors();
        if authors.is_empty() {
            return self.field("key").to_string();
        }

        let case = |surname: &str| -> String {
            if style == BibStyle::Abnt && !textual {
                surname.to_uppercase()
            } else {
                surname.to_string()
            }
        };

        let connective = match (style, textual) {
            (BibStyle::Abnt, false) => "; ",
            (BibStyle::Abnt, true)  => " e ",
            (_, true)               => " and ",
            (_, false)              => " & ",
        };

        match authors.len() {
            1 => case(&authors[0].0),
            2 => format!("{}{}{}", case(&authors[0].0), connective, case(&authors[1].0)),
            _ => format!("{} et al.", case(&authors[0].0)),
        }
    }

    // Alphabetic-style label: "Sil20" (one author) or "SK20" (multiple)
    pub fn alpha_label(&self) -> String {
        let authors = self.authors();
        let year = self.year();
        let yy = if year.len() >= 2 { &year[year.len() - 2..] } else { "" };

        let prefix = match authors.len() {
            0 => "???".to_string(),
            1 => authors[0].0.chars().take(3).collect(),
            _ => authors.iter()
                .take(4)
                .filter_map(|(surname, _)| surname.chars().next())
                .collect(),
        };

        format!("{}{}", prefix, yy)
    }

    // Key used to sort alphabetical reference lists
    pub fn sort_key(&self) -> String {
        let authors = self.authors()
            .iter()
            .map(|(surname, _)| surname.to_uppercase())
            .collect::<Vec<_>>()
            .join(" ");
        format!("{} {}", authors, self.year())
    }

    fn author_list(&self, style: BibStyle) -> String {
        let authors = self.authors();
        let formatted: Vec<String> = authors.iter()
            .map(|(surname, given)| {
                let initials = Self::initials(given);
                let surname = if style == BibStyle::Abnt {
                    surname.to_uppercase()
                } else {
                    surname.clone()
                };
                if initials.is_empty() {
                    surname
                } else {
                    format!("{}, {}", surname, initials)
                }
            })
            .collect();

        match style {
            BibStyle::Abnt => formatted.join("; "),
            BibStyle::AuthorYear => match formatted.len() {
                0 | 1 => formatted.join(""),
                2 => format!("{}, & {}", formatted[0], formatted[1]),
                _ => {
                    let (last, init) = formatted.split_last().unwrap();
                    format!("{}, & {}", init.join(", "), last)
                }
            },
            _ => formatted.join(", "),
        }
    }

    // Where the work appeared: journal, "In: proceedings", or publisher
    fn venue(&self) -> String {
        let journal = if !self.field("journal").is_empty() {
            self.field("journal")
        } else {
            self.field("journaltitle")
        };
        if !journal.is_empty() {
            return format!("<em>{}</em>", journal);
        }

        let booktitle = self.field("booktitle");
        if !booktitle.is_empty() {
            return format!("In: <em>{}</em>", booktitle);
        }

        let mut parts = Vec::new();
        let address = if !self.field("address").is_empty() {
            self.field("address")
        } else {
            self.field("location")
        };
        if !address.is_empty() { parts.push(address.to_string()); }
        if !self.field("publisher").is_empty() { parts.push(self.field("publisher").to_string()); }
        parts.join(": ")
    }

    fn volume_pages(&self, style: BibStyle) -> String {
        let mut parts = Vec::new();
        let (volume, number, pages) = (self.field("volume"), self.field("number"), self.field("pages"));

        if style == BibStyle::Abnt {
            if !volume.is_empty() { parts.push(format!("v. {}", volume)); }
            if !number.is_empty() { parts.push(format!("n. {}", number)); }
            if !pages.is_empty()  { parts.push(format!("p. {}", pages)); }
        } else {
            if !volume.is_empty() {
                if number.is_empty() {
                    parts.push(volume.to_string());
                } else {
                    parts.push(format!("{}({})", volume, number));
                }
            }
            if !pages.is_empty() { parts.push(format!("pp. {}", pages)); }
        }

        parts.join(", ")
    }

    fn link(&self) -> String {
        let doi = self.field("doi");
        if !doi.is_empty() {
            return format!(" <a href=\"https://doi.org/{}\">doi:{}</a>", doi, doi);
        }
        let url = self.field("url");
        if !url.is_empty() {
            return format!(" <a href=\"{}\">{}</a>", url, url);
        }
        String::new()
    }

    // Full reference-list entry, formatted according to the active style.
    pub fn full_reference(&self, style: BibStyle) -> String {
        let title = self.field("title");
        let is_container = !self.field("journal").is_empty()
            || !self.field("journaltitle").is_empty()
            || !self.field("booktitle").is_empty();

        let title_html = match style {
            BibStyle::Abnt => format!("<strong>{}</strong>", title),
            _ if is_container => title.to_string(),
            _ => format!("<em>{}</em>", title),
        };

        let mut tail = Vec::new();
        let venue = self.venue();
        if !venue.is_empty() { tail.push(venue); }
        let vp = self.volume_pages(style);
        if !vp.is_empty() { tail.push(vp); }

        // author lists ending in an initial already carry a period
        let close = |text: String| -> String {
            if text.ends_with('.') { text } else { format!("{}.", text) }
        };

        match style {
            BibStyle::AuthorYear => {
                // Silva, M., & Santos, J. (2020). Title. Journal, 10(2), pp. 1-9.
                let mut out = format!("{} ({}). {}.", self.author_list(style), self.year(), title_html);
                if !tail.is_empty() {
                    out.push_str(&format!(" {}.", tail.join(", ")));
                }
                out.push_str(&self.link());
                out
            }
            BibStyle::Abnt => {
                // SILVA, M.; SANTOS, J. Title. Journal, v. 10, n. 2, p. 1-9, 2020.
                tail.push(self.year());
                format!("{} {}. {}.{}", close(self.author_list(style)), title_html, tail.join(", "), self.link())
            }
            _ => {
                // Silva, M., Santos, J. Title. Journal, 10(2), pp. 1-9, 2020.
                tail.push(self.year());
                format!("{} {}. {}.{}", close(self.author_list(style)), title_html, tail.join(", "), self.link())
            }
        }
    }

}

// ---------------------------------------------------------------------------
// BibTeX file parsing
// ---------------------------------------------------------------------------
pub struct BibTextRender;

impl BibTextRender {

    pub fn fetch_bibliography(source: &str) -> Result<String, String> {
        if source.starts_with("http://") || source.starts_with("https://") {
            ureq::get(source)
                .call()
                .map_err(|e| e.to_string())?
                .body_mut()
                .read_to_string()
                .map_err(|e| e.to_string())
        } else {
            read_to_string(source).map_err(|e| e.to_string())
        }
    }

    // Resolve a \bibliography/\addbibresource argument (file stem, .bib path
    // or URL), fetch it and parse it. Returns an empty map on failure.
    pub fn load(source: &str) -> HashMap<String, BibEntry> {
        let path = if source.starts_with("http://")
            || source.starts_with("https://")
            || source.ends_with(".bib")
        {
            source.to_string()
        } else {
            format!("{}.bib", source)
        };

        Self::fetch_bibliography(&path)
            .map(|content| Self::parse_bibtex(&content))
            .unwrap_or_default()
    }

    pub fn parse_bibtex(content: &str) -> HashMap<String, BibEntry> {
        let mut db = HashMap::new();
        let chars: Vec<char> = content.chars().collect();
        let mut pos = 0;

        while pos < chars.len() {
            if chars[pos] != '@' {
                pos += 1;
                continue;
            }
            pos += 1;

            let entry_type = Self::read_while(&chars, &mut pos, |c| c.is_alphabetic()).to_lowercase();
            Self::skip_whitespace(&chars, &mut pos);

            if pos >= chars.len() || chars[pos] != '{' {
                continue;
            }
            pos += 1; // consume '{'

            // @comment/@preamble/@string blocks carry no citable entry
            if matches!(entry_type.as_str(), "comment" | "preamble" | "string") {
                Self::skip_balanced(&chars, &mut pos);
                continue;
            }

            let key = Self::read_while(&chars, &mut pos, |c| c != ',' && c != '}').trim().to_string();
            if pos < chars.len() && chars[pos] == ',' { pos += 1; }

            let mut entry = BibEntry { fields: HashMap::new() };

            loop {
                Self::skip_whitespace(&chars, &mut pos);
                if pos >= chars.len() { break; }
                if chars[pos] == '}' { pos += 1; break; }
                if chars[pos] == ',' { pos += 1; continue; }

                let name = Self::read_while(&chars, &mut pos, |c| c != '=' && c != '}' && c != ',')
                    .trim().to_lowercase();
                if pos >= chars.len() || chars[pos] != '=' {
                    continue;
                }
                pos += 1; // consume '='
                Self::skip_whitespace(&chars, &mut pos);

                let value = Self::read_value(&chars, &mut pos);
                if !name.is_empty() {
                    entry.fields.insert(name, Self::clean_value(&value));
                }
            }

            if !key.is_empty() {
                db.insert(key, entry);
            }
        }

        db
    }

    fn read_while(chars: &[char], pos: &mut usize, keep: impl Fn(char) -> bool) -> String {
        let start = *pos;
        while *pos < chars.len() && keep(chars[*pos]) {
            *pos += 1;
        }
        chars[start..*pos].iter().collect()
    }

    fn skip_whitespace(chars: &[char], pos: &mut usize) {
        while *pos < chars.len() && chars[*pos].is_whitespace() {
            *pos += 1;
        }
    }

    // Consume up to and including the '}' that closes an already-open brace.
    fn skip_balanced(chars: &[char], pos: &mut usize) {
        let mut depth = 1usize;
        while *pos < chars.len() && depth > 0 {
            match chars[*pos] {
                '{' => depth += 1,
                '}' => depth -= 1,
                _ => {}
            }
            *pos += 1;
        }
    }

    // Field value: {braced, possibly nested}, "quoted", or a bare word.
    fn read_value(chars: &[char], pos: &mut usize) -> String {
        if *pos >= chars.len() {
            return String::new();
        }

        match chars[*pos] {
            '{' => {
                *pos += 1;
                let mut depth = 1usize;
                let mut value = String::new();
                while *pos < chars.len() {
                    let c = chars[*pos];
                    *pos += 1;
                    match c {
                        '{' => { depth += 1; value.push(c); }
                        '}' => {
                            depth -= 1;
                            if depth == 0 { break; }
                            value.push(c);
                        }
                        _ => value.push(c),
                    }
                }
                value
            }
            '"' => {
                *pos += 1;
                let mut value = String::new();
                while *pos < chars.len() && chars[*pos] != '"' {
                    value.push(chars[*pos]);
                    *pos += 1;
                }
                if *pos < chars.len() { *pos += 1; } // closing quote
                value
            }
            _ => Self::read_while(chars, pos, |c| c != ',' && c != '}' && c != '\n'),
        }
    }

    // Strip protective braces and TeX escapes, normalize for HTML output.
    fn clean_value(value: &str) -> String {
        let mut out = String::with_capacity(value.len());
        let mut chars = value.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '{' | '}' => {}
                '~' => out.push(' '),
                '&' => out.push_str("&amp;"),
                '<' => out.push_str("&lt;"),
                '>' => out.push_str("&gt;"),
                '\\' => match chars.peek() {
                    Some('&') => { chars.next(); out.push_str("&amp;"); }
                    Some('%') | Some('_') | Some('$') | Some('#') => {
                        out.push(chars.next().unwrap());
                    }
                    _ => out.push(c),
                },
                '-' if chars.peek() == Some(&'-') => {
                    chars.next();
                    out.push('–');
                }
                _ => out.push(c),
            }
        }

        out.split_whitespace().collect::<Vec<_>>().join(" ")
    }

}
