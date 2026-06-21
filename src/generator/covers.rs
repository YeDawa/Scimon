use regex::Regex;
use walkdir::WalkDir;
use image::ImageFormat;
use pdfium_render::prelude::*;

use std::{
    fs,
    io::Read,
    fs::File,
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

    async fn render(pdfium: &Pdfium, input: &Path, output: &Path, file: &str) -> Result<(), Box<dyn Error>> {
        let render_config = PdfRenderConfig::new()
            .set_target_width(2000)
            .set_maximum_height(2000)
            .rotate_if_landscape(PdfPageRenderRotation::Degrees90, true);

        let document = pdfium.load_pdf_from_file(input, None)?;
        let page = document.pages().get(0).expect("Error loading page: page not found");
        let image = page
            .render_with_config(&render_config)?
            .as_image()?
            .into_rgb8();
            
        image.save_with_format(output, ImageFormat::Jpeg)?;
        SuccessAlerts::cover_generated(file);

        Ok(())
    }

    // Finds the cover image href inside an OPF, supporting both the EPUB3
    // (`properties="cover-image"`) and EPUB2 (`<meta name="cover">`) conventions.
    fn cover_href(opf: &str) -> Option<String> {
        let href = Regex::new(r#"href="([^"]+)""#).unwrap();

        let epub3 = Regex::new(r#"<item\b[^>]*properties="[^"]*cover-image[^"]*"[^>]*>"#).unwrap();
        if let Some(tag) = epub3.find(opf) {
            if let Some(caps) = href.captures(tag.as_str()) {
                return caps.get(1).map(|m| m.as_str().to_string());
            }
        }

        let meta = Regex::new(r#"<meta\b[^>]*name="cover"[^>]*content="([^"]+)""#).unwrap();
        if let Some(id) = meta.captures(opf).and_then(|c| c.get(1)).map(|m| m.as_str().to_string()) {
            let item = Regex::new(&format!(r#"<item\b[^>]*id="{}"[^>]*>"#, regex::escape(&id))).unwrap();
            if let Some(tag) = item.find(opf) {
                if let Some(caps) = href.captures(tag.as_str()) {
                    return caps.get(1).map(|m| m.as_str().to_string());
                }
            }
        }

        None
    }

    fn epub_cover(epub_path: &Path, covers_path: &str) -> Result<(), Box<dyn Error>> {
        let mut zip = zip::ZipArchive::new(File::open(epub_path)?)?;

        let opf_path = {
            let mut container = zip.by_name("META-INF/container.xml")?;
            let mut xml = String::new();
            container.read_to_string(&mut xml)?;

            Regex::new(r#"full-path="([^"]+)""#).unwrap()
                .captures(&xml)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().to_string())
        };

        let opf_path = match opf_path {
            Some(path) => path,
            None => return Ok(()),
        };

        let opf = {
            let mut file = zip.by_name(&opf_path)?;
            let mut text = String::new();
            file.read_to_string(&mut text)?;
            text
        };

        let href = match Self::cover_href(&opf) {
            Some(href) => href,
            None => return Ok(()),
        };

        let base_dir = Path::new(&opf_path)
            .parent()
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_default();

        let entry_name = if base_dir.is_empty() {
            href.clone()
        } else {
            format!("{}/{}", base_dir, href)
        };

        let mut bytes = Vec::new();
        zip.by_name(&entry_name)?.read_to_end(&mut bytes)?;

        let stem = epub_path.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
        let ext = Path::new(&href).extension().and_then(|e| e.to_str()).unwrap_or("jpg");
        let output_file = format!("{}{}.{}", covers_path, stem, ext);

        fs::write(&output_file, bytes)?;
        SuccessAlerts::cover_generated(&output_file);

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

            let bindings = Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
                .or_else(|_| Pdfium::bind_to_system_library())?;

            let pdfium = Pdfium::new(bindings);
            for entry in WalkDir::new(pdf_path) {
                let entry = entry?;
                let path = entry.path();
                let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();

                if ext == "pdf" {
                    let file_name = path.strip_prefix(Path::new(&pdf_path)).unwrap();
                    let new_name = FileUtils.replace_extension(file_name.to_str().unwrap_or_default(), "jpg");
                    let output_path = PathBuf::from(format!("{}{}", covers_path, new_name));

                    let output_file = format!("{}{}", covers_path, new_name);
                    let _ = Self::render(&pdfium, path, &output_path, &output_file).await;
                } else if ext == "epub" {
                    let _ = Self::epub_cover(path, &covers_path);
                }
            }
        }

        Ok(())
    }
}