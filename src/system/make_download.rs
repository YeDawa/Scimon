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
    system::{
        pdf::Pdf,
        providers::Providers,
    },

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
};

pub struct MakeDownload;

impl MakeDownload {

    async fn make(&self, url: &str, path: &str) -> Result<String, Box<dyn Error>> {
        UrlMisc::check_url_status(url).await?;

        let (request_uri, filename) = Providers::new(url).get_from_provider().await?;
        let response = reqwest::get(&request_uri).await?;
        let total_size = Remote.get_file_size(&request_uri).await?;

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

    pub async fn download_line(&self, line_url: &str, url: &str, path: &str) -> Result<String, Box<dyn Error>> {
        if Pdf.is_pdf_file(&line_url).await? || Providers::new(url).valid_provider_domain() && !line_url.contains(".md") {
            let result = self.make(&line_url, path).await;
            
            match result {
                Ok(file) => {
                    let file_path = &format!("{}{}", &path, &file);
                    let password = Pdf.is_pdf_encrypted(&file_path);
                    
                    SuccessAlerts::download(&file, url, password);
                    return Ok(file_path.to_string())
                },

                Err(e) => ErrorsAlerts::download(e, url),
            }
        }

        Ok("".to_string())
    }

    pub async fn download_doi(&self, line_url: &str, url: &str, path: &str) -> Result<String, Box<dyn Error>> {
        let result = self.make(&line_url, path).await;
            
        match result {
            Ok(file) => {
                let file_path = &format!("{}{}", &path, &file);
                SuccessAlerts::download(&file, url, false);
                return Ok(file_path.to_string())
            },

            Err(e) => ErrorsAlerts::download(e, url),
        }

        Ok("".to_string())
    }

}
