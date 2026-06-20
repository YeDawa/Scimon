pub struct Extended;

impl Extended {
    
    pub fn rename_on_the_fly(&self, line: &str) -> Option<String> {
        let clean_line = line.replace("!ignore", "");
        let clean_line = clean_line.trim();

        if let Some((_, name_part)) = clean_line.split_once(" as ") {
            let clean_name = name_part.trim().trim_matches('"');
            let lower = clean_name.to_lowercase();

            if lower.ends_with(".pdf") || lower.ends_with(".epub") {
                Some(clean_name.to_string())
            } else {
                Some(format!("{}.pdf", clean_name))
            }
        } else {
            None
        }
    }

}