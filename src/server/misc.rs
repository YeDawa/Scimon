use std::path::Path;

pub struct Misc;

impl Misc {

    pub fn html_escape(&self, input: &str) -> String {
        input
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
    }

    pub fn percent_decode(&self, input: &str) -> String {
        let bytes = input.as_bytes();
        let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
        let mut i = 0;

        while i < bytes.len() {
            if bytes[i] == b'%' && i + 2 < bytes.len() {
                let hi = (bytes[i + 1] as char).to_digit(16);
                let lo = (bytes[i + 2] as char).to_digit(16);

                if let (Some(hi), Some(lo)) = (hi, lo) {
                    out.push((hi * 16 + lo) as u8);
                    i += 3;
                    continue;
                }
            }

            out.push(bytes[i]);
            i += 1;
        }

        String::from_utf8_lossy(&out).to_string()
    }

    pub fn percent_encode(&self, input: &str) -> String {
        let mut out = String::with_capacity(input.len());

        for &byte in input.as_bytes() {
            match byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    out.push(byte as char);
                }
                _ => out.push_str(&format!("%{:02X}", byte)),
            }
        }

        out
    }

    pub fn content_type(&self, path: &Path) -> &'static str {
        match path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase().as_str() {
            "pdf" => "application/pdf",
            "html" | "htm" => "text/html; charset=utf-8",
            "css" => "text/css; charset=utf-8",
            "js" => "text/javascript; charset=utf-8",
            "json" => "application/json; charset=utf-8",
            "txt" | "md" | "sha256" | "sha1" | "sha512" | "md5" | "crc32" => "text/plain; charset=utf-8",
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "svg" => "image/svg+xml",
            "webp" => "image/webp",
            "ico" => "image/x-icon",
            "zip" => "application/zip",
            "gz" | "tgz" => "application/gzip",
            "tar" => "application/x-tar",
            "epub" => "application/epub+zip",
            _ => "application/octet-stream",
        }
    }

    pub fn lightbox_kind(&self, name: &str) -> Option<&'static str> {
        match Path::new(name)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase()
            .as_str()
        {
            "png" | "jpg" | "jpeg" | "gif" | "svg" | "webp" | "ico" | "bmp" | "avif" => Some("image"),
            "pdf" => Some("pdf"),
            "sha256" | "sha1" | "sha512" | "md5" | "crc32" => Some("text"),
            _ => None,
        }
    }

    pub fn parse_range(&self, header: &str, total: usize) -> Option<(usize, usize)> {
        if total == 0 {
            return None;
        }

        let spec = header.trim().strip_prefix("bytes=")?;
        let spec = spec.split(',').next()?.trim();
        let (start_str, end_str) = spec.split_once('-')?;

        if start_str.is_empty() {
            let n: usize = end_str.parse().ok()?;
            if n == 0 {
                return None;
            }

            return Some((total.saturating_sub(n), total - 1));
        }

        let start: usize = start_str.parse().ok()?;
        if start >= total {
            return None;
        }

        let end = if end_str.is_empty() {
            total - 1
        } else {
            end_str.parse::<usize>().ok()?.min(total - 1)
        };

        (start <= end).then_some((start, end))
    }

}