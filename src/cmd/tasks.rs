use is_url::is_url;
use image::ImageFormat;

use std::{
    path::Path,
    borrow::Cow,
    io::BufRead,
    error::Error,
};

use crate::{
    args_cli::Flags,
    configs::settings::Settings,
    generator::qr_code::GenQrCode,

    ui::{
        ui_base::UI,
        success_alerts::SuccessAlerts,
    },

    utils::{
        remote::Remote,
        file::FileUtils,
        file_name_remote::FileNameRemote,
    },
    
    syntax::{
        vars::Vars,
        macro_handler::MacroHandler,
    },

    system::{
        latex::LaTex,
        markdown::Markdown,
        reporting::Reporting,
        providers::Providers,
        make_download::MakeDownload,
    },
};

pub struct Tasks;

impl Tasks {

    pub async fn prints<R>(&self, reader: R) -> Result<(), Box<dyn Error>> where R: BufRead {
        let contents = reader.lines().collect::<Result<Vec<_>, _>>()?.join("\n");

        for line in contents.lines() {
            Vars.get_print(line);
        }

        Ok(())
    }

    pub async fn qr_codes(&self, contents: &str, _custom_name: Option<&str>) -> Result<(), Box<dyn Error>> {
        if let Some(qrcode_path) = Vars.get_qrcode(contents) {
            UI::section_header("QR Codes", "normal");
            let only_mode = MacroHandler::any(contents, "only");

            let mut depth = 0;
            for line in contents.lines() {
                let trimmed = line.trim();

                if depth == 0 {
                    if trimmed.starts_with("downloads {") {
                        depth = 1;
                    }

                    continue;
                }

                if trimmed.ends_with('{') {
                    depth += 1;
                    continue;
                }

                if trimmed.starts_with('}') {
                    depth -= 1;
                    continue;
                }

                if only_mode && !MacroHandler::handle_check_macro_line(line, "only") {
                    continue;
                }

                if MacroHandler::handle_check_macro_line(line, "no-qr") {
                    continue;
                }

                let url = trimmed.split_whitespace().next().unwrap_or("");
                if !MacroHandler::handle_check_macro_line(line, "ignore") && !url.is_empty() && is_url(url) {
                    FileUtils.create_path(&qrcode_path);
        
                    let value = Settings.get("general.qrcode_size", "INT");
                    let qrcode_size = value.as_i64().expect("Invalid qrcode_size value. Must be an integer.") as usize;
        
                    let qr_code_name = FileNameRemote::new(url).get();
                    let name_pdf = FileUtils.replace_extension(&qr_code_name, "png");
                    let file_path = format!("{}{}", qrcode_path, name_pdf);

                    if Path::new(&file_path).exists() {
                        SuccessAlerts::skipped(&file_path);
                    } else {
                        GenQrCode::new(url, qrcode_size, ImageFormat::Png).png(&file_path).unwrap();
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn download(&self, contents: Option<&str>, url: &str, path: &str, custom_name: Option<&str>, flags: &Flags, retries: u32, fallbacks: &[String], unzip: bool) -> Result<String, Box<dyn Error>> {
        let mut line_url = Cow::Borrowed(
            url.trim()
        );

        Reporting.check_download_errors(&line_url).await?;
        if !is_url(&line_url) { return Ok("".to_string()) }
    
        match MacroHandler::handle_ignore_macro_flag(&line_url, flags.no_ignore) {
            Ok(new_line) => line_url = Cow::Owned(new_line),
            Err(_) => return Ok("".to_string()),
        }

        let mut output_path = "".to_string();

        if let Some(contents) = contents {
            if Remote.check_content_type(url, "text/markdown").await.unwrap_or(false) || url.contains(".md") {
                output_path = Markdown.create(contents, url, path, custom_name).await?;
            }
        }

        if line_url.ends_with(".tex") {
            output_path = LaTex.create_pdf(path, &line_url, custom_name).await?;
        }

        if !Providers::new(&line_url).check_provider_domain() {
            let mut candidates: Vec<String> = vec![line_url.to_string()];
            candidates.extend_from_slice(fallbacks);

            let download_path = MakeDownload.download_line(&candidates, url, path, custom_name, retries, unzip).await?;
            if !download_path.is_empty() {
                output_path = download_path;
            }
        }

        Ok(output_path)
    }

}