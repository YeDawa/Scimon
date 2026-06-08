use std::{
    error::Error,
    collections::HashMap,
};

use crate::{
    system::pdf::Pdf,
    generator::templates::Templates,
    ui::success_alerts::SuccessAlerts,

    utils::{
        remote::Remote,
        file::FileUtils,
        file_name_remote::FileNameRemote,
    },

    render::latex::{
        nodes::Nodes,
        parser::Parser,
        context::RenderContext,
    }
};

pub struct LaTex;

impl LaTex {

    pub fn render(&self, content: &str) -> String {
        let mut labels = HashMap::new();
        let document_ast = Parser::new(content).parse(false, &mut labels);

        let mut context = RenderContext::new(labels);
        Nodes::pre_pass(&document_ast, &mut context);
        // context.reset_counters();

        let mut html_body = Nodes::render(&document_ast, &mut context);

        let footnotes = context.flush_footnotes();
        if !footnotes.is_empty() {
            html_body.push_str(&footnotes);
        }

        Templates::latex(&html_body)
    }

    pub async fn create_pdf(&self, path: &str, url: &str, custom_name: Option<&str>) -> Result<(), Box<dyn Error>> {
        let content = Remote.content(url).await?;
        let html = self.render(&content);

        let original_name = FileNameRemote::new(url).get();
        let new_filename = if let Some(name) = custom_name {
            FileUtils.replace_extension(name, "pdf")
        } else {
            FileUtils.replace_extension(&original_name, "pdf")
        };

        let output_path = FileUtils.get_output_path(path, &new_filename);
        Pdf.create_pdf(&html, output_path, url).await?;
        SuccessAlerts::download_and_generated_pdf(&new_filename, url);
        Ok(())
    }

}