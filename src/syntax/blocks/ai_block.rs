use regex::Regex;
use tokio::task;
use futures::future::join_all;

use std::{
    fs,
    error::Error,
};

use crate::{
    syntax::vars::Vars,
    utils::file::FileUtils,
    render::render::Render,
    system::markdown::Markdown,
    addons::openrouter::OpenRouter,
    render::render_inject::RenderInject,
    regexp::regex_blocks::BlocksRegExp,
    syntax::macro_handler::MacroHandler,

    ui::{
        ui_base::UI,
        macros_alerts::MacrosAlerts,
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },
};

struct AiEntry {
    prompt: String,
    file: String,
    model: Option<String>,
    pdf: bool,
}

pub struct AiBlock;

impl AiBlock {

    fn extract_block(&self, contents: &str) -> Option<String> {
        let start_pattern = Regex::new(BlocksRegExp::GET_AI_BLOCK).unwrap();
        let start_match = start_pattern.find(contents)?;

        let start_index = start_match.end();
        let mut end_index = start_index;
        let mut open_braces = 1;

        for (i, c) in contents[start_index..].char_indices() {
            match c {
                '{' => open_braces += 1,
                '}' => open_braces -= 1,
                _ => {}
            }

            if open_braces == 0 {
                end_index = start_index + i;
                break;
            }
        }

        if open_braces != 0 {
            eprintln!("Unmatched braces in the 'ai' block.");
            return None;
        }

        Some(contents[start_index..end_index].to_string())
    }

    fn parse_line(&self, line: &str) -> Option<AiEntry> {
        let pattern = Regex::new(BlocksRegExp::GET_AI_ENTRY).unwrap();
        let caps = pattern.captures(line.trim())?;

        let prompt = caps.name("prompt")?
            .as_str()
            .replace("\\\"", "\"")
            .trim()
            .to_string();

        let mut file = caps.name("file")?.as_str().trim().to_string();
        let model = caps.name("model").map(|m| m.as_str().trim().to_string());

        // The output extension decides the format: ".pdf" renders the generated
        // Markdown to a styled PDF, anything else is saved as a ".md" file.
        let lower = file.to_lowercase();
        let pdf = lower.ends_with(".pdf");

        if !pdf && !lower.ends_with(".md") {
            file.push_str(".md");
        }

        Some(AiEntry { prompt, file, model, pdf })
    }

    async fn save_pdf(list_contents: &str, output_path: &str, markdown: &str) -> Result<(), Box<dyn Error>> {
        let html_body = Markdown.append_extras_and_render(markdown);
        let document = RenderInject.html_content(list_contents, html_body).await?;
        let pdf_bytes = Render.connect_to_browser(&document).await?;

        fs::write(output_path, pdf_bytes)?;
        Ok(())
    }

    async fn process_entry(entry: AiEntry, path: String, list_contents: String) {
        let output_path = FileUtils
            .get_output_path(&path, &entry.file)
            .to_string_lossy()
            .to_string();

        // The (non-`Send`) error is turned into a `String` right after each
        // `await` so nothing non-`Send` is held across an await point — that
        // keeps the future `Send`, which `tokio::spawn` requires.
        let markdown = match OpenRouter::new(entry.model).generate(&entry.prompt).await {
            Ok(markdown) => markdown,
            Err(e) => {
                ErrorsAlerts::generic(&e.to_string());
                return;
            }
        };

        if entry.pdf {
            if let Err(e) = Self::save_pdf(&list_contents, &output_path, &markdown).await {
                ErrorsAlerts::generic(&e.to_string());
            } else {
                SuccessAlerts::generated_pdf(&output_path);
            }
        } else {
            FileUtils.write_file(&output_path, markdown);
            SuccessAlerts::ai_markdown(&output_path);
        }
    }

    pub async fn generate_and_save_files(&self, contents: &str) {
        let block = match self.extract_block(contents) {
            Some(block) => block,
            None => return,
        };

        let path = Vars.get_path(contents);
        FileUtils.create_path(&path);

        UI::section_header("ai", "normal");

        // Each entry is an independent network request (and optional PDF render),
        // so they are dispatched concurrently instead of one after another.
        let mut tasks = Vec::new();

        for line in block.lines() {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                continue;
            }

            if MacroHandler::handle_check_macro_line(trimmed, "ignore") {
                MacrosAlerts::ignore(trimmed);
                continue;
            }

            let entry = match self.parse_line(trimmed) {
                Some(entry) => entry,
                None => continue,
            };

            let path = path.clone();
            let list_contents = contents.to_string();
            tasks.push(task::spawn(Self::process_entry(entry, path, list_contents)));
        }

        join_all(tasks).await;
    }

}
