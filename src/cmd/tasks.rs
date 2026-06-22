use is_url::is_url;

use std::{
    borrow::Cow,
    io::BufRead,
    error::Error,
};

use image::ImageFormat;

use crate::{
    args_cli::Flags,
    consts::uris::Uris,
    configs::settings::Settings,
    generator::qr_code::GenQrCode,

    system::{
        latex::LaTex,
        providers::Providers,
    },

    addons::{
        gemini::Gemini,
        chatgpt::ChatGPT,
    },
    
    ui::ui_base::UI,

    utils::{
        file::FileUtils,
        file_name_remote::FileNameRemote,
    },
    
    syntax::{
        vars::Vars,
        macro_handler::MacroHandler,
    },

    system::{
        markdown::Markdown,
        reporting::Reporting,
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

    pub async fn qr_codes(&self, contents: &str, custom_name: Option<&str>) -> Result<(), Box<dyn Error>> {
        if let Some(qrcode_path) = Vars.get_qrcode(contents) {
            UI::section_header("QR Codes", "normal");

            // Honor the same focus mode as downloads: with `!only` in play, only
            // the marked lines get a QR code.
            let only_mode = MacroHandler::any(contents, "only");

            let mut in_downloads_block = false;
            for line in contents.lines() {
                let trimmed = line.trim();

                if trimmed.starts_with("downloads {") {
                    in_downloads_block = true;
                    continue;
                }

                if in_downloads_block && trimmed == "}" {
                    in_downloads_block = false;
                    continue;
                }

                if !in_downloads_block {
                    continue;
                }

                if only_mode && !MacroHandler::handle_check_macro_line(line, "only") {
                    continue;
                }

                if MacroHandler::handle_check_macro_line(line, "no-qr") {
                    continue;
                }

                let url = trimmed.split_whitespace().next().unwrap_or("");
                if !MacroHandler::handle_check_macro_line(line, "ignore")
                    && !url.is_empty() && is_url(url) {
                        FileUtils.create_path(&qrcode_path);
            
                        let value = Settings.get("general.qrcode_size", "INT");
                        let qrcode_size = value.as_i64().expect("Invalid qrcode_size value. Must be an integer.") as usize;
            
                        let name = FileNameRemote::new(url).get();
                        let qr_code_name = if url.contains(Uris::PROVIDERS_DOMAINS[6]) {
                            ChatGPT::new(url, "", custom_name).title().await
                                .map(|t| t.to_string().replace(" ", "_"))
                                .unwrap_or_else(|_| name.clone())
                        } else if url.contains(Uris::PROVIDERS_DOMAINS[7]) {
                            Gemini::new(url, "", custom_name).title().await
                                .map(|t| t.to_string().replace(" ", "_"))
                                .unwrap_or_else(|_| name.clone())
                        } else {
                            name.clone()
                        };

                        let name_pdf = FileUtils.replace_extension(&qr_code_name, "png");
                        let file_path = format!("{}{}", qrcode_path, name_pdf);
                        
                        GenQrCode::new(url, qrcode_size, ImageFormat::Png).png(&file_path).unwrap();
                    }
            }
        }

        Ok(())
    }

    pub async fn download(&self, contents: Option<&str>, url: &str, path: &str, custom_name: Option<&str>, flags: &Flags, retries: u32, fallbacks: &[String]) -> Result<(), Box<dyn Error>> {
        let mut line_url = Cow::Borrowed(
            url.trim()
        );

        Reporting.check_download_errors(&line_url).await?;
        if !is_url(&line_url) { return Ok(()) }
    
        match MacroHandler::handle_ignore_macro_flag(&line_url, flags.no_ignore) {
            Ok(new_line) => line_url = Cow::Owned(new_line),
            Err(_) => return Ok(()),
        }

        if let Some(contents) = contents {
            Markdown.create(contents, url, path, custom_name).await?;
        }

        if line_url.ends_with(".tex") {
            let _ = LaTex.create_pdf(path, &line_url, custom_name).await;
        }

        if line_url.contains(Uris::PROVIDERS_DOMAINS[6]) {
            ChatGPT::new(&line_url, path, custom_name).convert().await?;
        }

        if line_url.contains(Uris::PROVIDERS_DOMAINS[7]) {
            Gemini::new(&line_url, path, custom_name).convert().await?;
        }

        if !Providers::new(&line_url).check_provider_domain() {
            let mut candidates: Vec<String> = vec![line_url.to_string()];
            candidates.extend_from_slice(fallbacks);

            MakeDownload.download_line(&candidates, url, path, custom_name, retries).await?;
        }

        Ok(())
    }

}