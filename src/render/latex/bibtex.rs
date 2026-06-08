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
            ureq::get(source)
                .call()
                .map_err(|e| e.to_string())?
                .body_mut()
                .read_to_string()
                .map_err(|e| e.to_string())
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

}