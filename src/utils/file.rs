use std::{
    fs,
    error::Error,

    path::{
        Path, 
        PathBuf
    }
};

use reqwest::{
    Url,
    self,
    header::HeaderValue
};

use crate::generator::uuid::Uuid;

pub struct FileUtils;

impl FileUtils {

    pub fn clean_path(&self, path: &str) -> PathBuf {
        let trimmed_path = path.trim();
        let cleaned_path = trimmed_path.replace(" ", "");
    
        let mut path_buf = PathBuf::new();
        for component in cleaned_path.split('/') {
            if !component.is_empty() && component != "." && component != ".." {
                path_buf.push(component);
            }
        }
    
        path_buf
    }

    pub fn check_path_exists(&self, path: &str) -> bool {
        Path::new(&path).exists()
    }

    pub fn create_path(&self, path: &str) {
        if !&self.check_path_exists(path) {
            fs::create_dir_all(path).expect(
                &"Error creating directory".to_string()
            );
        }
    }
    
    pub fn write_file(&self, path: &str, contents: String) {
        fs::write(path, contents).expect(
            &"Error saving file".to_string()
        );
    }

    pub fn set_final_filename(&self, file: Option<String>) -> String {
        if let Some(ref filename) = file {
            if !filename.contains(".pdf") {
                filename.clone() + ".pdf"
            } else {
                filename.clone()
            }
        } else {
            let uuid_v4 = Uuid::new([0u8; 16]);
            let generated_uuid = uuid_v4.v4();
            format!("{}.pdf", generated_uuid)
        }
    }

    pub fn get_output_path(&self, path: &str, filename: &str) -> PathBuf {
        let file_path = format!("{}/{}", path, filename);
        self.clean_path(&file_path)
    }

    pub async fn detect_name(&self, url: &str, disposition: Option<&HeaderValue>, pdf: bool) -> Result<String, Box<dyn Error>> {
        let filename_option = if let Some(value) = disposition {
            let cd_string = value.to_str()?;
            let parts: Vec<&str> = cd_string.split("filename=").collect();
    
            if parts.len() > 1 {
                Some(
                    parts[1].trim_matches('"').to_string()
                )
            } else {
                None
            }
        } else {
            let parsed_url = Url::parse(url)?;
            parsed_url.path_segments()
                .and_then(|segments| segments.last())
                .map(|name| name.to_string())
        };
    
        let final_filename = if pdf == true {
            self.set_final_filename(filename_option)
        } else {
            filename_option.unwrap()
        };
        
        Ok(final_filename)
    }

    pub fn replace_extension(&self, file_name: &str, new_extension: &str) -> String {
        let mut path = PathBuf::from(file_name);

        if let Some(name) = path.file_name() {
            if let Some(name_str) = name.to_str() {
                if let Some(_) = name_str.rfind('.') {
                    path.set_extension(new_extension);
                    return path.to_string_lossy().to_string();
                }
            }
        }
        
        path.set_extension(new_extension);
        path.to_string_lossy().to_string()
    }

    pub fn human_size(&self, bytes: u64) -> String {
        let units = ["b", "k", "M", "G", "T"];
        let mut size = bytes as f64;
        let mut unit = 0;

        while size >= 1024.0 && unit < units.len() - 1 {
            size /= 1024.0;
            unit += 1;
        }

        if unit == 0 {
            format!("{}b", bytes)
        } else {
            format!("{}{}", format!("{:.1}", size).trim_end_matches(".0"), units[unit])
        }
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

}
