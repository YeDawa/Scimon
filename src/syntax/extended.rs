pub struct Extended;

impl Extended {
    
    pub fn rename_on_the_fly(&self, line: &str) -> Option<String> {
        let clean_line = line.replace("!ignore", "");
        let clean_line = clean_line.trim();

        if let Some((_, name_part)) = clean_line.split_once(" as ") {
            let clean_name = name_part.trim().trim_matches('"');
            let custom_name = format!("{}.pdf", clean_name.trim_end_matches(".pdf"));
            Some(custom_name)
        } else {
            None
        }
    }

}