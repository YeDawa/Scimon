extern crate colored;

use colored::*;

use std::{
    thread,
    error::Error,
    path::{Path, PathBuf},

    fs::{
        read,
        read_dir,
    },

    io::{
        Write,
        BufRead,
        BufReader,
    },

    net::{
        TcpStream,
        TcpListener,
    },
};

use crate::{
    consts::folders::Folders,

    ui::{
        ui_base::UI,
        server_alerts::ServerAlerts,
    },
};

const LOGO_PNG: &[u8] = include_bytes!("../../assets/logo.png");
const LOGO_ROUTE: &str = "/__scimon/logo.png";

pub struct Serve {
    root: PathBuf,
    port: u16,
}

impl Serve {

    pub fn new(path: Option<String>, port: u16) -> Self {
        let root = match path {
            Some(path) => PathBuf::from(path),
            None => Folders::DOWNLOAD_FOLDER.clone(),
        };

        Self { root, port }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        UI::header();
        UI::section_header("Web Server", "info");

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

            thread::spawn(move || {
                let _ = Self::handle(stream, &root);
            });
        }

        Ok(())
    }

    fn handle(mut stream: TcpStream, root: &Path) -> Result<(), Box<dyn Error>> {
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
            return Self::respond(&mut stream, 405, "Method Not Allowed", "text/plain; charset=utf-8", b"405 Method Not Allowed");
        }

        let target = raw_target.split('?').next().unwrap_or("/");
        let decoded = Self::percent_decode(target);

        if decoded == LOGO_ROUTE {
            return Self::respond(&mut stream, 200, "OK", "image/png", LOGO_PNG);
        }

        let Some(path) = Self::resolve(root, &decoded) else {
            Self::log(method, target, 403);
            return Self::respond(&mut stream, 403, "Forbidden", "text/plain; charset=utf-8", b"403 Forbidden");
        };

        if !path.exists() {
            Self::log(method, target, 404);
            return Self::respond(&mut stream, 404, "Not Found", "text/html; charset=utf-8", Self::not_found().as_bytes());
        }

        let Some(path) = path.canonicalize().ok().filter(|p| p.starts_with(root)) else {
            Self::log(method, target, 403);
            return Self::respond(&mut stream, 403, "Forbidden", "text/plain; charset=utf-8", b"403 Forbidden");
        };

        if path.is_dir() {
            let body = Self::directory_listing(&path, &decoded, root);
            Self::log(method, target, 200);
            return Self::respond(&mut stream, 200, "OK", "text/html; charset=utf-8", body.as_bytes());
        }

        match read(&path) {
            Ok(bytes) => {
                let content_type = Self::content_type(&path);
                let filename = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("file")
                    .replace('"', "");

                Self::serve_file(&mut stream, method, target, &bytes, content_type, &filename, range.as_deref())
            }

            Err(_) => {
                Self::log(method, target, 500);
                Self::respond(&mut stream, 500, "Internal Server Error", "text/plain; charset=utf-8", b"500 Internal Server Error")
            }
        }
    }

    fn serve_file(
        stream: &mut TcpStream,
        method: &str,
        target: &str,
        bytes: &[u8],
        content_type: &str,
        filename: &str,
        range: Option<&str>,
    ) -> Result<(), Box<dyn Error>> {
        let total = bytes.len();
        let disposition = format!("inline; filename=\"{}\"", filename);

        let (status, status_text, start, end) = match range.and_then(|r| Self::parse_range(r, total)) {
            Some((start, end)) => (206, "Partial Content", start, end),
            None => (200, "OK", 0, total.saturating_sub(1)),
        };

        let body: &[u8] = if total == 0 { &[] } else { &bytes[start..=end] };

        let mut header = format!(
            "HTTP/1.1 {} {}\r\n\
             Content-Type: {}\r\n\
             Content-Length: {}\r\n\
             Accept-Ranges: bytes\r\n\
             Content-Disposition: {}\r\n",
            status, status_text, content_type, body.len(), disposition
        );

        if status == 206 {
            header.push_str(&format!("Content-Range: bytes {}-{}/{}\r\n", start, end, total));
        }

        header.push_str("Connection: close\r\n\r\n");

        Self::log(method, target, status);
        stream.write_all(header.as_bytes())?;

        if method != "HEAD" {
            stream.write_all(body)?;
        }

        stream.flush()?;

        Ok(())
    }

    fn parse_range(header: &str, total: usize) -> Option<(usize, usize)> {
        if total == 0 {
            return None;
        }

        let spec = header.trim().strip_prefix("bytes=")?;
        let spec = spec.split(',').next()?.trim();
        let (start_str, end_str) = spec.split_once('-')?;

        if start_str.is_empty() {
            let n: usize = end_str.parse().ok()?;
            if n == 0 {
                return None;
            }

            return Some((total.saturating_sub(n), total - 1));
        }

        let start: usize = start_str.parse().ok()?;
        if start >= total {
            return None;
        }

        let end = if end_str.is_empty() {
            total - 1
        } else {
            end_str.parse::<usize>().ok()?.min(total - 1)
        };

        (start <= end).then_some((start, end))
    }

    fn resolve(root: &Path, url_path: &str) -> Option<PathBuf> {
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

    fn respond(
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

    fn directory_listing(dir: &Path, url_path: &str, root: &Path) -> String {
        let mut entries: Vec<(String, bool)> = read_dir(dir)
            .into_iter()
            .flatten()
            .flatten()
            .map(|entry| {
                let name = entry.file_name().to_string_lossy().to_string();
                let is_dir = entry.path().is_dir();
                (name, is_dir)
            })
            .collect();

        entries.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.to_lowercase().cmp(&b.0.to_lowercase())));

        let base = if url_path.ends_with('/') {
            url_path.to_string()
        } else {
            format!("{}/", url_path)
        };

        let mut items = String::new();

        if dir.canonicalize().ok().as_deref() != Some(root) {
            items.push_str("<li><a href=\"../\">../</a></li>");
        }

        for (name, is_dir) in entries {
            let display = if is_dir { format!("{}/", name) } else { name.clone() };
            let href = format!("{}{}", base, Self::percent_encode(&name));
            let href = if is_dir { format!("{}/", href) } else { href };

            let attrs = match (is_dir, Self::lightbox_kind(&name)) {
                (false, Some(kind)) => format!(" class=\"lb\" data-type=\"{}\"", kind),
                _ => String::new(),
            };

            items.push_str(&format!(
                "<li><a{} href=\"{}\">{}</a></li>",
                attrs,
                href,
                Self::html_escape(&display)
            ));
        }

        let escaped_base = Self::html_escape(&base);

        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n<html lang=\"en\"><head><meta charset=\"utf-8\">");
        html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">");
        html.push_str(&format!("<title>Scimon: {}</title>", escaped_base));
        html.push_str("<style>");
        html.push_str(Self::STYLE);
        html.push_str("</style>");
        html.push_str(Self::THEME_EARLY);
        html.push_str("</head><body>");
        html.push_str(Self::THEME_TOGGLE);
        html.push_str(&format!("<a class=\"logo\" href=\"/\">{}</a>", Self::logo()));
        html.push_str(&format!("<h1>Index of {}</h1>", escaped_base));
        html.push_str("<ul>");
        html.push_str(&items);
        html.push_str("</ul>");
        html.push_str(Self::LIGHTBOX_HTML);
        html.push_str("<script>");
        html.push_str(Self::THEME_JS);
        html.push_str(Self::LIGHTBOX_JS);
        html.push_str("</script></body></html>");

        html
    }

    const THEME_EARLY: &'static str = "<script>(function(){try{\
        var t=localStorage.getItem('scimon-theme');\
        if(t)document.documentElement.setAttribute('data-theme',t);\
        }catch(e){}})();</script>";

    const THEME_TOGGLE: &'static str =
        "<button id=\"themeBtn\" class=\"theme-toggle\" aria-label=\"Toggle theme\"></button>";

    const THEME_JS: &'static str = r#"
        (function(){
            var root=document.documentElement;
            var btn=document.getElementById('themeBtn');
            if(!btn)return;
            function isDark(){
                var t=root.getAttribute('data-theme');
                if(t)return t==='dark';
                return window.matchMedia&&window.matchMedia('(prefers-color-scheme: dark)').matches;
            }
            function refresh(){btn.textContent=isDark()?'☀️':'🌙';}
            refresh();
            btn.addEventListener('click',function(){
                var next=isDark()?'light':'dark';
                root.setAttribute('data-theme',next);
                try{localStorage.setItem('scimon-theme',next);}catch(e){}
                refresh();
            });
        })();
    "#;

    const STYLE: &'static str = r#"
        :root{--bg:#fff;--fg:#1a1a1a;--link:#2563eb;--muted:#777;--logo-filter:none;
            --lb-img-bg:transparent;--lb-img-pad:0;}
        @media (prefers-color-scheme:dark){
            :root{--bg:#0f1115;--fg:#e6e6e6;--link:#6ea8fe;--muted:#9aa0a6;--logo-filter:brightness(1.7);
                --lb-img-bg:#fff;--lb-img-pad:8px;}
        }
        :root[data-theme="light"]{--bg:#fff;--fg:#1a1a1a;--link:#2563eb;--muted:#777;--logo-filter:none;
            --lb-img-bg:transparent;--lb-img-pad:0;}
        :root[data-theme="dark"]{--bg:#0f1115;--fg:#e6e6e6;--link:#6ea8fe;--muted:#9aa0a6;--logo-filter:brightness(1.7);
            --lb-img-bg:#fff;--lb-img-pad:8px;}
        body{font-family:system-ui,sans-serif;margin:2rem;background:var(--bg);color:var(--fg);}
        .logo{display:inline-block;margin-bottom:1.2rem;}
        .logo img{display:block;filter:var(--logo-filter);}
        h1{font-size:1.2rem;}
        ul{list-style:none;padding:0;}
        li{padding:.2rem 0;}
        a{text-decoration:none;color:var(--link);}
        a:hover{text-decoration:underline;}
        .theme-toggle{position:fixed;top:1rem;right:1rem;background:transparent;
            border:1px solid var(--muted);color:var(--fg);border-radius:6px;
            padding:.3rem .55rem;cursor:pointer;font-size:1rem;line-height:1;}
        .theme-toggle:hover{border-color:var(--fg);}
        #lb{position:fixed;inset:0;background:rgba(0,0,0,.85);display:none;
            align-items:center;justify-content:center;z-index:1000;}
        #lb.open{display:flex;}
        #lb .figure{margin:0;display:flex;flex-direction:column;align-items:center;gap:.7rem;}
        #lb .stage{display:flex;align-items:center;justify-content:center;}
        #lb img{max-width:90vw;max-height:82vh;border-radius:4px;
            background:var(--lb-img-bg);padding:var(--lb-img-pad);box-sizing:border-box;}
        #lb iframe{width:90vw;height:82vh;border:0;border-radius:4px;background:#fff;}
        #lb pre{margin:0;background:#111;color:#eee;padding:1rem 1.2rem;border-radius:4px;
            max-width:90vw;max-height:82vh;overflow:auto;white-space:pre-wrap;
            word-break:break-all;font-family:ui-monospace,Consolas,monospace;font-size:.9rem;}
        #lb .cap{color:#eee;font-size:.95rem;max-width:90vw;text-align:center;
            overflow-wrap:anywhere;}
        #lb .btn{position:absolute;color:#fff;cursor:pointer;user-select:none;
            font-size:2rem;padding:.4rem 1rem;opacity:.8;line-height:1;}
        #lb .btn:hover{opacity:1;}
        #lb .close{top:1rem;right:1.5rem;font-size:2.4rem;}
        #lb .prev{left:.5rem;top:50%;transform:translateY(-50%);}
        #lb .next{right:.5rem;top:50%;transform:translateY(-50%);}
    "#;

    const LIGHTBOX_HTML: &'static str = "<div id=\"lb\">\
        <span class=\"btn close\">&times;</span>\
        <span class=\"btn prev\">&#10094;</span>\
        <figure class=\"figure\"><div class=\"stage\"></div><figcaption class=\"cap\"></figcaption></figure>\
        <span class=\"btn next\">&#10095;</span></div>";

    const LIGHTBOX_JS: &'static str = r#"
        (function(){
            var lb=document.getElementById('lb');
            var stage=lb.querySelector('.stage');
            var cap=lb.querySelector('.cap');
            var links=[].slice.call(document.querySelectorAll('a.lb'));
            var i=0;
            if(links.length<2){
                lb.querySelector('.prev').style.display='none';
                lb.querySelector('.next').style.display='none';
            }
            function show(n){
                i=(n+links.length)%links.length;
                var link=links[i];
                var type=link.getAttribute('data-type');
                var href=link.getAttribute('href');
                stage.innerHTML='';
                if(type==='image'){
                    var img=document.createElement('img');
                    img.src=href;img.alt=link.textContent;
                    stage.appendChild(img);
                }else if(type==='pdf'){
                    var frame=document.createElement('iframe');
                    frame.src=href;
                    stage.appendChild(frame);
                }else{
                    var pre=document.createElement('pre');
                    pre.textContent='Loading…';
                    stage.appendChild(pre);
                    fetch(href).then(function(r){return r.text();})
                        .then(function(t){pre.textContent=t;})
                        .catch(function(){pre.textContent='Failed to load file.';});
                }
                cap.textContent=link.textContent;
            }
            function open(n){show(n);lb.classList.add('open');}
            function close(){lb.classList.remove('open');stage.innerHTML='';}
            links.forEach(function(a,idx){
                a.addEventListener('click',function(e){e.preventDefault();open(idx);});
            });
            lb.addEventListener('click',function(e){
                var t=e.target;
                if(t.classList.contains('next')){show(i+1);}
                else if(t.classList.contains('prev')){show(i-1);}
                else if(t.classList.contains('close')||t===lb||t.classList.contains('stage')||t.classList.contains('figure')){close();}
            });
            document.addEventListener('keydown',function(e){
                if(!lb.classList.contains('open'))return;
                if(e.key==='Escape')close();
                else if(e.key==='ArrowRight'&&links.length>1)show(i+1);
                else if(e.key==='ArrowLeft'&&links.length>1)show(i-1);
            });
        })();
    "#;

    fn lightbox_kind(name: &str) -> Option<&'static str> {
        match Path::new(name)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase()
            .as_str()
        {
            "png" | "jpg" | "jpeg" | "gif" | "svg" | "webp" | "ico" | "bmp" | "avif" => Some("image"),
            "pdf" => Some("pdf"),
            "sha256" | "sha1" | "sha512" | "md5" | "crc32" => Some("text"),
            _ => None,
        }
    }

    fn not_found() -> String {
        format!(
            "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><title>404</title>\
             <style>{0}</style>{1}</head>\
             <body>{2}<a class=\"logo\" href=\"/\">{3}</a><h1>404 Not Found</h1>\
             <p><a href=\"/\">Back to index</a></p>\
             <script>{4}</script></body></html>",
            Self::STYLE,
            Self::THEME_EARLY,
            Self::THEME_TOGGLE,
            Self::logo(),
            Self::THEME_JS,
        )
    }

    fn logo() -> String {
        format!("<img src=\"{}\" alt=\"SciMon\" height=\"40\">", LOGO_ROUTE)
    }

    fn content_type(path: &Path) -> &'static str {
        match path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase().as_str() {
            "pdf" => "application/pdf",
            "html" | "htm" => "text/html; charset=utf-8",
            "css" => "text/css; charset=utf-8",
            "js" => "text/javascript; charset=utf-8",
            "json" => "application/json; charset=utf-8",
            "txt" | "md" | "sha256" | "sha1" | "sha512" | "md5" | "crc32" => "text/plain; charset=utf-8",
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "svg" => "image/svg+xml",
            "webp" => "image/webp",
            "ico" => "image/x-icon",
            "zip" => "application/zip",
            "gz" | "tgz" => "application/gzip",
            "tar" => "application/x-tar",
            "epub" => "application/epub+zip",
            _ => "application/octet-stream",
        }
    }

    fn log(method: &str, target: &str, status: u16) {
        let status_str = match status {
            200 => status.to_string().green(),
            400..=499 => status.to_string().yellow(),
            _ => status.to_string().red(),
        };

        println!("{} {} {}", status_str, method.bold(), target);
    }

    fn percent_decode(input: &str) -> String {
        let bytes = input.as_bytes();
        let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
        let mut i = 0;

        while i < bytes.len() {
            if bytes[i] == b'%' && i + 2 < bytes.len() {
                let hi = (bytes[i + 1] as char).to_digit(16);
                let lo = (bytes[i + 2] as char).to_digit(16);

                if let (Some(hi), Some(lo)) = (hi, lo) {
                    out.push((hi * 16 + lo) as u8);
                    i += 3;
                    continue;
                }
            }

            out.push(bytes[i]);
            i += 1;
        }

        String::from_utf8_lossy(&out).to_string()
    }

    fn percent_encode(input: &str) -> String {
        let mut out = String::with_capacity(input.len());

        for &byte in input.as_bytes() {
            match byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    out.push(byte as char);
                }
                _ => out.push_str(&format!("%{:02X}", byte)),
            }
        }

        out
    }

    fn html_escape(input: &str) -> String {
        input
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
    }

}
