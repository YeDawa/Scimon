use std::{
    thread,
    fs::read,
    sync::Arc,
    error::Error,

    io::{
        BufRead,
        BufReader,
    },

    net::{
        TcpStream,
        TcpListener,
    },

    path::{
        Path,
        PathBuf
    },
};

use crate::{
    utils::file::FileUtils,
    generator::checksum::Checksum,

    ui::{
        ui_base::UI,
        server_alerts::ServerAlerts,
    },
    
    consts::{
        server::Server,
        folders::Folders,
    },

    server::{
        pages::Pages,
        files::Files,
        stream::Stream,
        encoders::Encoders,
    },
};

pub struct Serve {
    port: u16,
    root: PathBuf,
    source: Option<Arc<(String, String)>>,
    files: Option<Arc<Vec<String>>>,
    archive: Option<Arc<(String, PathBuf)>>,
    scripts: Option<Arc<Vec<String>>>,
}

impl Serve {

    pub fn new(path: Option<String>, port: u16) -> Self {
        let root = match path {
            Some(path) => PathBuf::from(path),
            None => Folders::DOWNLOAD_FOLDER.clone(),
        };

        Self { root, port, source: None, files: None, archive: None, scripts: None }
    }

    pub fn with_source(mut self, name: String, body: String) -> Self {
        self.source = Some(Arc::new((name, body)));
        self
    }

    pub fn with_files(mut self, files: Vec<String>) -> Self {
        self.files = Some(Arc::new(files));
        self
    }

    pub fn with_archive(mut self, name: String, path: PathBuf) -> Self {
        self.archive = Some(Arc::new((name, path)));
        self
    }

    pub fn with_scripts(mut self, scripts: Vec<String>) -> Self {
        self.scripts = Some(Arc::new(scripts));
        self
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        UI::section_header("Web Server", "normal");

        if !self.root.is_dir() {
            return Err(
                format!("Directory not found: {}", self.root.display()).into()
            );
        }

        let root = self.root.canonicalize()?;
        let addr = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(&addr)?;

        let url = &format!("http://{}", addr);
        ServerAlerts::started(self.port, url);
        ServerAlerts::to_quit();

        for stream in listener.incoming() {
            let Ok(stream) = stream else { continue };
            let root = root.clone();
            let source = self.source.clone();
            let files = self.files.clone();
            let archive = self.archive.clone();
            let scripts = self.scripts.clone();

            thread::spawn(move || {
                let _ = Self::handle(stream, &root, source, files, archive, scripts);
            });
        }

        Ok(())
    }

