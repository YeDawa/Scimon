use is_url::is_url;

use std::error::Error;

use image::ImageFormat;

use crate::{
    consts::uris::Uris,
    configs::settings::Settings,
    generator::qr_code::GenQrCode,
    
    ui::{
        ui_base::UI,
        success_alerts::SuccessAlerts,
    },

    utils::{
        file::FileUtils,
        file_name_remote::FileNameRemote,
    },
    
    syntax::{
        vars::Vars,
        macro_handler::MacroHandler,
    },

    addons::{
        gemini::Gemini,
        chatgpt::ChatGPT,
    },
};

pub struct TasksRaw;

impl TasksRaw {

    pub async fn prints(&self, content: &str) -> Result<(), Box<dyn Error>> {
        let contents = content.to_string();

        for line in contents.lines() {
            Vars.get_print(&line);
        }

        Ok(())
    }

    pub async fn qr_codes(&self, contents: &str, custom_name: Option<&str>) -> Result<(), Box<dyn Error>> {
        if let Some(qrcode_path) = Vars.get_qrcode(contents) {
            UI::section_header("QR Codes", "normal");

            for line in contents.lines() {
                let url = line.trim().split_whitespace().next().unwrap_or("");

                if line.trim().starts_with("downloads {") || line.trim().starts_with("}") {
                    continue;
                }

                if !MacroHandler::handle_check_macro_line(&line, "ignore") {
                    if !url.is_empty() && is_url(&url) {
                        FileUtils.create_path(&qrcode_path);
            
                        let value = Settings.get("general.qrcode_size", "INT");
                        let qrcode_size = value.as_i64().expect("Invalid qrcode_size value. Must be an integer.") as usize;
            
                        let name = FileNameRemote::new(url).get();
                        let qr_code_name = if url.contains(Uris::PROVIDERS_DOMAINS[7]) {
                            ChatGPT::new(&url, "", custom_name).title()?.to_string().replace(" ", "_")
                        } else if url.contains(Uris::PROVIDERS_DOMAINS[8]) {
                            Gemini::new(&url, "", custom_name).title()?.to_string().replace(" ", "_")
                        } else {
                            name
                        };

                        let name_pdf = FileUtils.replace_extension(&qr_code_name, "png");
                        let file_path = format!("{}{}", qrcode_path, name_pdf);
                        
                        GenQrCode::new(&url, qrcode_size, ImageFormat::Png).png(&file_path).unwrap();
                        SuccessAlerts::qrcode(file_path.as_str());
                    }
                }
            }
        }

        Ok(())
    }

}
