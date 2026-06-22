pub struct Extended;

impl Extended {
    
    pub fn rename_on_the_fly(&self, line: &str) -> Option<String> {
        let (_, after_as) = line.split_once(" as ")?;
        let after_as = after_as.trim();

        let clean_name = if let Some(rest) = after_as.strip_prefix('"') {
            rest.split('"').next().unwrap_or("").trim().to_string()
        } else {
            after_as.split_whitespace().next().unwrap_or("").to_string()
        };

        if clean_name.is_empty() {
            return None;
        }

        let lower = clean_name.to_lowercase();

        if lower.ends_with(".pdf") || lower.ends_with(".epub") {
            Some(clean_name)
        } else {
            Some(format!("{}.pdf", clean_name))
        }
    }

}