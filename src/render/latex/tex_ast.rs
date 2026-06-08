use std::fs;

use crate::render::latex::{
    nodes::Nodes,
    bibtex::BibTextRender,
    context::RenderContext,
};

#[derive(Debug, Clone)]
pub enum LatexNode {
    Text(String),
    Bold(Vec<LatexNode>),
    Italic(Vec<LatexNode>),

    Section(String),
    Subsection(String),
    Itemize(Vec<Vec<LatexNode>>),
    Enumerate(Vec<Vec<LatexNode>>),
    MathInline(Vec<LatexNode>),
    Superscript(Vec<LatexNode>),
    Subscript(Vec<LatexNode>),
    Fraction { num: Vec<LatexNode>, den: Vec<LatexNode> },
    Cite(String),
    Bibliography(String),
    Mermaid(String),
    Table(Vec<Vec<Vec<LatexNode>>>),
    
    Title(Vec<LatexNode>),
    Author(Vec<LatexNode>),
    
    MakeTitle,
    Image(String),
    CodeBlock(String),
    EquationBlock(Vec<LatexNode>),

    Underline(Vec<LatexNode>),
    Monospace(Vec<LatexNode>),
    SmallCaps(Vec<LatexNode>),

    FontSize(String, Vec<LatexNode>),
    VSpace(String),

    Url(String),

    Href {
        url: String,
        text: Vec<LatexNode>
    },
    
    Label(String),
    Ref(String),
    TableOfContents,
    Caption(String),
}

impl LatexNode {

