use regex::Regex;
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
    system::shutdown::Shutdown,
    system::providers::Providers,
    regexp::regex_blocks::BlocksRegExp,

    ui::{
        ui_base::UI,
        macros_alerts::MacrosAlerts,
    },

    cmd::{
        tasks::Tasks,
        compress::Compress,
    },

    syntax::{
        vars::Vars,
        ranges::Ranges,
        extended::Extended,
        blocks::ai_block::AiBlock,
        macro_handler::MacroHandler,
        blocks::readme_block::ReadMeBlock,
    },
    
    generator::{
        math::Math,
        merge::Merge,
        split::Split,
        rotate::Rotate,
        covers::Covers,
        convert::Convert,
        checksum::Checksum,
        watermark::Watermark,
    },
};

// Stops the step sequence early when a graceful shutdown has been requested.
macro_rules! stop_if_cancelled {
    () => {
        if Shutdown.cancelled() {
            return Ok(());
        }
    };
}

pub struct DownloadsBlock;

impl DownloadsBlock {
    
    async fn block(&self, contents: &str, downloads_content: &str, path: &str, flags: &Flags) -> Result<(), Box<dyn Error>> {
        let mut seen_urls = HashSet::new();
        let mut tasks = Vec::new();

        let only_mode = MacroHandler::any(downloads_content, "only");
        let group_open = Regex::new(BlocksRegExp::GET_GROUP_BLOCK).unwrap();

        // Nesting of `group "name" { ... }` blocks; files inside go to the
        // matching subdirectory of `path`.
        let mut groups: Vec<String> = Vec::new();

        for raw_line in downloads_content.lines() {
            // Stop queuing new downloads once a shutdown has been requested.
            if Shutdown.cancelled() {
                break;
            }

            let trimmed = raw_line.trim();

            if trimmed.starts_with("downloads {") {
                continue;
            }

            if let Some(caps) = group_open.captures(trimmed) {
                groups.push(caps.get(1).unwrap().as_str().to_string());
                FileUtils.create_path(&Self::group_path(path, &groups));
                continue;
            }

            if trimmed.starts_with('}') {
                // A group's closing brace pops the stack; the downloads block's
                // own closing brace (stack empty) ends the loop.
                if groups.is_empty() {
                    break;
                }

                groups.pop();
                continue;
            }

            if only_mode && !MacroHandler::handle_check_macro_line(raw_line, "only") {
                continue;
            }

            let group_path = Self::group_path(path, &groups);

            // A `{A..B}` range expands one line into several, one per value.
            for line in Ranges.expand_line(raw_line) {
                let parts: Vec<&str> = line.split('|').collect();
                let download_part = parts[0].trim();
                let pipe_parts: Vec<String> = parts[1..].iter().map(|s| s.trim().to_string()).collect();

                let candidates: Vec<String> = download_part
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

                if !MacroHandler::handle_check_macro_line(download_part, "ignore") {
                    let final_name = if let Some(custom_name) = Extended.rename_on_the_fly(download_part) {
                        custom_name
                    } else {
                        "".to_string()
                    };

                    if !primary.is_empty() && is_url(&primary) && primary.starts_with("http") {
                        let contents = contents.to_string();
                        let path = group_path.clone();
                        let flags = flags.clone();
                        let retries = MacroHandler::retry_count(download_part);
                        let fallbacks: Vec<String> = candidates[1..].to_vec();
                        let unzip = MacroHandler::handle_check_macro_line(download_part, "unzip")
                            || MacroHandler::handle_check_macro_line(download_part, "extract");

                        let task = task::spawn(async move {
                            if Shutdown.cancelled() {
                                return;
                            }

                            match Tasks.download(Some(&contents), &primary, &path, Some(&final_name), &flags, retries, &fallbacks, unzip).await {
                                Ok(file_path) => {
                                    if !file_path.is_empty() && !pipe_parts.is_empty() {
                                        if let Err(e) = Self::apply_pipes(&file_path, &pipe_parts) {
                                            crate::ui::errors_alerts::ErrorsAlerts::generic(&format!("Error applying pipes to {}: {}", file_path, e));
                                        }
                                    }
                                }
                                Err(_) => {}
                            }
                        });

                        tasks.push(task);
                    }
                } else {
                    MacrosAlerts::ignore(&primary);
                }
            }
        }

        join_all(tasks).await;
        Ok(())
    }

