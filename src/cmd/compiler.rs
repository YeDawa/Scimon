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

    ui::{
        ui_base::UI,
        success_alerts::SuccessAlerts,
    },

    utils::{
        remote::Remote,
        file::FileUtils,
        file_name_remote::FileNameRemote,
    },
};

pub struct Compiler {
    file: String,
    output: Option<String>,
}

impl Compiler {

    pub fn new(file: &str, output: Option<String>) -> Self {
        Self {
            file: file.to_string(),
            output,
        }
    }

    pub async fn latex(&self) -> Result<(), Box<dyn Error>> {
        UI::header();
        UI::section_header("LaTex Compiler", "info");

        let content = if is_url(&self.file) {
            Remote.content(&self.file).await?
        } else {
            read_to_string(&self.file)?
        };

        let output_name = if let Some(output) = &self.output {
            FileUtils.replace_extension(output, "pdf")
        } else if is_url(&self.file) {
            FileUtils.replace_extension(
                &FileNameRemote::new(&self.file).get(), "pdf"
            )
        } else {
            FileUtils.replace_extension(&self.file, "pdf")
        };

        let html = LaTex.render(&content).await;
        let pdf_contents = Render.connect_to_browser(&html).await?;

        write(&output_name, pdf_contents)?;
        SuccessAlerts::generated_pdf(&output_name);

        Ok(())
    }

}
