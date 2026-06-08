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
        let mut parser = Parser::new(&content);
        let mut labels = HashMap::new();
        let document_ast = parser.parse(false, &mut labels);

        let mut context = RenderContext::new();
        Nodes::pre_pass(&document_ast, &mut context);
        context.reset_counters();
        
        let html_body = Nodes::render(&document_ast, &mut context);
        Templates::latex(&html_body)
    }

    pub async fn create_pdf(&self, path: &str, url: &str, custom_name: Option<&str>) -> Result<(), Box<dyn Error>> {
        let content = Remote.content(&url).await?;
        let html = &self.render(&content);
        
        let original_name = FileNameRemote::new(url).get();
        let new_filename = if let Some(custom_name) = custom_name {
            FileUtils.replace_extension(custom_name, "pdf")
        } else {
            FileUtils.replace_extension(&original_name, "pdf")
        };

        let output_path = FileUtils.get_output_path(&path, &new_filename);
        Pdf.create_pdf(html, output_path, url).await?;
        SuccessAlerts::download_and_generated_pdf(&new_filename, url);
        Ok(())
    }

}