    fn apply_pipes(file_path: &str, pipe_parts: &[String]) -> Result<(), Box<dyn Error>> {
        let current_path = file_path.to_string();

        for pipe in pipe_parts {
            let trimmed_pipe = pipe.trim();
            if trimmed_pipe.is_empty() {
                continue;
            }

            if trimmed_pipe.starts_with("rotate ") {
                let angle_str = trimmed_pipe.strip_prefix("rotate ").unwrap().trim();
                let angle = angle_str.parse::<i64>()?;
                
                let rotate = Rotate::new("");
                rotate.rotate_one(&current_path, angle, &current_path)?;
            } else if trimmed_pipe.starts_with("watermark ") {
                let args = trimmed_pipe.strip_prefix("watermark ").unwrap().trim();
                let watermark = Watermark::new("");
                
                if args.starts_with("image ") {
                    let img_path = args.strip_prefix("image ").unwrap().trim().trim_matches('"').to_string();
                    watermark.image(&current_path, &img_path, &current_path)?;
                } else {
                    let text = args.trim_matches('"').to_string();
                    watermark.text(&current_path, &text, &current_path)?;
                }
            } else {
                return Err(format!("Unknown pipe command: {}", trimmed_pipe).into());
            }
        }

        Ok(())
    }

    // Joins the base path with the current group nesting, keeping a trailing
    // slash so downstream `path + filename` concatenation stays valid.
    fn group_path(path: &str, groups: &[String]) -> String {
        if groups.is_empty() {
            path.to_string()
        } else {
            format!("{}/{}/", path.trim_end_matches('/'), groups.join("/"))
        }
    }

    pub async fn read_lines<R>(&self, reader: R, flags: &Flags) -> Result<(), Box<dyn Error>> where R: BufRead {
        let contents = reader.lines().collect::<Result<Vec<_>, _>>()?.join("\n");
        let path = Vars.get_path(&contents);

        let start_index = match (contents.find("downloads {"), contents.find("downloads{")) {
            (Some(idx1), Some(idx2)) => Some(idx1.min(idx2)),
            (Some(idx), None) | (None, Some(idx)) => Some(idx),
            (None, None) => None,
        };

        let mut downloaded = false;
        let end_index = contents.rfind("}");

        if let (Some(start_index), Some(end_index)) = (start_index, end_index) {
            let downloads_content = &contents[start_index + "downloads ".len()..end_index];

            if !downloads_content.trim().starts_with("commands {") {
                FileUtils.create_path(&path);
                UI::section_header("downloads", "normal");
                self.block(&contents, downloads_content, &path, flags).await?;
                downloaded = true;
            }
        }

        stop_if_cancelled!();
        let _ = AiBlock.generate_and_save_files(&contents).await;
        stop_if_cancelled!();
        let _ = Merge::new(&contents).get();
        stop_if_cancelled!();
        let _ = Split::new(&contents).get();
        stop_if_cancelled!();
        let _ = Rotate::new(&contents).get();
        stop_if_cancelled!();
        let _ = Watermark::new(&contents).get();

        stop_if_cancelled!();
        let _ = Covers::new(&contents).get().await;
        stop_if_cancelled!();
        let _ = Compress::new(&contents).get();
        stop_if_cancelled!();
        let _ = Tasks.qr_codes(&contents, None).await;
        stop_if_cancelled!();
        let _ = Math::new(&contents).render().await;
        stop_if_cancelled!();
        let _ = Convert::new(&contents).run().await;

        stop_if_cancelled!();
        Vars.get_open(&contents, flags.no_open_link).await;
        let _ = ReadMeBlock.render_var_and_save_file(&contents, flags).await;

        if downloaded {
            let _ = Checksum::new(Some(contents)).files();
        }

        Ok(())
    }

}
