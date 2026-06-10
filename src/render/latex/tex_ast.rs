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
    /// Apply a \setlength to a document-scoped CSS variable
    SetLength { param: String, value: String },
    /// Invisible box with same dimensions as content (all axes)
    Phantom(Vec<LatexNode>),
    /// Invisible box with same width as content (zero height)
    HPhantom(Vec<LatexNode>),
    /// Invisible box with same height as content (zero width)
    VPhantom(Vec<LatexNode>),
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
    /// \nocite{*} or \nocite{key,key2} — include in bibliography without inline cite
    NoCite(Vec<String>),
    Bibliography(String),
    /// \begin{thebibliography}{widest-label}...\end{thebibliography} inline bib
    TheBibliography(Vec<(String, Vec<LatexNode>)>),
    Label(String),
    Ref(String),
    PageRef(String),

    // --- Cross-document ---
    Footnote(Vec<LatexNode>),
    /// \footnotemark[n] — places the superscript mark only
    FootnoteMark(Option<usize>),
    /// \footnotetext[n]{text} — places the footnote text only
    FootnoteText { num: Option<usize>, content: Vec<LatexNode> },

    // --- Floats ---
    Image(String),
    Caption(String),
    /// \caption*{text} — unnumbered caption
    CaptionStar(String),
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
    FancyHeader { pos: String, nodes: Vec<LatexNode> },
    FancyFooter { pos: String, nodes: Vec<LatexNode> },
    FancyClear,
    ThePage,

    // --- font declarations (group-scoped) ---
    FontDecl { style: String, nodes: Vec<LatexNode> },

    // --- color ---
    /// \definecolor{name}{model}{spec}
    DefineColor { name: String, css: String },
    /// \color{x} scoped to a group
    ColorDecl { color: String, nodes: Vec<LatexNode> },

    // --- boxes ---
    /// \parbox[pos]{width}{content}
    Parbox { width: String, nodes: Vec<LatexNode> },
    /// \raisebox{lift}[h][d]{content}
    Raisebox { lift: String, nodes: Vec<LatexNode> },

    // --- counter display ---
    /// \arabic{c}, \roman{c}, \Roman{c}, \alph{c}, \Alph{c}
    CounterValue { style: String, counter: String },

    // --- cross-reference ---
    /// \nameref{label}
    NameRef(String),
    /// \hyperref[label]{text}
    HyperRef { label: String, text: Vec<LatexNode> },
    /// \hypertarget{name}{text} — named anchor
    HyperTarget { name: String, nodes: Vec<LatexNode> },
    /// \hyperlink{name}{text} — link to \hypertarget
    HyperLink { name: String, nodes: Vec<LatexNode> },
    /// \phantomsection — invisible anchor for hyperref
    PhantomSection,

    // --- line / page spacing ---
    /// \linespread or setspace commands — emits a <style> block
    LineSpread(f64),
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

            LatexNode::SetLength { param, value } => {
                let css_var = match param.as_str() {
                    "\\parskip"      => "--latex-parskip",
                    "\\parindent"    => "--latex-parindent",
                    "\\baselineskip" => "--latex-baselineskip",
                    "\\textwidth"    => "--latex-textwidth",
                    "\\linewidth"    => "--latex-linewidth",
                    "\\textheight"   => "--latex-textheight",
                    "\\columnwidth"  => "--latex-columnwidth",
                    "\\columnsep"    => "--latex-columnsep",
                    "\\topmargin"    => "--latex-topmargin",
                    "\\oddsidemargin"| "\\evensidemargin" => "--latex-sidemargin",
                    _ => return String::new(),
                };
                format!("<style>:root {{ {}: {}; }}</style>", css_var, value)
            }

            LatexNode::Phantom(nodes) => {
                let inner = Nodes::render(nodes, ctx);
                format!(
                    "<span style=\"visibility:hidden;\">{}</span>",
                    inner
                )
            }

            LatexNode::HPhantom(nodes) => {
                let inner = Nodes::render(nodes, ctx);
                format!(
                    "<span style=\"visibility:hidden; display:inline-block; height:0; overflow:hidden;\">{}</span>",
                    inner
                )
            }

            LatexNode::VPhantom(nodes) => {
                let inner = Nodes::render(nodes, ctx);
                format!(
                    "<span style=\"visibility:hidden; display:inline-block; width:0; overflow:hidden;\">{}</span>",
                    inner
                )
            }

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

            // \footnotemark — place mark without text; deferred number resolved later
            LatexNode::FootnoteMark(explicit) => {
                let num = if let Some(n) = explicit {
                    *n
                } else {
                    ctx.footnote_num += 1;
                    ctx.footnote_num
                };
                format!(
                    "<sup class=\"footnote-ref\"><a href=\"#fn-{}\" id=\"fnref-{}\">{}</a></sup>",
                    num, num, num
                )
            }

            // \footnotetext — register text without emitting a mark
            LatexNode::FootnoteText { num, content } => {
                let n = if let Some(n) = num {
                    *n
                } else {
                    // use current counter (mark was placed first)
                    ctx.footnote_num
                };
                let html = Nodes::render(content, ctx);
                ctx.pending_footnotes.push((n, html));
                String::new()
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

                // \nocite{*} → include every entry in the database
                if ctx.nocite_all {
                    let mut all: Vec<String> = ctx.bib_database.keys().cloned().collect();
                    all.sort();
                    for key in all {
                        if !ctx.citation_map.contains_key(&key) {
                            ctx.register_citation(&key);
                        }
                    }
                }

                // Render in citation order if we have one, otherwise alphabetical
                let keys_ordered: Vec<String> = if !ctx.citation_order.is_empty() {
                    ctx.citation_order.clone()
                } else {
                    let mut k: Vec<String> = ctx.bib_database.keys().cloned().collect();
                    k.sort();
                    k
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

            // \nocite{*} or \nocite{key,...} — no visual output; registers keys
            LatexNode::NoCite(keys) => {
                for key in keys {
                    if key == "*" {
                        ctx.nocite_all = true;
                    } else {
                        ctx.register_citation(key);
                    }
                }
                String::new()
            }

            // \begin{thebibliography}{widest} \bibitem{key} ... \end{thebibliography}
            LatexNode::TheBibliography(items) => {
                let mut html = String::from("<h2 class=\"bib-title\">References</h2><ol class=\"bibliography\">");
                for (key, nodes) in items {
                    let content = Nodes::render(nodes, ctx);
                    html.push_str(&format!(
                        "<li id=\"ref-{}\">{}</li>",
                        key, content
                    ));
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

            LatexNode::CaptionStar(text) =>
                format!("<div class=\"caption\">{}</div>", text),

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

            // ----------------------------------------------------------------
            // Font declarations  \itshape \bfseries \ttfamily …
            // ----------------------------------------------------------------
            LatexNode::FontDecl { style, nodes } => {
                let inner = Nodes::render(nodes, ctx);
                match style.as_str() {
                    "itshape" | "slshape"  => format!("<em>{}</em>", inner),
                    "bfseries"             => format!("<strong>{}</strong>", inner),
                    "ttfamily"             => format!("<code>{}</code>", inner),
                    "sffamily"             => format!("<span style=\"font-family:sans-serif\">{}</span>", inner),
                    "rmfamily"             => format!("<span style=\"font-family:serif\">{}</span>", inner),
                    "upshape"              => format!("<span style=\"font-style:normal\">{}</span>", inner),
                    "scshape"              => format!("<span style=\"font-variant:small-caps\">{}</span>", inner),
                    "normalfont"           => format!("<span style=\"font-style:normal;font-weight:normal\">{}</span>", inner),
                    _                     => inner,
                }
            }

            // ----------------------------------------------------------------
            // \definecolor — register CSS value in context, no output
            // ----------------------------------------------------------------
            LatexNode::DefineColor { name, css } => {
                ctx.color_defs.insert(name.clone(), css.clone());
                String::new()
            }

            // ----------------------------------------------------------------
            // \color{x} — scoped text color
            // ----------------------------------------------------------------
            LatexNode::ColorDecl { color, nodes } => {
                let css = ctx.resolve_color(color);
                let inner = Nodes::render(nodes, ctx);
                format!("<span style=\"color:{}\">{}</span>", css, inner)
            }

            // ----------------------------------------------------------------
            // \parbox{w}{content}
            // ----------------------------------------------------------------
            LatexNode::Parbox { width, nodes } => {
                let inner = Nodes::render(nodes, ctx);
                format!(
                    "<div style=\"display:inline-block;vertical-align:top;width:{}\">{}</div>",
                    width, inner
                )
            }

            // ----------------------------------------------------------------
            // \raisebox{lift}{content}
            // ----------------------------------------------------------------
            LatexNode::Raisebox { lift, nodes } => {
                let inner = Nodes::render(nodes, ctx);
                format!(
                    "<span style=\"position:relative;bottom:{}\">{}</span>",
                    lift, inner
                )
            }

            // ----------------------------------------------------------------
            // Counter display  \arabic{c} \roman{c} …
            // ----------------------------------------------------------------
            LatexNode::CounterValue { style, counter } => {
                let n = ctx.counter_value(counter);
                match style.as_str() {
                    "arabic"  => n.to_string(),
                    "roman"   => Self::to_roman(n),
                    "Roman"   => Self::to_roman(n).to_uppercase(),
                    "alph"    => {
                        let idx = ((n.saturating_sub(1)) % 26) as u8;
                        (b'a' + idx) as char
                    }.to_string(),
                    "Alph"    => {
                        let idx = ((n.saturating_sub(1)) % 26) as u8;
                        (b'A' + idx) as char
                    }.to_string(),
                    "fnsymbol" => match n {
                        1 => "*", 2 => "†", 3 => "‡", 4 => "§",
                        5 => "¶", 6 => "‖", 7 => "**", 8 => "††",
                        _ => "?",
                    }.to_string(),
                    _ => n.to_string(),
                }
            }

            // ----------------------------------------------------------------
            // \nameref{label}
            // ----------------------------------------------------------------
            LatexNode::NameRef(label) => {
                let target = ctx.labels.get(label).cloned().unwrap_or_default();
                format!("<a href=\"#item-{}\" class=\"nameref\">{}</a>", target, label)
            }

            // ----------------------------------------------------------------
            // \hyperref[label]{text}
            // ----------------------------------------------------------------
            LatexNode::HyperRef { label, text } => {
                let target = ctx.labels.get(label).cloned().unwrap_or_default();
                let inner  = Nodes::render(text, ctx);
                format!("<a href=\"#item-{}\" class=\"hyperref\">{}</a>", target, inner)
            }

            // ----------------------------------------------------------------
            // \hypertarget{name}{text}
            // ----------------------------------------------------------------
            LatexNode::HyperTarget { name, nodes } => {
                let inner = Nodes::render(nodes, ctx);
                format!("<span id=\"ht-{}\">{}</span>", name, inner)
            }

            // ----------------------------------------------------------------
            // \hyperlink{name}{text}
            // ----------------------------------------------------------------
            LatexNode::HyperLink { name, nodes } => {
                let inner = Nodes::render(nodes, ctx);
                format!("<a href=\"#ht-{}\" class=\"hyperlink\">{}</a>", name, inner)
            }

            // ----------------------------------------------------------------
            // \phantomsection
            // ----------------------------------------------------------------
            LatexNode::PhantomSection => {
                ctx.phantom_id += 1;
                format!("<span id=\"phantom-{}\" aria-hidden=\"true\"></span>", ctx.phantom_id)
            }

            // ----------------------------------------------------------------
            // \linespread / \onehalfspacing / \doublespacing
            // ----------------------------------------------------------------
            LatexNode::LineSpread(factor) =>
                format!("<style>:root {{ --latex-baselineskip: {}; }} .document-container p, .document-container li {{ line-height: {}; }}</style>",
                    factor, factor),
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