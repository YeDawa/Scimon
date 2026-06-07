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
    
}
