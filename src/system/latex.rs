use std::{
    error::Error,
    collections::HashMap,
};

use crate::{
    system::pdf::Pdf,
    templates::latex::TemplateLaTex, 
    ui::success_alerts::SuccessAlerts,

    utils::{
        remote::Remote,
        file::FileUtils, 
        file_name_remote::FileNameRemote, 
    },
    
    render::{
        render_inject_files::RenderInjectFiles,

        latex::{
            nodes::Nodes,
            parser::Parser,
            tex_ast::LatexNode,
            bibtex::BibTextRender,

            context::{
                AcronymInfo,
                RenderContext,
            },
        },
    },
};

pub struct LaTex;

impl LaTex {

    pub async fn render(&self, content: &str) -> String {
        let mut parser = Parser::new(content);
        let mut labels = HashMap::new();

        let force_active = !content.contains("\\begin{document}");
        let document_ast = parser.parse(force_active, &mut labels);

        let mut context = RenderContext::new(labels);
        self.prescan(&document_ast, &mut context);
        let mut html_body = Nodes::render(&document_ast, &mut context);

        let toc_html = self.build_toc(&context);
        html_body = html_body.replace("__TOC_PLACEHOLDER__", &toc_html);

        let footnotes = context.flush_footnotes();
        if !footnotes.is_empty() { html_body.push_str(&footnotes); }

        let header_html = if context.has_fancy {
            self.build_header_footer(&context)
        } else {
            String::new()
        };

        let css_style = RenderInjectFiles.latex_css_style().await;
        let js_script = RenderInjectFiles.latex_js_script().await;
        TemplateLaTex.base(&html_body, &header_html, &css_style, &js_script)
    }

    /// Load bibliography databases, the citation style and acronym
    /// definitions before the body renders, so references resolve even when
    /// they are declared after their first use.
    fn prescan(&self, ast: &[LatexNode], ctx: &mut RenderContext) {
        for node in ast {
            match node {
                LatexNode::BibStyleSet(style) => ctx.bib_style = *style,

                LatexNode::BibResource(file) =>
                    ctx.bib_database.extend(BibTextRender::load(file)),

                LatexNode::Bibliography { file, .. } if !file.is_empty() =>
                    ctx.bib_database.extend(BibTextRender::load(file)),

                LatexNode::AcronymDef { label, short, long, short_plural, long_plural } => {
                    ctx.acronyms.insert(label.clone(), AcronymInfo {
                        short:        short.clone(),
                        long:         long.clone(),
                        short_plural: short_plural.clone(),
                        long_plural:  long_plural.clone(),
                    });
                }

                _ => {}
            }
        }
    }

    fn build_toc(&self, ctx: &RenderContext) -> String {
        if ctx.toc.is_empty() {
            return String::new();
        }
        
        let mut html = String::from("<div class=\"toc\"><h2>Contents</h2><ul>");
        for (level, num, title) in &ctx.toc {
            let (indent, href) = match level {
                0 => ("",                   format!("part-{}", num)),      // \part
                1 => ("",                   format!("item-{}", num)),      // \chapter
                2 => ("margin-left: 25px;", format!("item-{}", num)),      // \section
                3 => ("margin-left: 50px;", format!("item-{}", num)),      // \subsection
                4 => ("margin-left: 75px;", format!("item-{}", num)),      // \subsubsection
                _ => ("margin-left: 25px;", format!("item-{}", num)),
            };

            let label = if num.is_empty() {
                title.clone()
            } else {
                format!("<strong>{}</strong> {}", num, title)
            };

            html.push_str(&format!(
                "<li style=\"{}\"><a href=\"#{}\">{}</a></li>",
                indent, href, label
            ));
        }

        html.push_str("</ul></div>");
        html
    }

    fn build_header_footer(&self, ctx: &RenderContext) -> String {
        let any_header = !ctx.header_left.is_empty()
            || !ctx.header_center.is_empty()
            || !ctx.header_right.is_empty();

        let any_footer = !ctx.footer_left.is_empty()
            || !ctx.footer_center.is_empty()
            || !ctx.footer_right.is_empty();

        let mut html = String::new();

        if any_header {
            html.push_str(&format!(
                "<div class=\"page-header\" role=\"banner\">\
                    <span class=\"hf-left\">{}</span>\
                    <span class=\"hf-center\">{}</span>\
                    <span class=\"hf-right\">{}</span>\
                </div>",
                ctx.header_left, ctx.header_center, ctx.header_right
            ));
        }

        if any_footer {
            html.push_str(&format!(
                "<div class=\"page-footer\" role=\"contentinfo\">\
                    <span class=\"hf-left\">{}</span>\
                    <span class=\"hf-center\">{}</span>\
                    <span class=\"hf-right\">{}</span>\
                </div>",
                ctx.footer_left, ctx.footer_center, ctx.footer_right
            ));
        }

        html
    }

    pub async fn create_pdf(&self, path: &str, url: &str, custom_name: Option<&str>) -> Result<(), Box<dyn Error>> {
        let content = Remote.content(url).await?;
        let html = self.render(&content).await;

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