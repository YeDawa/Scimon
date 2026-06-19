use std::{
    io,
    path::Path
};

pub struct Validate;

impl Validate {

    pub fn file(file: &str) -> io::Result<()> {
        if !Path::new(file).exists() {
            println!();
            
            return Err(
                io::Error::new(
                    io::ErrorKind::NotFound, format!(
                        "The file {} does not exist.", file
                    )
                )
            );
        }
    
        Self::file_type(file, ".mon")?;
        Ok(())
    }
    
    pub fn file_type(file: &str, file_type: &str) -> io::Result<()> {
        if !file.ends_with(file_type) {
            return Err(
                io::Error::new(
                    io::ErrorKind::InvalidData, format!(
                        "The file {} is not a {}", file, file_type
                    )
                )
            );
        }
    
        Ok(())
    }
    
}
