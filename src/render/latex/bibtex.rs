use reqwest::blocking::get;
use std::{
    fs,
    collections::HashMap,
};

#[derive(Debug, Clone)]
pub struct BibEntry {
    pub author: String,
    pub title: String,
    pub year: String,
}

pub struct BibTextRender;

impl BibTextRender {

    pub fn fetch_bibliography(source: &str) -> Result<String, String> {
        if source.starts_with("http://") || source.starts_with("https://") {
            let response = get(source).map_err(|e| e.to_string())?;
            response.text().map_err(|e| e.to_string())
        } else {
            fs::read_to_string(source).map_err(|e| e.to_string())
        }
    }

    pub fn parse_bibtex(content: &str) -> HashMap<String, BibEntry> {
        let mut db = HashMap::new();
        let lower_content = content.to_lowercase();
        
        for block in lower_content.split('@').skip(1) {
            if let Some(start) = block.find('{') {
                let key_end = block[start + 1..].find(',').unwrap_or(0) + start + 1;
                let key = block[start + 1..key_end].trim().to_string();

                let extract = |field: &str| -> String {
                    if let Some(pos) = block.find(field) {
                        let start_val = block[pos..].find('{').unwrap_or(0) + pos + 1;
                        let end_val = block[start_val..].find('}').unwrap_or(0) + start_val;
                        
                        block[start_val..end_val].trim().to_string()
                    } else {
                        "".to_string()
                    }
                };

                db.insert(key, BibEntry { 
                    author: extract("author"), 
                    title: extract("title"), 
                    year: extract("year") 
                });
            }
        }

        db
    }

    pub fn process_document(input: &str) -> (HashMap<String, BibEntry>, String) {
        let mut bib_db = HashMap::new();
        let mut main_content = input.to_string();

        if let Some(start) = main_content.find(r"\bibliography{") {
            if let Some(end) = main_content[start..].find('}') {
                let end_idx = start + end + 1;
                let source = &main_content[start + 14..start + end];
                
                if let Ok(content) = Self::fetch_bibliography(source) {
                    bib_db = Self::parse_bibtex(&content);
                }
                
                let mut html_list = String::from(r#"<div class="bibliography"><h3>References</h3><ul>"#);
                for (key, entry) in &bib_db {
                    html_list.push_str(&format!(
                        r#"<li id="ref-{}"><strong>{}</strong>. <em>{}</em> ({})</li>"#,
                        key, entry.author, entry.title, entry.year
                    ));
                }
                
                html_list.push_str("</ul></div>");
                main_content.replace_range(start..end_idx, &html_list);
            }
        }

        (bib_db, main_content)
    }

}