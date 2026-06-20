use is_url::is_url;

use std::{
    error::Error,

    fs::{
        write,
        read_to_string,
    },
};

use crate::{
    system::latex::LaTex,
    render::render::Render,
    system::markdown::Markdown,
    render::render_inject::RenderInject,

    ui::{
        ui_base::UI,
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },

    utils::{
        remote::Remote,
        file::FileUtils,
        file_name_remote::FileNameRemote,
    },
};

pub struct Compile {
    file: String,
    output: Option<String>,
}

impl Compile {

    pub fn new(file: &str, output: Option<String>) -> Self {
        Self {
            file: file.to_string(),
            output,
        }
    }

    async fn read_content(&self) -> Result<String, Box<dyn Error>> {
        if is_url(&self.file) {
            Ok(Remote.content(&self.file).await?)
        } else {
            Ok(read_to_string(&self.file)?)
        }
    }

    fn output_name(&self) -> String {
        if let Some(output) = &self.output {
            FileUtils.replace_extension(output, "pdf")
        } else if is_url(&self.file) {
            FileUtils.replace_extension(
                &FileNameRemote::new(&self.file).get(), "pdf"
            )
        } else {
            FileUtils.replace_extension(&self.file, "pdf")
        }
    }

    pub async fn latex(&self) -> Result<(), Box<dyn Error>> {
        UI::header();
        UI::section_header("LaTex Compiler", "info");

        let content = self.read_content().await?;
        let output_name = self.output_name();

        let html = LaTex.render(&content).await;
        let pdf_contents = Render.connect_to_browser(&html).await?;

        write(&output_name, pdf_contents)?;
        SuccessAlerts::generated_pdf(&output_name);

        Ok(())
    }

    pub async fn markdown(&self) -> Result<(), Box<dyn Error>> {
        UI::header();
        UI::section_header("Markdown Compiler", "info");

        let content = self.read_content().await?;
        let output_name = self.output_name();

        let html_body = Markdown.append_extras_and_render(&content);
        let html = RenderInject.html_content("", html_body).await?;
        let pdf_contents = Render.connect_to_browser(&html).await?;

        write(&output_name, pdf_contents)?;
        SuccessAlerts::generated_pdf(&output_name);

        Ok(())
    }

    pub async fn compile(&self) -> Result<(), Box<dyn Error>> {
        match self.file.split('.').next_back().unwrap_or_default() {
            "tex" => {
                if let Err(err) = &self.latex().await {
                    ErrorsAlerts::generic(&err.to_string());
                }
            },

            "md" | "markdown" => {
                if let Err(err) = &self.markdown().await {
                    ErrorsAlerts::generic(&err.to_string());
                }
            },

            _ => ErrorsAlerts::unsupported_file_type(),
        };

        Ok(())
    }

}
