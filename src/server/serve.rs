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
    ui::{
        ui_base::UI,
        server_alerts::ServerAlerts,
    },
    
    consts::{
        server::Server,
        folders::Folders,
    },

    server::{
        misc::Misc,
        logs::Logs,
        pages::Pages,
        files::Files,
        stream::Stream,
    },
};

pub struct Serve {
    port: u16,
    root: PathBuf,
    // The reference `.mon` that started the server: (display name, contents).
    source: Option<Arc<(String, String)>>,
    // Files produced during the run (relative to root); when set, the root page
    // lists only these instead of browsing the whole directory.
    files: Option<Arc<Vec<String>>>,
}

impl Serve {

    pub fn new(path: Option<String>, port: u16) -> Self {
        let root = match path {
            Some(path) => PathBuf::from(path),
            None => Folders::DOWNLOAD_FOLDER.clone(),
        };

        Self { root, port, source: None, files: None }
    }

    // Attaches the reference `.mon` so it can be viewed from the server.
    pub fn with_source(mut self, name: String, body: String) -> Self {
        self.source = Some(Arc::new((name, body)));
        self
    }

    // Restricts the root listing to the files produced during the run.
    pub fn with_files(mut self, files: Vec<String>) -> Self {
        self.files = Some(Arc::new(files));
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
        ServerAlerts::started(self.port, &url);
        ServerAlerts::to_quit();

        for stream in listener.incoming() {
            let Ok(stream) = stream else { continue };
            let root = root.clone();
            let source = self.source.clone();
            let files = self.files.clone();

            thread::spawn(move || {
                let _ = Self::handle(stream, &root, source, files);
            });
        }

        Ok(())
    }

    fn handle(mut stream: TcpStream, root: &Path, source: Option<Arc<(String, String)>>, files: Option<Arc<Vec<String>>>) -> Result<(), Box<dyn Error>> {
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
        let decoded = Misc.percent_decode(target);

        if decoded == Server::SOURCE_ROUTE {
            if let Some(source) = &source {
                Logs.print(method, target, 200);
                return stream_instance.respond(&mut stream, 200, "OK", "text/plain; charset=utf-8", source.1.as_bytes());
            }

            Logs.print(method, target, 404);
            return stream_instance.respond(&mut stream, 404, "Not Found", "text/html; charset=utf-8", Pages.not_found().as_bytes());
        }

        let files_instance = Files;
        let stream_instance = Stream;
        let Some(path) = stream_instance.resolve(root, &decoded) else {
            Logs.print(method, target, 403);
            return stream_instance.respond(&mut stream, 403, "Forbidden", "text/plain; charset=utf-8", Pages.forbiden().as_bytes());
        };

        if !path.exists() {
            Logs.print(method, target, 404);
            return stream_instance.respond(&mut stream, 404, "Not Found", "text/html; charset=utf-8", Pages.not_found().as_bytes());
        }

        let Some(path) = path.canonicalize().ok().filter(|p| p.starts_with(root)) else {
            Logs.print(method, target, 403);
            return stream_instance.respond(&mut stream, 403, "Forbidden", "text/plain; charset=utf-8", Pages.forbiden().as_bytes());
        };

        if path.is_dir() {
            let source_name = source.as_ref().map(|s| s.0.as_str());

            // At the root, show only the produced files when that list is set.
            let body = match &files {
                Some(files) if path == root => files_instance.produced_listing(files, source_name),
                _ => files_instance.directory_listing(&path, &decoded, root, source_name),
            };

            Logs.print(method, target, 200);
            return stream_instance.respond(&mut stream, 200, "OK", "text/html; charset=utf-8", body.as_bytes());
        }

        match read(&path) {
            Ok(bytes) => {
                let content_type = Misc.content_type(&path);
                let filename = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("file")
                    .replace('"', "");

                files_instance.serve_file(&mut stream, method, target, &bytes, content_type, &filename, range.as_deref())
            }

            Err(_) => {
                Logs.print(method, target, 500);
                stream_instance.respond(&mut stream, 500, "Internal Server Error", "text/plain; charset=utf-8", Pages.internal_server_error().as_bytes())
            }
        }
    }

}
