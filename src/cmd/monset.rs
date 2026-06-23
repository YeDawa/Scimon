use reqwest;
use is_url::is_url;
use walkdir::WalkDir;

use std::{
    fs::File,
    error::Error,

    io::{
        Read,
        Cursor
    },

    path::{
        Path,
        PathBuf,
    },
};

use crate::{
    args_cli::Flags,
    server::serve::Serve,
    configs::package::Package,
    utils::validation::Validate,
    ui::errors_alerts::ErrorsAlerts,

    cmd::{
        tasks::Tasks,
        tasks_raw::TasksRaw,
    },

    syntax::{
        vars::Vars,
        loops::Loops,
        comments::Comments,
        includes::Includes,
        functions::Functions,

        blocks::{
            runner_block::RunnerBlock,
            downloads_block::DownloadsBlock
        }
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

        let raw = String::from_utf8_lossy(&buffer).to_string();
        let raw = if is_url(&self.run) {
            raw
        } else {
            format!("{}{}", Package.metadata_block(&self.run), Package.strip_inline(&raw))
        };

        let clean = Comments.strip(&raw);
        let clean = Includes.expand(&clean);
        let clean = Loops.expand(&clean);
        let clean = Functions.expand(&clean);
        let clean = Vars.interpolate(&clean);

        Ok(Cursor::new(clean.into_bytes()))
    }

    pub async fn downloads(&self, flags: &Flags) -> Result<(), Box<dyn Error>> {
        let mut reader = self.read_file().await?;
        let reader_clone = reader.clone();
        
        Tasks.prints(reader_clone).await?;
        DownloadsBlock.read_lines(&mut reader, flags).await?;

        Ok(())
    }

    pub async fn downloads_raw(&self, flags: &Flags) -> Result<(), Box<dyn Error>> {
        let content = Comments.strip(&self.run);
        let content = Includes.expand(&content);
        let content = Loops.expand(&content);
        let content = Functions.expand(&content);
        let content = Vars.interpolate(&content);

        TasksRaw.prints(&content).await?;
        DownloadsBlock.read_lines_raw(&content, flags).await?;

        Ok(())
    }

    pub async fn run_code(&self, flags: &Flags) -> Result<(), Box<dyn Error>> {
        let mut reader = self.read_file().await?;
        RunnerBlock.read_lines(&mut reader, flags).await?;

        Ok(())
    }

    pub async fn run_code_raw(&self, flags: &Flags) -> Result<(), Box<dyn Error>> {
        let content = Comments.strip(&self.run);
        let content = Includes.expand(&content);
        let content = Loops.expand(&content);
        let content = Functions.expand(&content);
        let content = Vars.interpolate(&content);
        RunnerBlock.read_lines_raw(&content, flags).await?;

        Ok(())
    }

    pub async fn raw_contents(&self) -> Result<String, Box<dyn Error>> {
        let cursor = self.read_file().await?;
        Ok(String::from_utf8_lossy(cursor.get_ref()).to_string())
    }

    pub async fn server(&self) -> Result<(), Box<dyn Error>> {
        let contents = self.raw_contents().await?;

        if let Some(port) = Vars.get_server(&contents) {
            let port: u16 = port.trim().parse()
                .map_err(|_| format!("Invalid server port: '{}'", port))?;

            let path = Vars.get_path(&contents);
            let path = if path.is_empty() { None } else { Some(path) };

            let name = self.run
                .rsplit(['/', '\\'])
                .next()
                .unwrap_or("list.mon")
                .to_string();

            let mut files = path
                .as_deref()
                .map(Self::produced_files)
                .unwrap_or_default();

            let archive = Self::produced_archive(&contents);
            if let (Some(root), Some((_, apath))) = (path.as_deref(), &archive) {
                if let Ok(rel) = apath.strip_prefix(Path::new(root)) {
                    let rel = rel.to_string_lossy().replace('\\', "/");
                    files.retain(|f| f != &rel);
                }
            }

            let scripts = Self::commands_scripts(&contents);

            let mut serve = Serve::new(path, port)
                .with_source(name, contents.clone())
                .with_files(files)
                .with_scripts(scripts);

            if let Some((archive_name, archive_path)) = archive {
                serve = serve.with_archive(archive_name, archive_path);
            }

            serve.run()?;
        }

        Ok(())
    }

    fn commands_scripts(contents: &str) -> Vec<String> {
        let mut scripts = Vec::new();
        let mut in_block = false;

        for line in contents.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with("commands {") || trimmed.starts_with("commands{") {
                in_block = true;
                continue;
            }

            if in_block {
                if trimmed.starts_with('}') {
                    break;
                }

                if trimmed.starts_with("http") {
                    scripts.push(trimmed.to_string());
                }
            }
        }

        scripts
    }

    fn produced_archive(contents: &str) -> Option<(String, PathBuf)> {
        let zip = Vars.get_compress(contents)?;
        let path = PathBuf::from(&zip);

        if path.is_file() {
            let name = path.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "archive.zip".to_string());

            Some((name, path))
        } else {
            None
        }
    }

    fn produced_files(root: &str) -> Vec<String> {
        let root_path = Path::new(root);
        let mut files: Vec<String> = WalkDir::new(root_path)
            .into_iter()
            .flatten()
            .filter(|entry| entry.file_type().is_file())
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
