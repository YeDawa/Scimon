use std::{
    io::Write,
    path::Path,
    fs::read_dir,
    error::Error,
    net::TcpStream,
};

use crate::server::{
    misc::Misc,
    logs::Logs,
    components::Components,
};

pub struct Files;

impl Files {

    pub fn serve_file(
        &self,
        stream: &mut TcpStream,
        method: &str,
        target: &str,
        bytes: &[u8],
        content_type: &str,
        filename: &str,
        range: Option<&str>,
    ) -> Result<(), Box<dyn Error>> {
        let total = bytes.len();
        let disposition = format!("inline; filename=\"{}\"", filename);

        let misc = Misc;
        let (status, status_text, start, end) = match range.and_then(|r| misc.parse_range(r, total)) {
            Some((start, end)) => (206, "Partial Content", start, end),
            None => (200, "OK", 0, total.saturating_sub(1)),
        };

        let body: &[u8] = if total == 0 { &[] } else { &bytes[start..=end] };

        let mut header = format!(
            "HTTP/1.1 {} {}\r\n\
             Content-Type: {}\r\n\
             Content-Length: {}\r\n\
             Accept-Ranges: bytes\r\n\
             Content-Disposition: {}\r\n",
            status, status_text, content_type, body.len(), disposition
        );

        if status == 206 {
            header.push_str(&format!("Content-Range: bytes {}-{}/{}\r\n", start, end, total));
        }

        header.push_str("Connection: close\r\n\r\n");

        Logs.print(method, target, status);
        stream.write_all(header.as_bytes())?;

        if method != "HEAD" {
            stream.write_all(body)?;
        }

        stream.flush()?;

        Ok(())
    }

    pub fn directory_listing(&self, dir: &Path, url_path: &str, root: &Path) -> String {
        let misc = Misc;
        let mut entries: Vec<(String, bool)> = read_dir(dir)
            .into_iter()
            .flatten()
            .flatten()
            .map(|entry| {
                let name = entry.file_name().to_string_lossy().to_string();
                let is_dir = entry.path().is_dir();
                (name, is_dir)
            })
            .collect();

        entries.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.to_lowercase().cmp(&b.0.to_lowercase())));

        let base = if url_path.ends_with('/') {
            url_path.to_string()
        } else {
            format!("{}/", url_path)
        };

        let mut items = String::new();

        if dir.canonicalize().ok().as_deref() != Some(root) {
            items.push_str("<li><a href=\"../\">../</a></li>");
        }

        for (name, is_dir) in entries {
            let display = if is_dir { format!("{}/", name) } else { name.clone() };
            let href = format!("{}{}", base, misc.percent_encode(&name));
            let href = if is_dir { format!("{}/", href) } else { href };

            let attrs = match (is_dir, misc.lightbox_kind(&name)) {
                (false, Some(kind)) => format!(" class=\"lb\" data-type=\"{}\"", kind),
                _ => String::new(),
            };

            items.push_str(&format!(
                "<li><a{} href=\"{}\">{}</a></li>",
                attrs,
                href,
                misc.html_escape(&display)
            ));
        }

        let components = Components;
        let escaped_base = misc.html_escape(&base);

        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n<html lang=\"en\"><head><meta charset=\"utf-8\">");
        html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">");
        html.push_str(&format!("<title>Scimon: {}</title>", escaped_base));
        html.push_str("<style>");
        html.push_str(&components.style());
        html.push_str("</style>");
        html.push_str(&components.theme_early());
        html.push_str("</head><body>");
        html.push_str(&components.theme_toggle());
        html.push_str(&format!("<a class=\"logo\" href=\"/\">{}</a>", components.logo()));
        html.push_str(&format!("<h1>Index of {}</h1>", escaped_base));
        html.push_str("<ul>");
        html.push_str(&items);
        html.push_str("</ul>");
        html.push_str(&components.lightbox());
        html.push_str("<script>");
        html.push_str(&components.theme_js());
        html.push_str(&components.lightbox_js());
        html.push_str("</script></body></html>");

        html
    }

}