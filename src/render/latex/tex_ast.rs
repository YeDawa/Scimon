use crate::render::latex::{
    nodes::Nodes,
    bibtex::BibTextRender,
    context::RenderContext,
};

// ---------------------------------------------------------------------------
// Table cell — carries colspan, rowspan and column-spec styling
// ---------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct TableCell {
    pub content: Vec<LatexNode>,
    pub colspan: usize,
    pub rowspan: usize,
    pub align:   String,         // CSS text-align value
    pub width:   Option<String>, // CSS width value
    pub hline:   bool,           // border-top from \hline / \cline
}

impl TableCell {
    pub fn simple(content: Vec<LatexNode>) -> Self {
        TableCell { content, colspan: 1, rowspan: 1,
                    align: String::new(), width: None, hline: false }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LatexNode {
    // --- Inline text formatting ---
    Text(String),
    Bold(Vec<LatexNode>),
    Italic(Vec<LatexNode>),
    Underline(Vec<LatexNode>),
    Monospace(Vec<LatexNode>),
    SmallCaps(Vec<LatexNode>),
    Strikethrough(Vec<LatexNode>),

    // --- Font size ---
    FontSize(String, Vec<LatexNode>),

    // --- Spacing ---
    VSpace(String),
    HSpace(String),
    LineBreak,
    NewPage,
    HorizontalRule,

    // --- Document structure ---
    Part(String),
    Chapter(String),
    Section(String),
    Subsection(String),
    Subsubsection(String),
    Paragraph(String),

    /// Manual TOC entry: (toc-level name, display title)
    AddContentsLine(String, String),

    // --- Lists ---
    Itemize(Vec<Vec<LatexNode>>),
    Enumerate(Vec<Vec<LatexNode>>),
    Description(Vec<(String, Vec<LatexNode>)>),

    // --- Math ---
    MathInline(Vec<LatexNode>),
    MathDisplay(Vec<LatexNode>),
    /// Raw LaTeX passed directly to MathJax — $...$ and \(...\)
    RawMathInline(String),
    /// Raw LaTeX passed directly to MathJax — $$...$$ and \[...\]
    RawMathDisplay(String),
    Superscript(Vec<LatexNode>),
    Subscript(Vec<LatexNode>),
    Fraction { num: Vec<LatexNode>, den: Vec<LatexNode> },
    EquationBlock(Vec<LatexNode>),
    AlignBlock(Vec<LatexNode>),

    // --- References & labels ---
    Cite(String),
    CiteMultiple(Vec<String>),
    Bibliography(String),
    Label(String),
    Ref(String),
    PageRef(String),

    // --- Cross-document ---
    Footnote(Vec<LatexNode>),

    // --- Floats ---
    Image(String),
    Caption(String),
    Table(Vec<Vec<TableCell>>),

    /// Wrapper for \begin{table}...\end{table} — increments tab_num first,
    /// then renders children so \caption sees the correct counter.
    TableFloat(Vec<LatexNode>),

    /// Wrapper for \begin{figure}...\end{figure}
    FigureFloat(Vec<LatexNode>),
    
    /// Inline math matrix — open/close delimiters + grid of cells
    Matrix {
        open:  &'static str,
        close: &'static str,
        rows:  Vec<Vec<Vec<LatexNode>>>,
    },

    // --- Verbatim / Code ---
    CodeBlock(String),
    Mermaid(String),

    // --- Links ---
    Url(String),
    Href { url: String, text: Vec<LatexNode> },

    // --- Document metadata ---
    Title(Vec<LatexNode>),
    Author(Vec<LatexNode>),
    Date(Vec<LatexNode>),
    Abstract(Vec<LatexNode>),
    MakeTitle,
    TableOfContents,

    // --- fancyhdr ---
    /// Set one header slot. `pos` is "L", "C", or "R".
    FancyHeader { pos: String, nodes: Vec<LatexNode> },
    /// Set one footer slot. `pos` is "L", "C", or "R".
    FancyFooter { pos: String, nodes: Vec<LatexNode> },
    /// \fancyhf{} — clear all header/footer slots.
    FancyClear,
    /// \thepage — current page number placeholder.
    ThePage,
}

impl LatexNode {

    pub fn to_html(&self, ctx: &mut RenderContext) -> String {
        match self {
            // ----------------------------------------------------------------
            // Plain text
            // ----------------------------------------------------------------
            LatexNode::Text(t) => t.clone(),

            // ----------------------------------------------------------------
            // Inline formatting
            // ----------------------------------------------------------------
            LatexNode::Bold(nodes) =>
                format!("<strong>{}</strong>", Nodes::render(nodes, ctx)),

            LatexNode::Italic(nodes) =>
                format!("<em>{}</em>", Nodes::render(nodes, ctx)),

            LatexNode::Underline(nodes) =>
                format!("<u>{}</u>", Nodes::render(nodes, ctx)),

            LatexNode::Monospace(nodes) =>
                format!("<code>{}</code>", Nodes::render(nodes, ctx)),

            LatexNode::SmallCaps(nodes) =>
                format!("<span style=\"font-variant: small-caps;\">{}</span>", Nodes::render(nodes, ctx)),

            LatexNode::Strikethrough(nodes) =>
                format!("<s>{}</s>", Nodes::render(nodes, ctx)),

            // ----------------------------------------------------------------
            // Font size
            // ----------------------------------------------------------------
            LatexNode::FontSize(size, nodes) =>
                format!("<span class=\"font-{}\">{}</span>", size, Nodes::render(nodes, ctx)),

            // ----------------------------------------------------------------
            // Spacing & breaks
            // ----------------------------------------------------------------
            LatexNode::VSpace(size) =>
                format!("<div style=\"height: {}\"></div>", size),

            LatexNode::HSpace(size) =>
                format!("<span style=\"display: inline-block; width: {}\"></span>", size),

            LatexNode::LineBreak =>
                "<br/>".to_string(),

            LatexNode::NewPage =>
                "<div style=\"page-break-after: always;\"></div>".to_string(),

            LatexNode::HorizontalRule =>
                "<hr class=\"latex-hr\"/>".to_string(),

            // ----------------------------------------------------------------
            // Document structure
            // ----------------------------------------------------------------
            LatexNode::Part(title) => {
                ctx.part_num += 1;
                ctx.chap_num = 0;
                ctx.sec_num = 0;
                ctx.subsec_num = 0;
                let roman = Self::to_roman(ctx.part_num);
                ctx.toc.push((0, roman.clone(), title.clone()));

                format!(
                    "<div class=\"latex-part\" id=\"part-{}\"><span class=\"part-label\">Part {}</span><span class=\"part-title\">{}</span></div>",
                    ctx.part_num, roman, title
                )
            }

            LatexNode::AddContentsLine(level, title) => {
                let toc_level = match level.as_str() {
                    "part"          => 0,
                    "chapter"       => 1,
                    "section"       => 2,
                    "subsection"    => 3,
                    "subsubsection" => 4,
                    _               => 2,
                };
                let num = match toc_level {
                    0 => Self::to_roman(ctx.part_num + 1),
                    1 => ctx.chap_num.to_string(),
                    2 => ctx.sec_num.to_string(),
                    3 => format!("{}.{}", ctx.sec_num, ctx.subsec_num),
                    _ => String::new(),
                };
                ctx.toc.push((toc_level, num, title.clone()));
                String::new()
            }

            LatexNode::Chapter(title) => {
                ctx.chap_num += 1;
                ctx.sec_num = 0;
                ctx.subsec_num = 0;
                let num = ctx.chap_num.to_string();
                ctx.toc.push((1, num.clone(), title.clone()));

                format!("<h1 id=\"item-{}\">{} &nbsp;&nbsp; {}</h1>", num, num, title)
            }

            LatexNode::Section(title) => {
                ctx.sec_num += 1;
                ctx.subsec_num = 0;
                ctx.subsubsec_num = 0;
                let num = ctx.sec_num.to_string();
                ctx.toc.push((2, num.clone(), title.clone()));

                format!("<h2 id=\"item-{}\">{} &nbsp;&nbsp; {}</h2>", num, num, title)
            }

            LatexNode::Subsection(title) => {
                ctx.subsec_num += 1;
                ctx.subsubsec_num = 0;
                let num = format!("{}.{}", ctx.sec_num, ctx.subsec_num);
                ctx.toc.push((3, num.clone(), title.clone()));

                format!("<h3 id=\"item-{}\">{} &nbsp;&nbsp; {}</h3>", num, num, title)
            }

            LatexNode::Subsubsection(title) => {
                ctx.subsubsec_num += 1;
                let num = format!("{}.{}.{}", ctx.sec_num, ctx.subsec_num, ctx.subsubsec_num);
                ctx.toc.push((4, num.clone(), title.clone()));

                format!("<h4 id=\"item-{}\">{} &nbsp;&nbsp; {}</h4>", num, num, title)
            }

            LatexNode::Paragraph(title) =>
                format!("<p class=\"latex-paragraph\"><strong>{}</strong> ", title),

            // ----------------------------------------------------------------
            // Abstract
            // ----------------------------------------------------------------
            LatexNode::Abstract(nodes) =>
                format!(
                    "<div class=\"abstract\"><h3 class=\"abstract-title\">Abstract</h3><p>{}</p></div>",
                    Nodes::render(nodes, ctx)
                ),

            // ----------------------------------------------------------------
            // Lists
            // ----------------------------------------------------------------
            LatexNode::Itemize(items) => {
                let items_html: String = items
                    .iter()
                    .map(|item| format!("<li>{}</li>", Nodes::render(item, ctx)))
                    .collect();
                format!("<ul>{}</ul>", items_html)
            }

            LatexNode::Enumerate(items) => {
                let items_html: String = items
                    .iter()
                    .map(|item| format!("<li>{}</li>", Nodes::render(item, ctx)))
                    .collect();
                format!("<ol>{}</ol>", items_html)
            }

            LatexNode::Description(items) => {
                let items_html: String = items
                    .iter()
                    .map(|(term, desc)| {
                        format!("<dt><strong>{}</strong></dt><dd>{}</dd>", term, Nodes::render(desc, ctx))
                    })
                    .collect();
                format!("<dl>{}</dl>", items_html)
            }

            // ----------------------------------------------------------------
            // Math
            // ----------------------------------------------------------------

            // Raw LaTeX passed directly to MathJax for full rendering
            LatexNode::RawMathInline(raw) =>
                format!("\\({}\\)", raw),

            LatexNode::RawMathDisplay(raw) =>
                format!("<div class=\"math-block\">\\[{}\\]</div>", raw),

            #[allow(dead_code)]
            LatexNode::MathInline(nodes) =>
                format!("<span class=\"math-inline\">{}</span>", Nodes::render(nodes, ctx)),

            #[allow(dead_code)]
            LatexNode::MathDisplay(nodes) =>
                format!("<div class=\"math-display\">{}</div>", Nodes::render(nodes, ctx)),

            LatexNode::Superscript(nodes) =>
                format!("<sup>{}</sup>", Nodes::render(nodes, ctx)),

            LatexNode::Subscript(nodes) =>
                format!("<sub>{}</sub>", Nodes::render(nodes, ctx)),

            LatexNode::Fraction { num, den } =>
                format!(
                    "<span class=\"latex-frac\"><span class=\"frac-num\">{}</span><span class=\"frac-den\">{}</span></span>",
                    Nodes::render(num, ctx),
                    Nodes::render(den, ctx)
                ),

            LatexNode::EquationBlock(nodes) => {
                ctx.eq_num += 1;
                let num = ctx.eq_num.to_string();
                Self::register_inner_labels(nodes, &num, ctx);
                let inner = if let Some(LatexNode::RawMathDisplay(raw)) = nodes.first() {
                    format!("\\[{}\\]", raw)
                } else {
                    Nodes::render(nodes, ctx)
                };
                format!(
                    "<div class=\"math-block\" id=\"item-{}\">{} <span class=\"eq-number\">({})</span></div>",
                    num, inner, num
                )
            }

            LatexNode::AlignBlock(nodes) => {
                ctx.eq_num += 1;
                let num = ctx.eq_num.to_string();
                Self::register_inner_labels(nodes, &num, ctx);
                let inner = if let Some(LatexNode::RawMathDisplay(raw)) = nodes.first() {
                    raw.clone()
                } else {
                    Nodes::render(nodes, ctx)
                };
                format!(
                    "<div class=\"math-block math-align\" id=\"item-{}\">{} <span class=\"eq-number\">({})</span></div>",
                    num, inner, num
                )
            }

            // ----------------------------------------------------------------
            // Footnotes
            // ----------------------------------------------------------------
            LatexNode::Footnote(nodes) => {
                ctx.footnote_num += 1;
                let num = ctx.footnote_num;
                let content = Nodes::render(nodes, ctx);
                ctx.pending_footnotes.push((num, content));

                format!(
                    "<sup class=\"footnote-ref\"><a href=\"#fn-{}\" id=\"fnref-{}\">{}</a></sup>",
                    num, num, num
                )
            }

            // ----------------------------------------------------------------
            // References & labels
            // ----------------------------------------------------------------
            LatexNode::Label(name) =>
                format!("<span id=\"label-{}\"></span>", name),

            LatexNode::Ref(key) => {
                let num = ctx.labels.get(key).cloned().unwrap_or_else(|| "??".to_string());
                let prefix = if key.starts_with("ref-") { "ref" } else { "item" };
                format!("<a href=\"#{}-{}\" class=\"cross-ref\">{}</a>", prefix, num, num)
            }

            LatexNode::PageRef(label) => {
                let (href, data_ref) = if let Some(num) = ctx.labels.get(label) {
                    (format!("item-{}", num), format!("item-{}", num))
                } else {
                    (format!("label-{}", label), format!("label-{}", label))
                };

                format!(
                    "<a href=\"#{}\" class=\"cross-ref pageref\" data-ref=\"{}\">??</a>",
                    href, data_ref
                )
            }

            LatexNode::Cite(key) => {
                let number = ctx.register_citation(key);
                format!("<a href=\"#ref-{}\" class=\"cite\">[{}]</a>", key, number)
            }

            LatexNode::CiteMultiple(keys) => {
                let links: Vec<String> = keys
                    .iter()
                    .map(|key| {
                        let number = ctx.register_citation(key);
                        format!("<a href=\"#ref-{}\" class=\"cite\">{}</a>", key, number)
                    })
                    .collect();

                format!("[{}]", links.join(", "))
            }

            // ----------------------------------------------------------------
            // Bibliography
            // ----------------------------------------------------------------
            LatexNode::Bibliography(file) => {
                let mut html = String::from("<h2 class=\"bib-title\">References</h2><ol class=\"bibliography\">");

                let source = if file.starts_with("http://") || file.starts_with("https://") {
                    file.clone()
                } else {
                    format!("{}.bib", file)
                };

                let bib_content = BibTextRender::fetch_bibliography(&source).unwrap_or_default();
                ctx.bib_database = BibTextRender::parse_bibtex(&bib_content);

                // Render in citation order if we have one, otherwise alphabetical
                let keys_ordered: Vec<String> = if !ctx.citation_order.is_empty() {
                    ctx.citation_order.clone()
                } else {
                    ctx.bib_database.keys().cloned().collect()
                };

                for key in &keys_ordered {
                    if let Some(entry) = ctx.bib_database.get(key) {
                        html.push_str(&format!(
                            "<li id=\"ref-{}\">{}, <em>{}</em>, {}.</li>",
                            key, entry.author, entry.title, entry.year
                        ));
                    } else {
                        html.push_str(&format!(
                            "<li><strong style='color:red;'>Error: Ref '{}' not found!</strong></li>",
                            key
                        ));
                    }
                }

                html.push_str("</ol>");
                html
            }

            // ----------------------------------------------------------------
            // Links
            // ----------------------------------------------------------------
            LatexNode::Url(url) =>
                format!("<a href=\"{}\">{}</a>", url, url),

            LatexNode::Href { url, text } =>
                format!("<a href=\"{}\">{}</a>", url, Nodes::render(text, ctx)),

            // ----------------------------------------------------------------
            // TOC
            // ----------------------------------------------------------------
            LatexNode::TableOfContents =>
                // Filled in after the full render pass by LaTex::build_toc()
                "__TOC_PLACEHOLDER__".to_string(),

            // ----------------------------------------------------------------
            // Floats
            // ----------------------------------------------------------------
            // TableFloat increments tab_num FIRST so \caption inside sees the
            // correct number regardless of where it appears in the float body.
            LatexNode::TableFloat(children) => {
                ctx.tab_num += 1;
                ctx.last_counter = ctx.tab_num.to_string();
                ctx.in_float = true;
                let html = Nodes::render(children, ctx);
                ctx.in_float = false;
                html
            }

            LatexNode::FigureFloat(children) => {
                ctx.fig_num += 1;
                ctx.last_counter = ctx.fig_num.to_string();
                ctx.in_float = true;
                let html = Nodes::render(children, ctx);
                ctx.in_float = false;
                html
            }

            LatexNode::Caption(text) =>
                format!(
                    "<div class=\"caption\"><strong>Figure/Table {}:</strong> {}</div>",
                    ctx.last_counter, text
                ),

            LatexNode::Image(url) => {
                if !ctx.in_float {
                    ctx.fig_num += 1;
                    ctx.last_counter = ctx.fig_num.to_string();
                }

                let num = ctx.last_counter.clone();
                format!(
                    "<img src=\"{}\" class=\"latex-image\" id=\"item-{}\" alt=\"Figure {}\" />",
                    url, num, num
                )
            }

            LatexNode::Table(rows) => {
                if ctx.last_counter == "0" || !ctx.in_float {
                    ctx.tab_num += 1;
                    ctx.last_counter = ctx.tab_num.to_string();
                }

                let num = ctx.last_counter.clone();
                let mut html = format!(
                    "<table class=\"latex-table\" id=\"item-{}\"><tbody>\n", num
                );

                for row in rows {
                    html.push_str("  <tr>\n");
                    for cell in row {
                        let mut attrs = String::new();

                        if cell.colspan > 1 {
                            attrs.push_str(&format!(" colspan=\"{}\"", cell.colspan));
                        }
                        if cell.rowspan > 1 {
                            attrs.push_str(&format!(" rowspan=\"{}\"", cell.rowspan));
                        }

                        let mut style = Vec::new();
                        if !cell.align.is_empty() {
                            style.push(format!("text-align:{}", cell.align));
                        }
                        if let Some(w) = &cell.width {
                            style.push(format!("width:{}", w));
                        }
                        if cell.hline {
                            style.push("border-top:2px solid #2c3e50".to_string());
                        }
                        if !style.is_empty() {
                            attrs.push_str(&format!(" style=\"{}\"", style.join("; ")));
                        }

                        html.push_str(&format!(
                            "    <td{}>{}</td>\n",
                            attrs, Nodes::render(&cell.content, ctx)
                        ));
                    }
                    html.push_str("  </tr>\n");
                }

                html.push_str("</tbody></table>\n");
                html
            }

            // Inline math matrix — delimiters + inline-grid, all in one span
            LatexNode::Matrix { open, close, rows } => {
                if rows.is_empty() {
                    return format!("{}{}", open, close);
                }

                let col_count = rows.iter().map(|r| r.len()).max().unwrap_or(1);
                let mut cells_html = String::new();
                for row in rows {
                    for cell in row {
                        // inline-flex keeps sub/sup attached to their base glyph
                        // instead of becoming independent grid items (display:contents bug)
                        cells_html.push_str(&format!(
                            "<span class=\"matrix-cell\">{}</span>",
                            Nodes::render(cell, ctx)
                        ));
                    }

                    // Pad short rows so the grid stays rectangular
                    for _ in row.len()..col_count {
                        cells_html.push_str("<span class=\"matrix-cell\"></span>");
                    }
                }

                format!(
                    "<span class=\"latex-matrix-wrap\">\
                        <span class=\"matrix-delim\">{open}</span>\
                        <span class=\"latex-matrix\" \
                              style=\"grid-template-columns: repeat({col_count}, auto);\">\
                            {cells_html}\
                        </span>\
                        <span class=\"matrix-delim\">{close}</span>\
                    </span>"
                )
            }

            // ----------------------------------------------------------------
            // Code / Mermaid
            // ----------------------------------------------------------------
            LatexNode::CodeBlock(code) =>
                format!(
                    "<pre class=\"code-block\"><code>{}</code></pre>",
                    code.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
                ),

            LatexNode::Mermaid(raw_code) =>
                format!("<div class=\"mermaid\">\n{}\n</div>", raw_code),

            // ----------------------------------------------------------------
            // Document metadata
            // ----------------------------------------------------------------
            LatexNode::Title(t) => {
                ctx.doc_title = Nodes::render(t, ctx);
                String::new()
            }

            LatexNode::Author(a) => {
                ctx.doc_author = Nodes::render(a, ctx);
                String::new()
            }

            LatexNode::Date(d) => {
                ctx.doc_date = Nodes::render(d, ctx);
                String::new()
            }

            LatexNode::MakeTitle => {
                let date_html = if ctx.doc_date.is_empty() {
                    String::new()
                } else {
                    format!("<div class=\"date\">{}</div>", ctx.doc_date)
                };
                format!(
                    "<div class=\"title-block\">\n  <h1>{}</h1>\n  <div class=\"author\">{}</div>\n  {}</div>",
                    ctx.doc_title, ctx.doc_author, date_html
                )
            }

            // ----------------------------------------------------------------
            // fancyhdr
            // ----------------------------------------------------------------
            LatexNode::FancyHeader { pos, nodes } => {
                let html = Nodes::render(nodes, ctx);
                ctx.has_fancy = true;
                for slot in pos.split(',').map(|s| s.trim()) {
                    match slot.chars().next().map(|c| c.to_ascii_uppercase()) {
                        Some('L') => ctx.header_left   = html.clone(),
                        Some('C') => ctx.header_center = html.clone(),
                        Some('R') => ctx.header_right  = html.clone(),
                        _ => {}
                    }
                }
                String::new()
            }

            LatexNode::FancyFooter { pos, nodes } => {
                let html = Nodes::render(nodes, ctx);
                ctx.has_fancy = true;
                for slot in pos.split(',').map(|s| s.trim()) {
                    match slot.chars().next().map(|c| c.to_ascii_uppercase()) {
                        Some('L') => ctx.footer_left   = html.clone(),
                        Some('C') => ctx.footer_center = html.clone(),
                        Some('R') => ctx.footer_right  = html.clone(),
                        _ => {}
                    }
                }
                String::new()
            }

            LatexNode::FancyClear => {
                ctx.header_left   = String::new();
                ctx.header_center = String::new();
                ctx.header_right  = String::new();
                ctx.footer_left   = String::new();
                ctx.footer_center = String::new();
                ctx.footer_right  = String::new();
                String::new()
            }

            LatexNode::ThePage =>
                "<span class=\"thepage\" aria-label=\"page number\"></span>".to_string(),
        }
    }

    fn register_inner_labels(nodes: &[LatexNode], value: &str, ctx: &mut RenderContext) {
        for node in nodes {
            if let LatexNode::Label(name) = node {
                ctx.labels.insert(name.clone(), value.to_string());
            }
        }
    }

    fn to_roman(n: usize) -> String {
        const VALS: &[(usize, &str)] = &[
            (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
            (100,  "C"), (90,  "XC"), (50,  "L"), (40,  "XL"),
            (10,   "X"), (9,   "IX"), (5,   "V"), (4,   "IV"),
            (1,    "I"),
        ];
        let mut n = n;
        let mut result = String::new();
        for &(val, sym) in VALS {
            while n >= val {
                result.push_str(sym);
                n -= val;
            }
        }
        result
    }

}