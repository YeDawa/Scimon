use std::{
    fs,
    path::Path,
    error::Error,
};

use crate::{
    syntax::vars::Vars,
    render::render::Render,
    generator::epub::Epub,
    system::latex::LaTex,
    system::markdown::Markdown,
    configs::package::Package,
    render::render_inject::RenderInject,

    ui::{
        ui_base::UI,
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },
};

pub struct Convert {
    contents: String,
}

impl Convert {

    pub fn new(contents: &str) -> Self {
        Self {
            contents: contents.to_string(),
        }
    }

    pub async fn run(&self) {
        let jobs = Vars.get_all_convert(&self.contents);

        if jobs.is_empty() {
            return;
        }

        UI::section_header("Converting", "normal");

        for (input, output) in jobs {
            match self.convert_one(&input, &output).await {
                Ok(()) => SuccessAlerts::converted(&input, &output),
                Err(e) => ErrorsAlerts::generic(&e.to_string()),
            }
        }
    }

    async fn convert_one(&self, input: &str, output: &str) -> Result<(), Box<dyn Error>> {
        if !Path::new(input).is_file() {
            return Err(format!("File not found: {}", input).into());
        }

        let from = Self::extension(input);
        let to = Self::extension(output);

        match (from.as_str(), to.as_str()) {
            ("md", "pdf")           => self.markdown_to_pdf(input, output).await,
            ("md", "html")          => self.markdown_to_html(input, output).await,
            ("md", "epub")          => self.markdown_to_epub(input, output),
            ("tex", "pdf")          => self.tex_to_pdf(input, output).await,
            ("html" | "htm", "pdf") => self.html_to_pdf(input, output).await,

            _ => Err(format!("Unsupported conversion: .{} -> .{}", from, to).into()),
        }
    }

    async fn markdown_to_pdf(&self, input: &str, output: &str) -> Result<(), Box<dyn Error>> {
        let document = self.markdown_document(input).await?;
        let pdf = Render.connect_to_browser(&document).await?;

        fs::write(output, pdf)?;
        Ok(())
    }

    async fn markdown_to_html(&self, input: &str, output: &str) -> Result<(), Box<dyn Error>> {
        let document = self.markdown_document(input).await?;

        fs::write(output, document)?;
        Ok(())
    }

    fn markdown_to_epub(&self, input: &str, output: &str) -> Result<(), Box<dyn Error>> {
        let markdown = fs::read_to_string(input)?;

        let title = Path::new(output)
            .file_stem()
            .map(|stem| stem.to_string_lossy().to_string())
            .unwrap_or_else(|| output.to_string());

        let author = Package.author();

        Epub.create(&markdown, &title, &author, output)?;
        Ok(())
    }

    async fn tex_to_pdf(&self, input: &str, output: &str) -> Result<(), Box<dyn Error>> {
        let tex = fs::read_to_string(input)?;
        let html = LaTex.render(&tex).await;
        let pdf = Render.connect_to_browser(&html).await?;

        fs::write(output, pdf)?;
        Ok(())
    }

    async fn html_to_pdf(&self, input: &str, output: &str) -> Result<(), Box<dyn Error>> {
        let html = fs::read_to_string(input)?;
        let pdf = Render.connect_to_browser(&html).await?;

        fs::write(output, pdf)?;
        Ok(())
    }

    // Renders a Markdown file into a full, styled HTML document — honoring the
    // list's `style` directive, like the rest of the renderers.
    async fn markdown_document(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let markdown = fs::read_to_string(input)?;
        let html_body = Markdown.append_extras_and_render(&markdown);

        RenderInject.html_content(&self.contents, html_body).await
    }

    fn extension(path: &str) -> String {
        Path::new(path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase()
    }

}
