use walkdir::WalkDir;
use image::ImageFormat;
use pdfium_render::prelude::*;

use std::{
    error::Error,

    path::{
        Path,
        PathBuf
    },
};

use crate::{
    syntax::vars::Vars,
    utils::file::FileUtils,
    helpers::pdfium::PdfiumHelper,

    ui::{
        ui_base::UI,
        helpers_alerts::HelpersAlerts,
        success_alerts::SuccessAlerts,
    },
};

pub struct Covers {
    contents: String,
}

impl Covers {

    pub fn new(contents: &str) -> Self {
        Self {
            contents: contents.to_string(),
        }
    }

    async fn render(input: &Path, output: &Path, file: &str) -> Result<(), Box<dyn Error>> {
        let bindings = Pdfium::bind_to_library(
            Pdfium::pdfium_platform_library_name_at_path("./"),
        )
        .or_else(|_| Pdfium::bind_to_system_library())?;
    
        let pdfium = Pdfium::new(bindings);
    
        let render_config = PdfRenderConfig::new()
            .set_target_width(2000)
            .set_maximum_height(2000)
            .rotate_if_landscape(PdfPageRenderRotation::Degrees90, true);
    
        let document = pdfium.load_pdf_from_file(input, None)?;
        let page = document.pages().get(0).expect("Page 0 not found.");
    
        let _ = page
            .render_with_config(&render_config)?
            .as_image()?
            .into_rgb8() 
            .save_with_format(output, ImageFormat::Jpeg)?;

        SuccessAlerts::cover_generated(file);
        Ok(())
    }

    pub async fn get(&self) -> Result<(), Box<dyn Error>> {
        if let Some(covers_path) = Vars.get_covers(&self.contents) {
            let pdf_path = &Vars.get_path(&self.contents);
            
            FileUtils.create_path(&covers_path);
            UI::section_header("Extracting covers", "normal");
            
            if let Err(e) = PdfiumHelper.ensure_pdfium_binary().await {
                HelpersAlerts::pdfium_download_failed(&e.to_string());
                return Err(e); 
            }

            for entry in WalkDir::new(&pdf_path) {
                let entry = entry?;
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("pdf") {
                    let file_name = path.strip_prefix(Path::new(&pdf_path)).unwrap();
                    let new_name = FileUtils.replace_extension(file_name.to_str().unwrap_or_default(), "jpg");
                    let output_path = PathBuf::from(format!("{}{}", covers_path, new_name));

                    let output_file = format!("{}{}", covers_path, new_name);
                    let _ = Self::render(path, &output_path, &output_file).await;
                }
            }
        }

        Ok(())
    }

}
