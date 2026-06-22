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
        merge::Merge,
        covers::Covers,
        convert::Convert,
        checksum::Checksum,
    },
};

pub struct DownloadsBlock;

impl DownloadsBlock {
    
    async fn block(&self, contents: &str, downloads_content: &str, path: &str, flags: &Flags) -> Result<(), Box<dyn Error>> {
        let mut seen_urls = HashSet::new();
        let mut tasks = Vec::new();

        // Focus mode: when any line is tagged `!only`, every untagged line is
        // skipped so just the marked entries are downloaded.
        let only_mode = MacroHandler::any(downloads_content, "only");

        for line in downloads_content.lines() {
            if line.trim().starts_with("downloads {") {
                continue;
            } else if line.trim().starts_with("}") {
                break;
            }

            if only_mode && !MacroHandler::handle_check_macro_line(line, "only") {
                continue;
            }

            // A line may list fallback URLs separated by `||`; the first one that
            // works wins. Each candidate is normalized (e.g. arxiv abs -> pdf).
            let candidates: Vec<String> = line
                .split("||")
                .filter_map(|segment| segment.trim().split_whitespace().next())
                .map(|url| Providers::new(url).arxiv())
                .collect();

            let Some(primary) = candidates.first().cloned() else {
                continue;
            };

            if seen_urls.contains(&primary) {
                continue;
            }

            seen_urls.insert(primary.clone());

            if !MacroHandler::handle_check_macro_line(line, "ignore") {
                let final_name = if let Some(custom_name) = Extended.rename_on_the_fly(line) {
                    custom_name
                } else {
                    "".to_string()
                };

                if !primary.is_empty() && is_url(&primary) && primary.starts_with("http") {
                    let contents = contents.to_string();
                    let path = path.to_string();
                    let flags = flags.clone();
                    let retries = MacroHandler::retry_count(line);
                    let fallbacks: Vec<String> = candidates[1..].to_vec();

                    let task = task::spawn(async move {
                        let _ = Tasks.download(Some(&contents), &primary, &path, Some(&final_name), &flags, retries, &fallbacks).await;
                    });

                    tasks.push(task);
                }
            } else {
                MacrosAlerts::ignore(&primary);
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

        let mut downloaded = false;

        if let (Some(start_index), Some(end_index)) = (start_index, end_index) {
            let downloads_content = &contents[start_index + "downloads ".len()..end_index];

            if !downloads_content.trim().starts_with("commands {") {
                FileUtils.create_path(&path);
                UI::section_header("downloads", "normal");
                self.block(&contents, downloads_content, &path, flags).await?;
                downloaded = true;
            }
        }

        let _ = Covers::new(&contents).get().await;
        let _ = Compress::new(&contents).get();
        let _ = Tasks.qr_codes(&contents, None).await;
        let _ = Math::new(&contents).render().await;
        Merge::new(&contents).get();
        Convert::new(&contents).run().await;

        Vars.get_open(&contents, flags.no_open_link).await;
        let _ = ReadMeBlock.render_var_and_save_file(&contents, flags).await;

        if downloaded {
            let _ = Checksum::new(Some(contents)).files();
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

        let mut downloaded = false;

        if let (Some(start_index), Some(end_index)) = (start_index, end_index) {
            let downloads_content = &contents[start_index + "downloads ".len()..end_index];

            if !downloads_content.trim().starts_with("commands {") {
                FileUtils.create_path(&path);
                UI::section_header("downloads", "normal");
                self.block(&contents, downloads_content, &path, flags).await?;
                downloaded = true;
            }
        }

        let _ = Compress::new(&contents).get();
        let _ = Covers::new(&contents).get().await;
        let _ = TasksRaw.qr_codes(&contents, None).await;
        let _ = Math::new(&contents).render().await;
        Merge::new(&contents).get();
        Convert::new(&contents).run().await;

        Vars.get_open(&contents, flags.no_open_link).await;
        let _ = ReadMeBlock.render_var_and_save_file(&contents, flags).await;

        if downloaded {
            let _ = Checksum::new(Some(contents)).files();
        }

        Ok(())
    }

}
