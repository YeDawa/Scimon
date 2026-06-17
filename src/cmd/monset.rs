use reqwest;
use walkdir::WalkDir;
use is_url::is_url;

use std::{
    fs::File,
    path::Path,
    error::Error,
    time::SystemTime,

    io::{
        Read,
        Cursor
    },
};

use crate::{
    args_cli::Flags,
    syntax::vars::Vars,
    server::serve::Serve,
    utils::validation::Validate,
    ui::errors_alerts::ErrorsAlerts,

    cmd::{
        tasks::Tasks,
        tasks_raw::TasksRaw,
    },

    syntax::blocks::{
        runner_block::RunnerBlock,
        downloads_block::DownloadsBlock
    },
};

pub struct Monset{
    pub run: String,
}

impl Monset {

    pub fn new(run: &str) -> Self {
        Self {
            run: run.to_string(),
        }
    }

    async fn read_file(&self) -> Result<Cursor<Vec<u8>>, Box<dyn Error>> {
        let mut buffer = Vec::new();

        if is_url(&self.run) {
            let response = reqwest::get(&self.run).await?;

            if !response.status().is_success() {
                ErrorsAlerts::generic(
                    &format!("Error while retrieving remote file: {}", response.status())
                );
            }

            let bytes = response.bytes().await?;
            buffer.extend_from_slice(&bytes);
        } else {
            let _ = Validate::file(&self.run).map_err(|e| {
                ErrorsAlerts::generic(&e.to_string());
            });

            let mut file = File::open(&self.run)?;
            file.read_to_end(&mut buffer)?;
        }

        Ok(Cursor::new(buffer))
    }

    pub async fn downloads(&self, flags: &Flags) -> Result<(), Box<dyn Error>> {
        let mut reader = self.read_file().await?;
        let reader_clone = reader.clone();
        
        let _ = Tasks.prints(reader_clone).await?;
        let _ = DownloadsBlock.read_lines(&mut reader, &flags).await?;

        Ok(())
    }

    pub async fn downloads_raw(&self, flags: &Flags) -> Result<(), Box<dyn Error>> {
        let content = self.run.clone();

        let _ = TasksRaw.prints(&content).await?;
        let _ = DownloadsBlock.read_lines_raw(&content, &flags).await?;

        Ok(())
    }

    pub async fn run_code(&self, flags: &Flags) -> Result<(), Box<dyn Error>> {
        let mut reader = self.read_file().await?;
        RunnerBlock.read_lines(&mut reader, flags).await?;

        Ok(())
    }

    pub async fn run_code_raw(&self, flags: &Flags) -> Result<(), Box<dyn Error>> {
        let content = self.run.clone();
        RunnerBlock.read_lines_raw(&content, flags).await?;

        Ok(())
    }

    pub async fn raw_contents(&self) -> Result<String, Box<dyn Error>> {
        let cursor = self.read_file().await?;
        Ok(String::from_utf8_lossy(cursor.get_ref()).to_string())
    }

    pub fn has_downloads_block(contents: &str) -> bool {
        contents.contains("downloads {") || contents.contains("downloads{")
    }

    // Starts the built-in web server when the list declares `server "PORT"`,
    // serving only the files produced during this run (modified at/after
    // `started`). Blocks until interrupted, so it must run after every other step.
    pub async fn server(&self, started: SystemTime) -> Result<(), Box<dyn Error>> {
        let contents = self.raw_contents().await?;

        if let Some(port) = Vars.get_server(&contents) {
            let port: u16 = port.trim().parse()
                .map_err(|_| format!("Invalid server port: '{}'", port))?;

            let path = Vars.get_path(&contents);
            let path = if path.is_empty() { None } else { Some(path) };

            let name = self.run
                .rsplit(|c| c == '/' || c == '\\')
                .next()
                .unwrap_or("list.mon")
                .to_string();

            let files = path
                .as_deref()
                .map(|p| Self::produced_files(p, started))
                .unwrap_or_default();

            Serve::new(path, port)
                .with_source(name, contents.clone())
                .with_files(files)
                .run()?;
        }

        Ok(())
    }

    // Collects files under `root` written during this run (mtime >= started),
    // as paths relative to `root` with forward slashes.
    fn produced_files(root: &str, started: SystemTime) -> Vec<String> {
        let root_path = Path::new(root);
        let mut files: Vec<String> = WalkDir::new(root_path)
            .into_iter()
            .flatten()
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| {
                entry.metadata().ok()
                    .and_then(|m| m.modified().ok())
                    .map(|modified| modified >= started)
                    .unwrap_or(false)
            })
            .filter_map(|entry| {
                entry.path()
                    .strip_prefix(root_path)
                    .ok()
                    .map(|rel| rel.to_string_lossy().replace('\\', "/"))
            })
            .collect();

        files.sort();
        files
    }

}
