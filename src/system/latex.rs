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
            macros::MacroDef,
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
        let force_active = !content.contains("\\begin{document}");

        // Parsing recurses one full parse() frame per nesting level (macros,
        // environments, arguments), which overflows the default 1 MB stack
        // on deep documents — give it a dedicated thread with room to spare.
        let source = content.to_string();
        let (document_ast, labels, user_macros) = std::thread::Builder::new()
            .name(String::from("latex-parse"))
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                let mut labels = HashMap::new();
                let mut parser = Parser::new(&source);
                let ast = parser.parse(force_active, &mut labels);
                (ast, labels, parser.macros)
            })
            .ok()
            .and_then(|handle| handle.join().ok())
            .unwrap_or_default();

        let mut context = RenderContext::new(labels);
        self.prescan(&document_ast, &mut context);
        let mut html_body = Nodes::render(&document_ast, &mut context);

        // Teach the user's macros to MathJax: raw math reaches the browser
        // unexpanded, so \newcommand definitions must exist there too. The
        // hidden block is typeset first and its definitions are document-wide.
        let preamble = Self::mathjax_preamble(&user_macros);
        if !preamble.is_empty() {
            html_body = format!("{}{}", preamble, html_body);
        }

        let toc_html = self.build_toc(&context);
        html_body = html_body.replace("__TOC_PLACEHOLDER__", &toc_html);

        let footnotes = context.flush_footnotes();
        if !footnotes.is_empty() { html_body.push_str(&footnotes); }

        let header_html = if context.has_fancy {
            self.build_header_footer(&context)
        } else {
            String::new()
        };

        let css_style = format!(
            "{}\n{}",
            RenderInjectFiles.latex_css_style().await,
            Self::PAGINATION_CSS,
        );
        let js_script = RenderInjectFiles.latex_js_script().await;
        TemplateLaTex.base(&html_body, &header_html, &css_style, &js_script)
    }

    /// Print-pagination rules layered on top of the remote stylesheet:
    /// headings keep their following content, blocks that must not split
    /// stay whole, and paragraphs avoid widows/orphans — approximating
    /// LaTeX's page-breaking decisions.
    const PAGINATION_CSS: &'static str = "\
        .document-container p { orphans: 3; widows: 3; }\n\
        h1, h2, h3, h4, .title-block, .bib-title, .acronym-title {\n\
            break-after: avoid-page; page-break-after: avoid;\n\
        }\n\
        .latex-table, .latex-theorem, .latex-tikz, .latex-pgfplot,\n\
        .latex-image, .caption, pre, blockquote, .footnote-list li {\n\
            break-inside: avoid; page-break-inside: avoid;\n\
        }\n";

    /// Hidden math block re-declaring every user macro for MathJax.
    /// Environment bodies (env@...) are parser-internal and skipped.
    fn mathjax_preamble(user_macros: &HashMap<String, MacroDef>) -> String {
        let mut definitions: Vec<String> = user_macros.iter()
            .filter(|(name, _)| !name.contains('@'))
            .map(|(name, def)| {
                def.to_tex(name)
                    .replace('<', "\\lt ")
                    .replace('>', "\\gt ")
            })
            .collect();

        if definitions.is_empty() {
            return String::new();
        }
        definitions.sort();

        format!(
            "<div class=\"mathjax-preamble\" style=\"display: none;\">\\({}\\)</div>",
            definitions.join(" "),
        )
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