use reqwest;
use is_url::is_url;

use std::{
    fs::File,
    error::Error,

    io::{
        Read, 
        Cursor
    },
};

use crate::{
    args_cli::Flags,
    syntax::vars::Vars,
    utils::validation::Validate,
    ui::errors_alerts::ErrorsAlerts,

    cmd::{
        serve::Serve,
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
    // serving the list's `path` (or the default downloads folder). Blocks until
    // interrupted, so it must run after every other step.
    pub async fn server(&self) -> Result<(), Box<dyn Error>> {
        let contents = self.raw_contents().await?;

        if let Some(port) = Vars.get_server(&contents) {
            let port: u16 = port.trim().parse()
                .map_err(|_| format!("Invalid server port: '{}'", port))?;

            let path = Vars.get_path(&contents);
            let path = if path.is_empty() { None } else { Some(path) };

            Serve::new(path, port).run()?;
        }

        Ok(())
    }

}
