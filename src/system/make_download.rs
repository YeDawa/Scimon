use indicatif::ProgressBar;

use std::{
    fs::File,
    error::Error,

    io::{
        Read,
        Write,
        Cursor,
    },
};

use crate::{
    cmd::extract::Extract,

    ui::{
        ui_base::UI,
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },

    utils::{
        url::UrlMisc,
        remote::Remote,
        file::FileUtils,
        validation::Validate,
    },

    system::{
        pdf::Pdf,
        providers::Providers,
    },
};

pub struct MakeDownload;

impl MakeDownload {

    // The archive's real filename (last URL path segment, without any query),
    // so its extension survives for the extractor.
    fn archive_name(url: &str) -> String {
        url.split('?').next().unwrap_or(url)
            .rsplit('/').next().unwrap_or("archive")
            .to_string()
    }

    async fn make(&self, url: &str, path: &str, final_name: &str) -> Result<String, Box<dyn Error>> {
        UrlMisc::check_url_status(url).await?;

        let (request_uri, mut filename) = Providers::new(url).get_from_provider().await?;
        let response = reqwest::get(&request_uri).await?;
        let total_size = Remote.get_file_size(&request_uri).await?;

        if !final_name.is_empty() {
            filename = final_name.to_string();
        }

        let pb = ProgressBar::new(total_size);
        pb.set_style(UI::pb_template());
    
        let output_path = FileUtils.get_output_path(path, &filename);
        let mut dest = File::create(&output_path)?;
        let content = response.bytes().await?;
        let mut reader = Cursor::new(content);

        let _ = Validate::file_type(&filename, ".pdf");
        let mut buffer = [0; 8192];
        while let Ok(size) = reader.read(&mut buffer) {
            if size == 0 { break; }
            
            dest.write_all(&buffer[..size])?;
            pb.inc(size as u64);
        }
    
        Ok(filename)
    }

    pub async fn download_line(&self, urls: &[String], url: &str, path: &str, final_name: Option<&str>, retries: u32, unzip: bool) -> Result<String, Box<dyn Error>> {
        let total = urls.len();

        // `urls` holds the primary plus any `||` fallbacks, in order. The first
        // candidate that downloads successfully wins; the rest are skipped.
        for (index, line_url) in urls.iter().enumerate() {
            let is_last = index + 1 == total;

            // `!unzip` forces the download even for non-PDF archives.
            let eligible = unzip
                || Pdf.is_pdf_file(line_url).await.unwrap_or(false)
                || (Providers::new(url).valid_provider_domain() && !line_url.contains(".md"));

            if !eligible {
                continue;
            }

            // Archives keep their real filename so extraction can detect the
            // type (the PDF-oriented naming would otherwise append `.pdf`).
            let name = if unzip && final_name.map_or(true, str::is_empty) {
                Self::archive_name(line_url)
            } else {
                final_name.unwrap_or("").to_string()
            };

            let mut attempt = 0;

            loop {
                match self.make(line_url, path, &name).await {
                    Ok(file) => {
                        let file_path = &format!("{}{}", &path, &file);
                        let password = Pdf.is_pdf_encrypted(file_path);

                        SuccessAlerts::download(&file, url, password);

                        if unzip {
                            let archive = FileUtils.get_output_path(path, &file);

                            match Extract.run(&archive.to_string_lossy()) {
                                Ok(count) => SuccessAlerts::extracted(&file, count),
                                Err(e) => ErrorsAlerts::generic(&e.to_string()),
                            }
                        }

                        return Ok(file_path.to_string())
                    },

                    Err(e) => {
                        // `!retry(N)` grants N extra attempts before giving up.
                        if attempt < retries {
                            attempt += 1;
                            ErrorsAlerts::retrying(url, attempt, retries);
                            continue;
                        }

                        // Exhausted this candidate: fall back to the next URL,
                        // or report the failure when none are left.
                        if is_last {
                            ErrorsAlerts::download(e, url);
                        } else {
                            ErrorsAlerts::fallback(line_url, &urls[index + 1]);
                        }

                        break;
                    }
                }
            }
        }

        Ok("".to_string())
    }

}
