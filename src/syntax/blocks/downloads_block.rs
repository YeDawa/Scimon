use tokio::task;
use is_url::is_url;
use futures::future::join_all;

use std::{
    io::BufRead,
    error::Error,
    collections::HashSet,
};

use crate::{
    args_cli::Flags, 
    utils::file::FileUtils,
    system::providers::Providers,

    ui::{
        ui_base::UI,
        panic_alerts::PanicAlerts, 
        macros_alerts::MacrosAlerts, 
    }, 

    cmd::{
        tasks::Tasks,
        compress::Compress, 
        tasks_raw::TasksRaw,
    },

    syntax::{
        vars::Vars,
        extended::Extended,
        macro_handler::MacroHandler, 
        blocks::readme_block::ReadMeBlock, 
    },
    
    generator::{
        math::Math,
        covers::Covers,
        checksum::Checksum,
    },
};

pub struct DownloadsBlock;

impl DownloadsBlock {
    
    async fn block(&self, contents: &str, downloads_content: &str, path: &str, flags: &Flags) -> Result<(), Box<dyn Error>> {
        let mut seen_urls = HashSet::new();
        let mut tasks = Vec::new();

        for line in downloads_content.lines() {
            let url = line.split_whitespace().next().unwrap_or("");
            let final_url = Providers::new(url).arxiv();

            if line.trim().starts_with("downloads {") {
                continue;
            } else if line.trim().starts_with("}") {
                break;
            }

            if seen_urls.contains(&final_url) {
                continue;
            }

            seen_urls.insert(final_url.to_string());

            if !MacroHandler::handle_check_macro_line(line, "ignore") {
                let final_name = if let Some(custom_name) = Extended.rename_on_the_fly(line) {
                    custom_name
                } else {
                    "".to_string()
                };

                if !final_url.is_empty() && is_url(&final_url) && final_url.starts_with("http") {
                    let contents = contents.to_string();
                    let url = final_url.clone();
                    let path = path.to_string();
                    let flags = flags.clone();
                    
                    let task = task::spawn(async move {
                        let _ = Tasks.download(Some(&contents), &url, &path, Some(&final_name), &flags).await;
                    });

                    tasks.push(task);
                }
            } else {
                MacrosAlerts::ignore(&final_url);
            }
        }

        join_all(tasks).await;
        Ok(())
    }

    pub async fn read_lines<R>(&self, reader: R, flags: &Flags) -> Result<(), Box<dyn Error>> where R: BufRead {
        let contents = reader.lines().collect::<Result<Vec<_>, _>>()?.join("\n");
        let path = Vars.get_path(&contents);

        let start_index = match (contents.find("downloads {"), contents.find("downloads{")) {
            (Some(idx1), Some(idx2)) => Some(idx1.min(idx2)),
            (Some(idx), None) | (None, Some(idx)) => Some(idx),
            (None, None) => None,
        };

        let end_index = contents.rfind("}");

        if let (Some(start_index), Some(end_index)) = (start_index, end_index) {
            FileUtils.create_path(&path);
            let downloads_content = &contents[start_index + "downloads ".len()..end_index];

            if downloads_content.trim().starts_with("commands {") {
                return Ok(());
            }

            UI::section_header("downloads", "normal");
            self.block(&contents, downloads_content, &path, flags).await?;

            let _ = Covers::new(&contents).get().await;
            let _ = Compress::new(&contents).get();
            let _ = Tasks.qr_codes(&contents, None).await;
            let _ = Math::new(&contents).render().await;

            Vars.get_open(&contents, flags.no_open_link).await;
            let _ = ReadMeBlock.render_var_and_save_file(&contents, flags).await;

            let _ = Checksum::new(Some(contents)).files();
        } else {
            PanicAlerts::downloads_block();
        }

        Ok(())
    }

    pub async fn read_lines_raw(&self, contents: &str, flags: &Flags) -> Result<(), Box<dyn Error>> {
        let contents = contents.lines().collect::<Vec<_>>().join("\n");
        let path = Vars.get_path(&contents);

        let start_index = match (contents.find("downloads {"), contents.find("downloads{")) {
            (Some(idx1), Some(idx2)) => Some(idx1.min(idx2)),
            (Some(idx), None) | (None, Some(idx)) => Some(idx),
            (None, None) => None,
        };

        let end_index = contents.rfind("}");

        if let (Some(start_index), Some(end_index)) = (start_index, end_index) {
            FileUtils.create_path(&path);
            let downloads_content = &contents[start_index + "downloads ".len()..end_index];

            if downloads_content.trim().starts_with("commands {") {
                return Ok(());
            }

            UI::section_header("downloads", "normal");
            self.block(&contents, downloads_content, &path, flags).await?;

            let _ = Compress::new(&contents).get();
            let _ = Covers::new(&contents).get().await;
            let _ = TasksRaw.qr_codes(&contents, None).await;
            let _ = Math::new(&contents).render().await;

            Vars.get_open(&contents, flags.no_open_link).await;
            let _ = ReadMeBlock.render_var_and_save_file(&contents, flags).await;

            let _ = Checksum::new(Some(contents)).files();
        } else {
            PanicAlerts::downloads_block();
        }

        Ok(())
    }

}
