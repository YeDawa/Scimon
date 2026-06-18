use std::{
    path::Path, 
    time::SystemTime
};

use chrono::{
    Local,
    DateTime, 
};

pub struct Misc;

impl Misc {

    pub fn html_escape(&self, input: &str) -> String {
        input
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
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

    pub fn epoch(&self, time: Option<SystemTime>) -> i64 {
        time.and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0)
    }

    pub fn format_mtime(&self, time: SystemTime) -> String {
        let dt: DateTime<Local> = time.into();
        dt.format("%Y-%m-%d %H:%M:%S").to_string()
    }

}