    pub fn to_html(&self, ctx: &mut RenderContext) -> String {
        match self {
            LatexNode::Text(t) => t.clone(),
            LatexNode::Bold(nodes) => format!("<strong>{}</strong>", Nodes::render(nodes, ctx)),
            LatexNode::Italic(nodes) => format!("<em>{}</em>", Nodes::render(nodes, ctx)),
            
            LatexNode::Underline(nodes) => format!("<u>{}</u>", Nodes::render(nodes, ctx)),
            LatexNode::Monospace(nodes) => format!("<code>{}</code>", Nodes::render(nodes, ctx)),
            LatexNode::SmallCaps(nodes) => format!("<span style=\"font-variant: small-caps;\">{}</span>", Nodes::render(nodes, ctx)),
            
            // Font size
            LatexNode::FontSize(size, nodes) => format!("<span style=\"font-size: {}\">{}</span>", size, Nodes::render(nodes, ctx)),
            
            // Vertical space
            LatexNode::VSpace(size) => format!("<div style=\"height: {}\"></div>", size),
            
            // Links
            LatexNode::Url(url) => format!("<a href=\"{}\">{}</a>", url, url),
            LatexNode::Href { url, text } => format!("<a href=\"{}\">{}</a>", url, Nodes::render(text, ctx)),
            
            // Headings with IDs for anchors
            LatexNode::Section(title) => {
                ctx.sec_num += 1; ctx.subsec_num = 0;
                let num = format!("{}", ctx.sec_num);
                format!("<h2 id=\"item-{}\">{} &nbsp;&nbsp; {}</h2>", num, num, title)
            }

            // Subsection with IDs for anchors
            LatexNode::Subsection(title) => {
                ctx.subsec_num += 1;
                let num = format!("{}.{}", ctx.sec_num, ctx.subsec_num);
                format!("<h3 id=\"item-{}\">{} &nbsp;&nbsp; {}</h3>", num, num, title)
            }
            
            // Lists
            LatexNode::Itemize(items) => {
                let items_html: String = items.iter().map(|item| format!("<li>{}</li>", Nodes::render(item, ctx))).collect();
                format!("<ul>{}</ul>", items_html)
            }
            LatexNode::Enumerate(items) => {
                let items_html: String = items.iter().map(|item| format!("<li>{}</li>", Nodes::render(item, ctx))).collect();
                format!("<ol>{}</ol>", items_html)
            }
            
            // Math
            LatexNode::MathInline(nodes) => format!("<span class=\"math-inline\">{}</span>", Nodes::render(nodes, ctx)),
            LatexNode::Superscript(nodes) => format!("<sup>{}</sup>", Nodes::render(nodes, ctx)),
            LatexNode::Subscript(nodes) => format!("<sub>{}</sub>", Nodes::render(nodes, ctx)),
            LatexNode::Fraction { num, den } => format!("<span class=\"latex-frac\"><span class=\"frac-num\">{}</span><span class=\"frac-den\">{}</span></span>", Nodes::render(num, ctx), Nodes::render(den, ctx)),
            LatexNode::EquationBlock(nodes) => {
                ctx.eq_num += 1;
                let num = format!("{}", ctx.eq_num);
                format!("<div class=\"math-block\" id=\"item-{}\">{} <span class=\"eq-number\">({})</span></div>", num, Nodes::render(nodes, ctx), num)
            }
            
            // Refs & TOC
            LatexNode::Label(_) => String::new(), // Processed in pre-pass

            // Cross-references
            LatexNode::Ref(key) => {
                let num = ctx.labels.get(key).cloned().unwrap_or_else(|| "??".to_string());
                format!("<a href=\"#item-{}\" class=\"cross-ref\">{}</a>", num, num)
            }

            // Table of Contents
            LatexNode::TableOfContents => {
                let mut html = String::from("<div class=\"toc\"><h2>Summary</h2><ul>");
                
                for (level, num, title) in &ctx.toc {
                    let indent = if *level == 2 { "margin-left: 25px;" } else { "" };
                    html.push_str(&format!("<li style=\"{}\"><a href=\"#item-{}\"><strong>{}</strong> {}</a></li>", indent, num, num, title));
                }
                
                html.push_str("</ul></div>");
                html
            }
            
            // Captions
            LatexNode::Caption(text) => {
                format!("<div class=\"caption\"><strong>Figure/Table {}:</strong> {}</div>", ctx.last_counter, text)
            }
            
            // External blocks
            LatexNode::Cite(key) => {
                let number = ctx.register_citation(key);
                format!("<a href=\"#ref-{}\" class=\"cite\">[{}]</a>", key, number)
            }

            // Bibliography
            LatexNode::Bibliography(file) => {
                let mut html = String::from("<h2 class=\"bib-title\">References</h2><ol class=\"bibliography\">");
                let bib_content = fs::read_to_string(format!("{}.bib", file)).unwrap_or_default();
                ctx.bib_database = BibTextRender::parse_bibtex(&bib_content);

                for key in &ctx.used_citations {
                    if let Some(entry) = ctx.bib_database.get(key) { html.push_str(&format!("<li id=\"ref-{}\">{}, <em>{}</em>, {}.</li>", key, entry.author, entry.title, entry.year)); } 
                    else { html.push_str(&format!("<li><strong style='color:red;'>Error: Ref '{}' not found!</strong></li>", key)); }
                }

                html.push_str("</ol>"); html
            }

            // Mermaid diagrams
            LatexNode::Mermaid(raw_code) => format!("<div class=\"mermaid\">\n{}\n</div>", raw_code),

            // Images
            LatexNode::Image(url) => {
                ctx.fig_num += 1;
                let num = format!("{}", ctx.fig_num);
                ctx.last_counter = num.clone();
                format!("<img src=\"{}\" class=\"latex-image\" id=\"item-{}\" alt=\"Figura {}\" />", url, num, num)
            }

            // Code blocks
            LatexNode::CodeBlock(code) => format!("<pre class=\"code-block\"><code>{}</code></pre>", code.replace('<', "&lt;").replace('>', "&gt;")),
            
            // Metadata & Tables
            LatexNode::Title(t) => { ctx.doc_title = Nodes::render(t, ctx); String::new() }
            LatexNode::Author(a) => { ctx.doc_author = Nodes::render(a, ctx); String::new() }
            LatexNode::MakeTitle => format!("<div class=\"title-block\">\n  <h1>{}</h1>\n  <div class=\"author\">{}</div>\n</div>", ctx.doc_title, ctx.doc_author),
            
            // Tables
            LatexNode::Table(rows) => {
                ctx.tab_num += 1;
                let num = format!("{}", ctx.tab_num);
                ctx.last_counter = num.clone();
                let mut html = format!("<table class=\"latex-table\" id=\"item-{}\"><tbody>\n", num);
                
                for row in rows {
                    html.push_str("  <tr>\n");
                    for cell in row { html.push_str("    <td>"); html.push_str(&Nodes::render(cell, ctx)); html.push_str("</td>\n"); }
                    html.push_str("  </tr>\n");
                }
                
                html.push_str("</tbody></table>\n");
                html
            }
        }
    }

}