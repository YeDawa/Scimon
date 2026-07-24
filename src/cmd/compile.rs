use is_url::is_url;

use std::{
    path::Path,
    error::Error,

    fs::{
        write,
        read_to_string,
    },
};

use crate::{
    system::latex::LaTex,
    generator::epub::Epub,
    consts::global::Global,
    system::markdown::Markdown,

    ui::{
        ui_base::UI,
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },

    render::{
        render::Render,
        render_inject::RenderInject,
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

    fn wants_epub(&self) -> bool {
        self.output
            .as_ref()
            .map(|output| output.to_lowercase().ends_with(".epub"))
            .unwrap_or(false)
    }

    pub async fn markdown(&self) -> Result<(), Box<dyn Error>> {
        UI::header();
        UI::section_header("Markdown Compiler", "info");

        let content = self.read_content().await?;

        if self.wants_epub() {
            let output_name = self.output.clone().unwrap_or_default();

            let title = Path::new(&output_name)
                .file_stem()
                .map(|stem| stem.to_string_lossy().to_string())
                .unwrap_or_else(|| output_name.clone());

            Epub.create(&content, &title, Global::APP_NAME, &output_name)?;
            SuccessAlerts::generated_epub(&output_name);

            return Ok(());
        }

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