    fn handle(mut stream: TcpStream, root: &Path, source: Option<Arc<(String, String)>>, files: Option<Arc<Vec<String>>>, archive: Option<Arc<(String, PathBuf)>>, scripts: Option<Arc<Vec<String>>>) -> Result<(), Box<dyn Error>> {
        let stream_instance = Stream;
        let mut reader = BufReader::new(stream.try_clone()?);
        let mut request_line = String::new();
        reader.read_line(&mut request_line)?;

        let mut parts = request_line.split_whitespace();
        let method = parts.next().unwrap_or("");
        let raw_target = parts.next().unwrap_or("/");

        let mut range: Option<String> = None;
        loop {
            let mut line = String::new();
            if reader.read_line(&mut line)? == 0 {
                break;
            }

            let line = line.trim_end();
            if line.is_empty() {
                break;
            }

            if let Some((name, value)) = line.split_once(':') {
                if name.eq_ignore_ascii_case("range") {
                    range = Some(value.trim().to_string());
                }
            }
        }

        if method != "GET" && method != "HEAD" {
            return stream_instance.respond(&mut stream, 405, "Method Not Allowed", "text/plain; charset=utf-8", Pages.method_not_allowed().as_bytes());
        }

        let target = raw_target.split('?').next().unwrap_or("/");
        let decoded = Encoders.percent_decode(target);

        if decoded == Server::SOURCE_ROUTE {
            if let Some(source) = &source {
                ServerAlerts::logs(method, target, 200);
                return stream_instance.respond(&mut stream, 200, "OK", "text/plain; charset=utf-8", source.1.as_bytes());
            }

            ServerAlerts::logs(method, target, 404);
            return stream_instance.respond(&mut stream, 404, "Not Found", "text/html; charset=utf-8", Pages.not_found().as_bytes());
        }

        if decoded == Server::ARCHIVE_ROUTE {
            if let Some(archive) = &archive {
                if let Ok(bytes) = read(&archive.1) {
                    let filename = archive.0.replace('"', "");
                    return Files.serve_file(&mut stream, method, target, &bytes, "application/zip", &filename, range.as_deref());
                }
            }

            ServerAlerts::logs(method, target, 404);
            return stream_instance.respond(&mut stream, 404, "Not Found", "text/html; charset=utf-8", Pages.not_found().as_bytes());
        }

        if let Some(rest) = decoded.strip_prefix(Server::SCRIPT_ROUTE) {
            if let (Some(scripts), Ok(idx)) = (&scripts, rest.parse::<usize>()) {
                if let Some(url) = scripts.get(idx) {
                    match stream_instance.fetch_remote(url) {
                        Ok(text) => {
                            ServerAlerts::logs(method, target, 200);
                            return stream_instance.respond(&mut stream, 200, "OK", "text/plain; charset=utf-8", text.as_bytes());
                        }

                        Err(_) => {
                            ServerAlerts::logs(method, target, 502);
                            return stream_instance.respond(&mut stream, 502, "Bad Gateway", "text/plain; charset=utf-8", b"Failed to fetch script.");
                        }
                    }
                }
            }

            ServerAlerts::logs(method, target, 404);
            return stream_instance.respond(&mut stream, 404, "Not Found", "text/html; charset=utf-8", Pages.not_found().as_bytes());
        }

        if decoded == Server::SCRIPTS_ROUTE {
            let source_name = source.as_ref().map(|s| s.0.as_str());
            let files_vec: &[String] = files.as_deref().map(|f| f.as_slice()).unwrap_or(&[]);
            let scripts_vec: &[String] = scripts.as_deref().map(|s| s.as_slice()).unwrap_or(&[]);

            let body = Files.scripts_listing(files_vec, scripts_vec, source_name);
            ServerAlerts::logs(method, target, 200);
            return stream_instance.respond(&mut stream, 200, "OK", "text/html; charset=utf-8", body.as_bytes());
        }

        // Compute a produced file's SHA-256 on demand.
        if let Some(rest) = decoded.strip_prefix(Server::CHECKSUM_HASH_ROUTE) {
            if let (Some(files), Ok(idx)) = (&files, rest.parse::<usize>()) {
                if let Some(rel) = files.get(idx) {
                    if let Some(full) = root.join(rel).to_str() {
                        if let Ok(hash) = Checksum::new(None).hash(full) {
                            ServerAlerts::logs(method, target, 200);
                            return stream_instance.respond(&mut stream, 200, "OK", "text/plain; charset=utf-8", hash.as_bytes());
                        }
                    }
                }
            }

            ServerAlerts::logs(method, target, 404);
            return stream_instance.respond(&mut stream, 404, "Not Found", "text/html; charset=utf-8", Pages.not_found().as_bytes());
        }

        let files_instance = Files;
        let stream_instance = Stream;
        let Some(path) = stream_instance.resolve(root, &decoded) else {
            ServerAlerts::logs(method, target, 403);
            return stream_instance.respond(&mut stream, 403, "Forbidden", "text/plain; charset=utf-8", Pages.forbiden().as_bytes());
        };

        if !path.exists() {
            ServerAlerts::logs(method, target, 404);
            return stream_instance.respond(&mut stream, 404, "Not Found", "text/html; charset=utf-8", Pages.not_found().as_bytes());
        }

        let Some(path) = path.canonicalize().ok().filter(|p| p.starts_with(root)) else {
            ServerAlerts::logs(method, target, 403);
            return stream_instance.respond(&mut stream, 403, "Forbidden", "text/plain; charset=utf-8", Pages.forbiden().as_bytes());
        };

        if path.is_dir() {
            let source_name = source.as_ref().map(|s| s.0.as_str());
            let archive_ref = archive.as_ref().map(|a| (a.0.as_str(), a.1.as_path()));
            let has_scripts = scripts.as_ref().map(|s| !s.is_empty()).unwrap_or(false);

            let body = match &files {
                Some(files) => {
                    let prefix = path.strip_prefix(root)
                        .map(|p| p.to_string_lossy().replace('\\', "/"))
                        .unwrap_or_default();

                    files_instance.produced_listing(root, files, &prefix, source_name, archive_ref, has_scripts)
                }

                None => files_instance.directory_listing(&path, &decoded, root, source_name),
            };

            ServerAlerts::logs(method, target, 200);
            return stream_instance.respond(&mut stream, 200, "OK", "text/html; charset=utf-8", body.as_bytes());
        }

        match read(&path) {
            Ok(bytes) => {
                let content_type = FileUtils.content_type(&path);
                let filename = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("file")
                    .replace('"', "");

                files_instance.serve_file(&mut stream, method, target, &bytes, content_type, &filename, range.as_deref())
            }

            Err(_) => {
                ServerAlerts::logs(method, target, 500);
                stream_instance.respond(&mut stream, 500, "Internal Server Error", "text/plain; charset=utf-8", Pages.internal_server_error().as_bytes())
            }
        }
    }

}
