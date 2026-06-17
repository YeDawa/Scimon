use std::{
    io::Write,
    error::Error,
    net::TcpStream,

    path::{
        Path, 
        PathBuf
    },
};

pub struct Stream;

impl Stream {
    
    pub fn resolve(&self, root: &Path, url_path: &str) -> Option<PathBuf> {
        let mut path = root.to_path_buf();
        for segment in url_path.split('/') {
            match segment {
                "" | "." => continue,
                ".." => return None,
                other => path.push(other),
            }
        }

        Some(path)
    }

    pub fn respond(
        &self,
        stream: &mut TcpStream,
        status: u16,
        status_text: &str,
        content_type: &str,
        body: &[u8],
    ) -> Result<(), Box<dyn Error>> {
        let header = format!(
            "HTTP/1.1 {} {}\r\n\
             Content-Type: {}\r\n\
             Content-Length: {}\r\n\
             Connection: close\r\n\r\n",
            status, status_text, content_type, body.len()
        );

        stream.write_all(header.as_bytes())?;
        stream.write_all(body)?;
        stream.flush()?;

        Ok(())
    }

}