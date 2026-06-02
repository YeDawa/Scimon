use std::{
    env,
    fs::File,
    io::Cursor,
    path::Path,
    error::Error,
};

use crate::ui::helpers_alerts::HelpersAlerts;

pub struct PdfiumHelper;

impl PdfiumHelper {

    pub async fn ensure_pdfium_binary(&self) -> Result<(), Box<dyn Error>> {
        let (os_name, bin_name, lib_path_in_tar) = match env::consts::OS {
            "windows" => ("win", "pdfium.dll", "bin/pdfium.dll"),
            "linux" => ("linux", "libpdfium.so", "lib/libpdfium.so"),
            "macos" => ("mac", "libpdfium.dylib", "lib/libpdfium.dylib"),
            _ => return Err("Unsupported operating system for automatic download".into()),
        };

        let arch = match env::consts::ARCH {
            "x86_64" => "x64",
            "aarch64" => "arm64",
            _ => return Err("Unsupported architecture".into()),
        };

        if Path::new(bin_name).exists() {
            return Ok(());
        }

        HelpersAlerts::pdfium_not_found();
        let url = format!("https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-{}-{}.tgz", os_name, arch);

        let response = reqwest::get(&url).await?.bytes().await?;
        let cursor = Cursor::new(response);

        let tar = flate2::read::GzDecoder::new(cursor);
        let mut archive = tar::Archive::new(tar);

        for entry in archive.entries()? {
            let mut entry = entry?;
            let path = entry.path()?.to_path_buf();
            
            if path.to_string_lossy() == lib_path_in_tar {
                let mut out_file = File::create(bin_name)?;
                std::io::copy(&mut entry, &mut out_file)?;

                HelpersAlerts::pdfium_downloaded();
                return Ok(());
            }
        }
        
        Err("Binary file not found inside the downloaded package.".into())
    }

}
