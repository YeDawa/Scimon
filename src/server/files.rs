use std::{
    io::Write,
    path::Path,
    error::Error,
    net::TcpStream,
    collections::BTreeSet,

    fs::{
        metadata,
        read_dir,
    },
};

use crate::{
    consts::server::Server,

    server::{
        misc::Misc,
        logs::Logs,
        icons::Icons,
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
            items.push_str(&format!(
                "<li>{}<a href=\"../\">../</a></li>",
                Icons.icon("corner-up-left")
            ));
        }

        for (name, is_dir) in entries {
            let display = if is_dir { format!("{}/", name) } else { name.clone() };
            let href = format!("{}{}", base, misc.percent_encode(&name));
            let href = if is_dir { format!("{}/", href) } else { href };

            let kind = misc.lightbox_kind(&name);
            let icon = if is_dir { "folder" } else { Icons.file_icon(kind) };

            let attrs = match (is_dir, kind) {
                (false, Some(kind)) => format!(" class=\"lb\" data-type=\"{}\"", kind),
                _ => String::new(),
            };

            items.push_str(&format!(
                "<li>{}<a{} href=\"{}\">{}</a></li>",
                Icons.icon(icon),
                attrs,
                href,
                misc.html_escape(&display)
            ));
        }

        let escaped_base = misc.html_escape(&base);
        self.render_page(&format!("Index of {}", escaped_base), &format!("<ul>{}</ul>", items), source_name, "")
    }

    pub fn produced_listing(&self, root: &Path, entries: &[String], prefix: &str, source_name: Option<&str>, archive: Option<(&str, &Path)>) -> String {
        let misc = Misc;
        let prefix = prefix.trim_matches('/');

        let mut files: Vec<&String> = Vec::new();
        for entry in entries {
            let rel = if prefix.is_empty() {
                Some(entry.as_str())
            } else {
                entry.strip_prefix(&format!("{}/", prefix))
            };

            if let Some(rel) = rel {
                if !rel.contains('/') {
                    files.push(entry);
                }
            }
        }

        let mut all_folders: BTreeSet<String> = BTreeSet::new();
        for entry in entries {
            let parts: Vec<&str> = entry.split('/').collect();
            for i in 1..parts.len() {
                all_folders.insert(parts[..i].join("/"));
            }
        }

        let mut rows = String::new();
        let sidebar = Components.folder_nav(&misc, &all_folders, prefix);

        for entry in files {
            let kind = misc.lightbox_kind(entry);

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

            let meta = metadata(root.join(entry));
            let size = meta.as_ref().map(|m| m.len()).unwrap_or(0);
            let mtime = meta.as_ref().ok().and_then(|m| m.modified().ok());

            rows.push_str(&format!(
                "<tr data-dir=\"0\" data-name=\"{0}\" data-size=\"{1}\" data-mtime=\"{2}\">\
                 <td class=\"name\"><a{3} href=\"{4}\">{5}{0}</a></td>\
                 <td class=\"meta\">{6}</td><td class=\"meta num\">{7}</td></tr>",
                misc.html_escape(&display),
                size,
                misc.epoch(mtime),
                attrs,
                href,
                Icons.icon(Icons.file_icon(kind)),
                mtime.map(|t| misc.format_mtime(t)).unwrap_or_default(),
                misc.human_size(size),
            ));
        }

        if prefix.is_empty() {
            if let Some((name, apath)) = archive {
                let meta = metadata(apath);
                let size = meta.as_ref().map(|m| m.len()).unwrap_or(0);
                let mtime = meta.as_ref().ok().and_then(|m| m.modified().ok());

                rows.push_str(&format!(
                    "<tr data-dir=\"0\" data-name=\"{0}\" data-size=\"{1}\" data-mtime=\"{2}\">\
                     <td class=\"name\"><a href=\"{3}\" download=\"{0}\">{4}{0}</a></td>\
                     <td class=\"meta\">{5}</td><td class=\"meta num\">{6}</td></tr>",
                    misc.html_escape(name),
                    size,
                    misc.epoch(mtime),
                    Server::ARCHIVE_ROUTE,
                    Icons.icon("file-archive"),
                    mtime.map(|t| misc.format_mtime(t)).unwrap_or_default(),
                    misc.human_size(size),
                ));
            }
        }

        let body = if entries.is_empty() {
            "<p>No files were generated.</p>".to_string()
        } else {
            let index: Vec<serde_json::Value> = entries.iter().map(|entry| {
                let meta = metadata(root.join(entry));
                let size = meta.as_ref().map(|m| m.len()).unwrap_or(0);
                let mtime = meta.as_ref().ok().and_then(|m| m.modified().ok());
                let kind = misc.lightbox_kind(entry);
                let href = format!(
                    "/{}",
                    entry.split('/').map(|s| misc.percent_encode(s)).collect::<Vec<_>>().join("/")
                );

                serde_json::json!({
                    "p": entry,
                    "h": href,
                    "s": misc.human_size(size),
                    "m": mtime.map(|t| misc.format_mtime(t)).unwrap_or_default(),
                    "t": kind.unwrap_or(""),
                    "i": Icons.file_icon(kind),
                    "size": size,
                    "mtime": misc.epoch(mtime),
                })
            }).collect();

            let index_json = serde_json::to_string(&index).unwrap_or_else(|_| "[]".to_string());

            let tbody = if rows.is_empty() {
                "<tr><td colspan=\"3\" class=\"meta\">No files in this folder.</td></tr>".to_string()
            } else {
                rows
            };

            let mut body = String::new();
            body.push_str("<script>window.__scimonFiles=");
            body.push_str(&index_json);
            body.push_str(";</script>");
            body.push_str("<input id=\"search\" class=\"search\" type=\"search\" placeholder=\"Search files…\" autocomplete=\"off\">");
            body.push_str(
                "<table class=\"files\"><thead><tr>\
                 <th data-key=\"name\">Name<span class=\"arrow\"></span></th>\
                 <th data-key=\"mtime\">Modified<span class=\"arrow\"></span></th>\
                 <th class=\"num\" data-key=\"size\">Size<span class=\"arrow\"></span></th>\
                 </tr></thead><tbody>"
            );
            body.push_str(&tbody);
            body.push_str("</tbody></table>");
            body
        };

        let heading = if prefix.is_empty() {
            "Generated files".to_string()
        } else {
            format!("Generated files: {}", misc.html_escape(prefix))
        };

        self.render_page(&heading, &body, source_name, &sidebar)
    }

    fn render_page(&self, heading: &str, body: &str, source_name: Option<&str>, sidebar: &str) -> String {
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
        html.push_str("<div class=\"layout\">");

        html.push_str("<aside class=\"sidebar\">");
        html.push_str(&format!("<a class=\"logo\" href=\"/\">{}</a>", components.logo()));

        if let Some(name) = source_name {
            html.push_str(&format!(
                "<a class=\"item lb\" data-type=\"text\" href=\"{}\">{} {}</a>",
                Server::SOURCE_ROUTE,
                Icons.icon("file-code"),
                misc.html_escape(name)
            ));
        }

        html.push_str(sidebar);
        html.push_str("</aside>");

        html.push_str("<main class=\"main\">");
        html.push_str(&format!("<h1>{}</h1>", heading));
        html.push_str(body);
        html.push_str("</main></div>");

        html.push_str(&components.lightbox());
        html.push_str("<script>");
        html.push_str(&components.theme_js());
        html.push_str(&components.lightbox_js());
        html.push_str(&components.table_js());
        html.push_str(&components.search_js());
        html.push_str("</script>");
        html.push_str("<script src=\"https://unpkg.com/lucide@latest\"></script>");
        html.push_str("<script>lucide.createIcons();</script>");
        html.push_str("</body></html>");

        html
    }

}