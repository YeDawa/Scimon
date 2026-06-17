use std::{
    io::Write,
    path::Path,
    fs::read_dir,
    error::Error,
    net::TcpStream,
    collections::BTreeSet,
};

use crate::{
    consts::server::Server,

    server::{
        misc::Misc,
        logs::Logs,
        components::Components,
    },
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

    pub fn directory_listing(&self, dir: &Path, url_path: &str, root: &Path, source_name: Option<&str>) -> String {
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
            items.push_str("<li><span class=\"icon\">📁</span><a href=\"../\">../</a></li>");
        }

        for (name, is_dir) in entries {
            let display = if is_dir { format!("{}/", name) } else { name.clone() };
            let href = format!("{}{}", base, misc.percent_encode(&name));
            let href = if is_dir { format!("{}/", href) } else { href };

            let kind = misc.lightbox_kind(&name);

            let icon = if is_dir {
                "📁"
            } else {
                match kind {
                    Some("image") => "🖼️",
                    Some("pdf") => "📕",
                    _ => "📄",
                }
            };

            let attrs = match (is_dir, kind) {
                (false, Some(kind)) => format!(" class=\"lb\" data-type=\"{}\"", kind),
                _ => String::new(),
            };

            items.push_str(&format!(
                "<li><span class=\"icon\">{}</span><a{} href=\"{}\">{}</a></li>",
                icon,
                attrs,
                href,
                misc.html_escape(&display)
            ));
        }

        let escaped_base = misc.html_escape(&base);
        self.render_page(&format!("Index of {}", escaped_base), &items, source_name)
    }

    // Lists the files produced during the run as a virtual tree: entries with a
    // `/` become navigable folders. `prefix` is the folder being viewed (empty at
    // root), relative to the served root.
    pub fn produced_listing(&self, entries: &[String], prefix: &str, source_name: Option<&str>) -> String {
        let misc = Misc;
        let prefix = prefix.trim_matches('/');

        let mut folders: BTreeSet<String> = BTreeSet::new();
        let mut files: Vec<&String> = Vec::new();

        for entry in entries {
            // The part of the entry inside the current folder, or skip it.
            let rel = if prefix.is_empty() {
                Some(entry.as_str())
            } else {
                entry.strip_prefix(&format!("{}/", prefix))
            };

            let Some(rel) = rel else { continue };

            match rel.split_once('/') {
                Some((dir, _)) => { folders.insert(dir.to_string()); }
                None => files.push(entry),
            }
        }

        let mut items = String::new();

        if !prefix.is_empty() {
            items.push_str("<li><span class=\"icon\">📁</span><a href=\"../\">../</a></li>");
        }

        if folders.is_empty() && files.is_empty() {
            items.push_str("<li>No files were generated.</li>");
        }

        // Folders first.
        for dir in &folders {
            let parent = if prefix.is_empty() { String::new() } else { format!("{}/", prefix) };
            let href = format!("/{}{}/", parent, misc.percent_encode(dir));

            items.push_str(&format!(
                "<li><span class=\"icon\">📁</span><a href=\"{}\">{}/</a></li>",
                href,
                misc.html_escape(dir)
            ));
        }

        // Then files in this folder.
        for entry in files {
            let kind = misc.lightbox_kind(entry);

            let icon = match kind {
                Some("image") => "🖼️",
                Some("pdf") => "📕",
                _ => "📄",
            };

            let display = Path::new(entry)
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| entry.clone());

            let href = format!(
                "/{}",
                entry.split('/').map(|s| misc.percent_encode(s)).collect::<Vec<_>>().join("/")
            );

            let attrs = match kind {
                Some(kind) => format!(" class=\"lb\" data-type=\"{}\"", kind),
                None => String::new(),
            };

            items.push_str(&format!(
                "<li><span class=\"icon\">{}</span><a{} href=\"{}\">{}</a></li>",
                icon,
                attrs,
                href,
                misc.html_escape(&display)
            ));
        }

        let heading = if prefix.is_empty() {
            "Generated files".to_string()
        } else {
            format!("Generated files: {}", misc.html_escape(prefix))
        };

        self.render_page(&heading, &items, source_name)
    }

    fn render_page(&self, heading: &str, items: &str, source_name: Option<&str>) -> String {
        let misc = Misc;
        let components = Components;

        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n<html lang=\"en\"><head><meta charset=\"utf-8\">");
        html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">");
        html.push_str(&format!("<title>Scimon: {}</title>", heading));
        html.push_str("<style>");
        html.push_str(&components.style());
        html.push_str("</style>");
        html.push_str(&components.theme_early());
        html.push_str("</head><body>");
        html.push_str(&components.theme_toggle());
        html.push_str(&format!("<a class=\"logo\" href=\"/\">{}</a>", components.logo()));
        html.push_str(&format!("<h1>{}</h1>", heading));

        if let Some(name) = source_name {
            html.push_str(&format!(
                "<p class=\"source\"><a class=\"lb\" data-type=\"text\" href=\"{}\">📄 {}</a></p>",
                Server::SOURCE_ROUTE,
                misc.html_escape(name)
            ));
        }

        html.push_str("<ul>");
        html.push_str(items);
        html.push_str("</ul>");
        html.push_str(&components.lightbox());
        html.push_str("<script>");
        html.push_str(&components.theme_js());
        html.push_str(&components.lightbox_js());
        html.push_str("</script></body></html>");

        html
    }

}