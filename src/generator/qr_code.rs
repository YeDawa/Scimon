use qrcode::QrCode;

use std::{
    fs::write,
    io::Cursor,
    path::Path,
    error::Error,
};

use image::{
    Luma,
    ImageFormat,
};

use crate::ui::success_alerts::SuccessAlerts;

pub struct GenQrCode {
    link: String,
    size: u32,
    format: ImageFormat,
}

impl GenQrCode {

    pub fn new(link: &str, size: usize, format: ImageFormat) -> Self {
        Self {
            link: link.to_string(),
            size: size as u32,
            format,
        }
    }

    pub fn png(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let code = QrCode::new(self.link.as_str())?;
        let image = code.render::<Luma<u8>>().max_dimensions(
            self.size, self.size
        ).build();
        
        let file_path = Path::new(file_path);
        let mut cursor = Cursor::new(Vec::new());
        image.write_to(&mut cursor, self.format)?;
        
        write(file_path, cursor.into_inner())?;
        SuccessAlerts::qrcode(file_path.to_str().unwrap_or(""));
        Ok(())
    }

}
