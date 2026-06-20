extern crate open;

use std::{
    env,
    path::Path,
    error::Error,
};

use pulldown_cmark::{
    html,
    Parser,
    Options,
};

use crate::{
    system::pdf::Pdf,
    syntax::vars::Vars,
    consts::global::Global,
    generator::epub::Epub,
    configs::settings::Settings,
    generator::file_name::FileName,
    ui::success_alerts::SuccessAlerts,

    utils::{
        remote::Remote,
        file::FileUtils,
        file_name_remote::FileNameRemote,
    },

    render::{
        render_io::RenderIO,
        render_inject::RenderInject,
    },
};

pub struct Markdown;

impl Markdown {

    pub fn open_file(&self, path: &str, no_open_link: bool) {
        if !no_open_link {
            let full_path = env::current_dir().expect(
                ""
            ).join(path).to_str().unwrap().replace(
                "\\", "/"
            );

            let url_file = &format!(
                "file://{}", full_path
            );

            let _ = open::that(url_file);
        }
    }

    pub fn get_filename_rendered(&self, file: &str) -> String {
        let filename = if Settings.get("render_markdown.overwrite", "BOOLEAN") == true {
            ".html".to_string()
        } else {
            FileName::new(16, "html").gen()
        };

        RenderIO.get_file_path(file).replace(".html", &filename)
    }

    pub fn append_extras_and_render(&self, markdown: &str) -> String {
        let parser = Parser::new_ext(markdown, Options::all());
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        format!("<div class='markdown-content'>{}</div>", html_output)
    }

    pub async fn render(&self, url: &str) -> Result<String, Box<dyn Error>> {
        let markdown_content = Remote.content(url).await?;
    
        let options = Options::empty();
        let parser = Parser::new_ext(&markdown_content, options);

        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
    
        Ok(html_output)
    }

    pub async fn create(&self, contents: &str, url: &str, path: &str, custom_name: Option<&str>) -> Result<(), Box<dyn Error>> {
        if Remote.check_content_type(url, "text/markdown").await? || url.contains(".md") {
            let wants_epub = custom_name
                .map(|name| name.to_lowercase().ends_with(".epub"))
                .unwrap_or(false);

            if wants_epub {
                let name = custom_name.unwrap();
                let output_path = FileUtils.get_output_path(path, name)
                    .to_string_lossy()
                    .to_string();

                let title = Path::new(name)
                    .file_stem()
                    .map(|stem| stem.to_string_lossy().to_string())
                    .unwrap_or_else(|| name.to_string());

                let author = Vars.get_metadata(contents, "author")
                    .unwrap_or_else(|| Global::APP_NAME.to_string());

                let markdown_content = Remote.content(url).await?;
                Epub.create(&markdown_content, &title, &author, &output_path)?;

                SuccessAlerts::generated_epub(&output_path);
            } else {
                let html_content = self.render(url).await?;
                let content = RenderInject.html_content(contents, html_content).await?;

                let original_name = FileNameRemote::new(url).get();
                let new_filename = FileUtils.replace_extension(&original_name, "pdf");
                let output_path = FileUtils.get_output_path(path, &new_filename);

                Pdf.create_pdf(&content, output_path, url).await?;
                SuccessAlerts::download_and_generated_pdf(&new_filename, url);
            }
        }

        Ok(())
    }

}
