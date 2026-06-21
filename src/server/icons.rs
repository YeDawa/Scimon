pub struct Icons;

impl Icons {

    pub fn icon(&self, name: &str) -> String {
        format!("<span class=\"icon\"><i data-lucide=\"{}\"></i></span>", name)
    }

    pub fn file_icon(&self, kind: Option<&str>) -> &'static str {
        match kind {
            Some("image") => "image",
            Some("pdf") => "file-text",
            Some("zip") => "file-archive",
            _ => "file",
        }
    }

}