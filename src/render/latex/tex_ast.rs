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
    /// enumitem \begin{enumerate}[label=...] — carries CSS list-style-type
    EnumerateLabeled { style: String, items: Vec<Vec<LatexNode>> },
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

    /// Render this node to HTML, appending into `buf`.
    /// This is the primary rendering path — a single buffer is reused across
    /// the whole document, avoiding thousands of intermediate `String` allocations.
    pub fn write_html(&self, ctx: &mut RenderContext, buf: &mut String) {
        use std::fmt::Write as _;
        match self {
            // ----------------------------------------------------------------
            // Plain text
            // ----------------------------------------------------------------
            LatexNode::Text(t) => buf.push_str(t),

            // ----------------------------------------------------------------
            // Inline formatting
            // ----------------------------------------------------------------
            LatexNode::Bold(nodes) => {
                buf.push_str("<strong>"); Nodes::write(nodes, ctx, buf); buf.push_str("</strong>");
            }
            LatexNode::Italic(nodes) => {
                buf.push_str("<em>"); Nodes::write(nodes, ctx, buf); buf.push_str("</em>");
            }
            LatexNode::Underline(nodes) => {
                buf.push_str("<u>"); Nodes::write(nodes, ctx, buf); buf.push_str("</u>");
            }
            LatexNode::Monospace(nodes) => {
                buf.push_str("<code>"); Nodes::write(nodes, ctx, buf); buf.push_str("</code>");
            }
            LatexNode::SmallCaps(nodes) => {
                buf.push_str("<span style=\"font-variant: small-caps;\">");
                Nodes::write(nodes, ctx, buf);
                buf.push_str("</span>");
            }
            LatexNode::Strikethrough(nodes) => {
                buf.push_str("<s>"); Nodes::write(nodes, ctx, buf); buf.push_str("</s>");
            }

            // ----------------------------------------------------------------
            // Font size
            // ----------------------------------------------------------------
            LatexNode::FontSize(size, nodes) => {
                let _ = write!(buf, "<span class=\"font-{}\">", size);
                Nodes::write(nodes, ctx, buf);
                buf.push_str("</span>");
            }

            // ----------------------------------------------------------------
            // Spacing & breaks
            // ----------------------------------------------------------------
            LatexNode::VSpace(size) => {
                let _ = write!(buf, "<div style=\"height: {}\"></div>", size);
            }
            LatexNode::HSpace(size) => {
                let _ = write!(buf, "<span style=\"display: inline-block; width: {}\"></span>", size);
            }

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
                    _ => return,
                };
                let _ = write!(buf, "<style>:root {{ {}: {}; }}</style>", css_var, value);
            }

            LatexNode::Phantom(nodes) => {
                buf.push_str("<span style=\"visibility:hidden;\">");
                Nodes::write(nodes, ctx, buf);
                buf.push_str("</span>");
            }
            LatexNode::HPhantom(nodes) => {
                buf.push_str("<span style=\"visibility:hidden; display:inline-block; height:0; overflow:hidden;\">");
                Nodes::write(nodes, ctx, buf);
                buf.push_str("</span>");
            }
            LatexNode::VPhantom(nodes) => {
                buf.push_str("<span style=\"visibility:hidden; display:inline-block; width:0; overflow:hidden;\">");
                Nodes::write(nodes, ctx, buf);
                buf.push_str("</span>");
            }

            LatexNode::LineBreak     => buf.push_str("<br/>"),
            LatexNode::NewPage       => buf.push_str("<div style=\"page-break-after: always;\"></div>"),
            LatexNode::HorizontalRule => buf.push_str("<hr class=\"latex-hr\"/>"),

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
                let _ = write!(
                    buf,
                    "<div class=\"latex-part\" id=\"part-{}\"><span class=\"part-label\">Part {}</span><span class=\"part-title\">{}</span></div>",
                    ctx.part_num, roman, title
                );
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
            }

            LatexNode::Chapter(title) => {
                ctx.chap_num += 1;
                ctx.sec_num = 0;
                ctx.subsec_num = 0;
                let num = ctx.chap_num;
                ctx.toc.push((1, num.to_string(), title.clone()));
                let _ = write!(buf, "<h1 id=\"item-{}\">{} &nbsp;&nbsp; {}</h1>", num, num, title);
            }

            LatexNode::Section(title) => {
                ctx.sec_num += 1;
                ctx.subsec_num = 0;
                ctx.subsubsec_num = 0;
                let num = ctx.sec_num;
                ctx.toc.push((2, num.to_string(), title.clone()));
                let _ = write!(buf, "<h2 id=\"item-{}\">{} &nbsp;&nbsp; {}</h2>", num, num, title);
            }

            LatexNode::Subsection(title) => {
                ctx.subsec_num += 1;
                ctx.subsubsec_num = 0;
                let num = format!("{}.{}", ctx.sec_num, ctx.subsec_num);
                ctx.toc.push((3, num.clone(), title.clone()));
                let _ = write!(buf, "<h3 id=\"item-{}\">{} &nbsp;&nbsp; {}</h3>", num, num, title);
            }

            LatexNode::Subsubsection(title) => {
                ctx.subsubsec_num += 1;
                let num = format!("{}.{}.{}", ctx.sec_num, ctx.subsec_num, ctx.subsubsec_num);
                ctx.toc.push((4, num.clone(), title.clone()));
                let _ = write!(buf, "<h4 id=\"item-{}\">{} &nbsp;&nbsp; {}</h4>", num, num, title);
            }

            LatexNode::Paragraph(title) => {
                let _ = write!(buf, "<p class=\"latex-paragraph\"><strong>{}</strong> ", title);
            }

            // ----------------------------------------------------------------
            // Abstract
            // ----------------------------------------------------------------
            LatexNode::Abstract(nodes) => {
                buf.push_str("<div class=\"abstract\"><h3 class=\"abstract-title\">Abstract</h3><p>");
                Nodes::write(nodes, ctx, buf);
                buf.push_str("</p></div>");
            }

            // ----------------------------------------------------------------
            // Lists
            // ----------------------------------------------------------------
            LatexNode::Itemize(items) => {
                buf.push_str("<ul>");
                for item in items {
                    buf.push_str("<li>"); Nodes::write(item, ctx, buf); buf.push_str("</li>");
                }
                buf.push_str("</ul>");
            }

            LatexNode::Enumerate(items) => {
                buf.push_str("<ol>");
                for item in items {
                    buf.push_str("<li>"); Nodes::write(item, ctx, buf); buf.push_str("</li>");
                }
                buf.push_str("</ol>");
            }

            LatexNode::EnumerateLabeled { style, items } => {
                let _ = write!(buf, "<ol style=\"list-style-type:{}\">" , style);
                for item in items {
                    buf.push_str("<li>"); Nodes::write(item, ctx, buf); buf.push_str("</li>");
                }
                buf.push_str("</ol>");
            }

            LatexNode::Description(items) => {
                buf.push_str("<dl>");
                for (term, desc) in items {
                    let _ = write!(buf, "<dt><strong>{}</strong></dt><dd>", term);
                    Nodes::write(desc, ctx, buf);
                    buf.push_str("</dd>");
                }
                buf.push_str("</dl>");
            }

            // ----------------------------------------------------------------
            // Math
            // ----------------------------------------------------------------
            LatexNode::RawMathInline(raw) => {
                let _ = write!(buf, "\\({}\\)", raw);
            }
            LatexNode::RawMathDisplay(raw) => {
                let _ = write!(buf, "<div class=\"math-block\">\\[{}\\]</div>", raw);
            }

            #[allow(dead_code)]
            LatexNode::MathInline(nodes) => {
                buf.push_str("<span class=\"math-inline\">");
                Nodes::write(nodes, ctx, buf);
                buf.push_str("</span>");
            }

            #[allow(dead_code)]
            LatexNode::MathDisplay(nodes) => {
                buf.push_str("<div class=\"math-display\">");
                Nodes::write(nodes, ctx, buf);
                buf.push_str("</div>");
            }

            LatexNode::Superscript(nodes) => {
                buf.push_str("<sup>"); Nodes::write(nodes, ctx, buf); buf.push_str("</sup>");
            }
            LatexNode::Subscript(nodes) => {
                buf.push_str("<sub>"); Nodes::write(nodes, ctx, buf); buf.push_str("</sub>");
            }

            LatexNode::Fraction { num, den } => {
                buf.push_str("<span class=\"latex-frac\"><span class=\"frac-num\">");
                Nodes::write(num, ctx, buf);
                buf.push_str("</span><span class=\"frac-den\">");
                Nodes::write(den, ctx, buf);
                buf.push_str("</span></span>");
            }

            LatexNode::EquationBlock(nodes) => {
                ctx.eq_num += 1;
                let num = ctx.eq_num;
                Self::register_inner_labels(nodes, &num.to_string(), ctx);
                let _ = write!(buf, "<div class=\"math-block\" id=\"item-{}\">" , num);
                if let Some(LatexNode::RawMathDisplay(raw)) = nodes.first() {
                    let _ = write!(buf, "\\[{}\\]", raw);
                } else {
                    Nodes::write(nodes, ctx, buf);
                }
                let _ = write!(buf, " <span class=\"eq-number\">({})</span></div>", num);
            }

            LatexNode::AlignBlock(nodes) => {
                // For multi-line align/gather/eqnarray, inject \tag{n} on each
                // \\ line so MathJax renders the right number without relying on
                // `tags:'ams'` (which conflicts with our HTML-level numbering).
                if let Some(LatexNode::RawMathDisplay(raw)) = nodes.first() {
                    // raw = \begin{align}body\end{align}
                    let begin_end = raw.find('}').map(|i| i + 1).unwrap_or(0);
                    let close_start = raw.rfind("\\end").unwrap_or(raw.len());
                    let env_open  = &raw[..begin_end];
                    let body      = &raw[begin_end..close_start];
                    let env_close = &raw[close_start..];

                    // Split on \\ (the line separator in LaTeX align bodies)
                    let lines: Vec<&str> = body.split("\\\\").collect();
                    let first_num = ctx.eq_num + 1;
                    ctx.eq_num += lines.len();

                    let _ = write!(buf, "<div class=\"math-block math-align\" id=\"item-{}\">", first_num);
                    buf.push_str(env_open);
                    for (i, line) in lines.iter().enumerate() {
                        let eq = first_num + i;
                        buf.push_str(line);
                        if i + 1 < lines.len() {
                            let _ = write!(buf, " \\tag{{{}}} \\\\", eq);
                        } else {
                            let _ = write!(buf, " \\tag{{{}}}", eq);
                        }
                    }
                    buf.push_str(env_close);
                    buf.push_str("</div>");
                } else {
                    ctx.eq_num += 1;
                    let num = ctx.eq_num;
                    Self::register_inner_labels(nodes, &num.to_string(), ctx);
                    let _ = write!(buf, "<div class=\"math-block math-align\" id=\"item-{}\">", num);
                    Nodes::write(nodes, ctx, buf);
                    buf.push_str("</div>");
                }
            }

            // ----------------------------------------------------------------
            // Footnotes
            // ----------------------------------------------------------------
            LatexNode::Footnote(nodes) => {
                ctx.footnote_num += 1;
                let num = ctx.footnote_num;
                let content = Nodes::render(nodes, ctx);
                ctx.pending_footnotes.push((num, content));
                let _ = write!(
                    buf,
                    "<sup class=\"footnote-ref\"><a href=\"#fn-{}\" id=\"fnref-{}\">{}</a></sup>",
                    num, num, num
                );
            }

            LatexNode::FootnoteMark(explicit) => {
                let num = if let Some(n) = explicit { *n } else {
                    ctx.footnote_num += 1; ctx.footnote_num
                };
                let _ = write!(
                    buf,
                    "<sup class=\"footnote-ref\"><a href=\"#fn-{}\" id=\"fnref-{}\">{}</a></sup>",
                    num, num, num
                );
            }

            LatexNode::FootnoteText { num, content } => {
                let n = if let Some(n) = num { *n } else { ctx.footnote_num };
                let html = Nodes::render(content, ctx);
                ctx.pending_footnotes.push((n, html));
            }

            // ----------------------------------------------------------------
            // References & labels
            // ----------------------------------------------------------------
            LatexNode::Label(name) => {
                let _ = write!(buf, "<span id=\"label-{}\"></span>", name);
            }

            LatexNode::Ref(key) => {
                let num = ctx.labels.get(key).cloned().unwrap_or_else(|| "??".to_string());
                let prefix = if key.starts_with("ref-") { "ref" } else { "item" };
                let _ = write!(buf, "<a href=\"#{}-{}\" class=\"cross-ref\">{}</a>", prefix, num, num);
            }

            LatexNode::PageRef(label) => {
                let (href, data_ref) = if let Some(num) = ctx.labels.get(label) {
                    (format!("item-{}", num), format!("item-{}", num))
                } else {
                    (format!("label-{}", label), format!("label-{}", label))
                };
                let _ = write!(
                    buf,
                    "<a href=\"#{}\" class=\"cross-ref pageref\" data-ref=\"{}\">??</a>",
                    href, data_ref
                );
            }

            LatexNode::Cite(key) => {
                let number = ctx.register_citation(key);
                let _ = write!(buf, "<a href=\"#ref-{}\" class=\"cite\">[{}]</a>", key, number);
            }

            LatexNode::CiteMultiple(keys) => {
                buf.push_str("[");
                let mut first = true;
                for key in keys {
                    if !first { buf.push_str(", "); }
                    first = false;
                    let number = ctx.register_citation(key);
                    let _ = write!(buf, "<a href=\"#ref-{}\" class=\"cite\">{}</a>", key, number);
                }
                buf.push_str("]");
            }

            // ----------------------------------------------------------------
            // Bibliography
            // ----------------------------------------------------------------
            LatexNode::Bibliography(file) => {
                let source = if file.starts_with("http://") || file.starts_with("https://") {
                    file.clone()
                } else {
                    format!("{}.bib", file)
                };
                let bib_content = BibTextRender::fetch_bibliography(&source).unwrap_or_default();
                ctx.bib_database = BibTextRender::parse_bibtex(&bib_content);

                if ctx.nocite_all {
                    let mut all: Vec<String> = ctx.bib_database.keys().cloned().collect();
                    all.sort();
                    for key in all {
                        if !ctx.citation_map.contains_key(&key) { ctx.register_citation(&key); }
                    }
                }

                let keys_ordered: Vec<String> = if !ctx.citation_order.is_empty() {
                    ctx.citation_order.clone()
                } else {
                    let mut k: Vec<String> = ctx.bib_database.keys().cloned().collect();
                    k.sort(); k
                };

                buf.push_str("<h2 class=\"bib-title\">References</h2><ol class=\"bibliography\">");
                for key in &keys_ordered {
                    if let Some(entry) = ctx.bib_database.get(key) {
                        let _ = write!(
                            buf,
                            "<li id=\"ref-{}\">{}, <em>{}</em>, {}.</li>",
                            key, entry.author, entry.title, entry.year
                        );
                    } else {
                        let _ = write!(
                            buf,
                            "<li><strong style='color:red;'>Error: Ref '{}' not found!</strong></li>",
                            key
                        );
                    }
                }
                buf.push_str("</ol>");
            }

            LatexNode::NoCite(keys) => {
                for key in keys {
                    if key == "*" { ctx.nocite_all = true; } else { ctx.register_citation(key); }
                }
            }

            LatexNode::TheBibliography(items) => {
                buf.push_str("<h2 class=\"bib-title\">References</h2><ol class=\"bibliography\">");
                for (key, nodes) in items {
                    let _ = write!(buf, "<li id=\"ref-{}\">", key);
                    Nodes::write(nodes, ctx, buf);
                    buf.push_str("</li>");
                }
                buf.push_str("</ol>");
            }

            // ----------------------------------------------------------------
            // Links
            // ----------------------------------------------------------------
            LatexNode::Url(url) => {
                let _ = write!(buf, "<a href=\"{}\">{}</a>", url, url);
            }
            LatexNode::Href { url, text } => {
                let _ = write!(buf, "<a href=\"{}\">", url);
                Nodes::write(text, ctx, buf);
                buf.push_str("</a>");
            }

            // ----------------------------------------------------------------
            // TOC
            // ----------------------------------------------------------------
            LatexNode::TableOfContents => buf.push_str("__TOC_PLACEHOLDER__"),

            // ----------------------------------------------------------------
            // Floats
            // ----------------------------------------------------------------
            LatexNode::TableFloat(children) => {
                ctx.tab_num += 1;
                ctx.last_counter = ctx.tab_num.to_string();
                ctx.in_float = true;
                Nodes::write(children, ctx, buf);
                ctx.in_float = false;
            }

            LatexNode::FigureFloat(children) => {
                ctx.fig_num += 1;
                ctx.last_counter = ctx.fig_num.to_string();
                ctx.in_float = true;
                Nodes::write(children, ctx, buf);
                ctx.in_float = false;
            }

            LatexNode::Caption(text) => {
                let _ = write!(
                    buf,
                    "<div class=\"caption\"><strong>Figure/Table {}:</strong> {}</div>",
                    ctx.last_counter, text
                );
            }
            LatexNode::CaptionStar(text) => {
                let _ = write!(buf, "<div class=\"caption\">{}</div>", text);
            }

            LatexNode::Image(url) => {
                if !ctx.in_float { ctx.fig_num += 1; ctx.last_counter = ctx.fig_num.to_string(); }
                let num = &ctx.last_counter;
                let _ = write!(
                    buf,
                    "<img src=\"{}\" class=\"latex-image\" id=\"item-{}\" alt=\"Figure {}\" />",
                    url, num, num
                );
            }

            LatexNode::Table(rows) => {
                if ctx.last_counter == "0" || !ctx.in_float {
                    ctx.tab_num += 1;
                    ctx.last_counter = ctx.tab_num.to_string();
                }
                let num = ctx.last_counter.clone();
                let _ = write!(buf, "<table class=\"latex-table\" id=\"item-{}\"><tbody>\n", num);

                for row in rows {
                    buf.push_str("  <tr>\n");
                    for cell in row {
                        let mut attrs = String::new();
                        if cell.colspan > 1 { let _ = write!(attrs, " colspan=\"{}\"", cell.colspan); }
                        if cell.rowspan > 1 { let _ = write!(attrs, " rowspan=\"{}\"", cell.rowspan); }

                        let mut style_parts: Vec<String> = Vec::new();
                        if !cell.align.is_empty() {
                            style_parts.push(format!("text-align:{}", cell.align));
                        }
                        if let Some(w) = &cell.width {
                            style_parts.push(format!("width:{}", w));
                        }
                        if cell.hline {
                            style_parts.push("border-top:2px solid #2c3e50".to_string());
                        }
                        if !style_parts.is_empty() {
                            let _ = write!(attrs, " style=\"{}\"", style_parts.join("; "));
                        }

                        let _ = write!(buf, "    <td{}>", attrs);
                        Nodes::write(&cell.content, ctx, buf);
                        buf.push_str("</td>\n");
                    }
                    buf.push_str("  </tr>\n");
                }
                buf.push_str("</tbody></table>\n");
            }

            LatexNode::Matrix { open, close, rows } => {
                if rows.is_empty() {
                    let _ = write!(buf, "{}{}", open, close);
                    return;
                }
                let col_count = rows.iter().map(|r| r.len()).max().unwrap_or(1);
                let _ = write!(
                    buf,
                    "<span class=\"latex-matrix-wrap\"><span class=\"matrix-delim\">{open}</span>\
                     <span class=\"latex-matrix\" style=\"grid-template-columns: repeat({col_count}, auto);\">",
                );
                for row in rows {
                    for cell in row {
                        buf.push_str("<span class=\"matrix-cell\">");
                        Nodes::write(cell, ctx, buf);
                        buf.push_str("</span>");
                    }
                    for _ in row.len()..col_count {
                        buf.push_str("<span class=\"matrix-cell\"></span>");
                    }
                }
                let _ = write!(buf, "</span><span class=\"matrix-delim\">{close}</span></span>");
            }

            // ----------------------------------------------------------------
            // Code / Mermaid
            // ----------------------------------------------------------------
            LatexNode::CodeBlock(code) => {
                buf.push_str("<pre class=\"code-block\"><code>");
                buf.push_str(&code.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;"));
                buf.push_str("</code></pre>");
            }
            LatexNode::Mermaid(raw_code) => {
                let _ = write!(buf, "<div class=\"mermaid\">\n{}\n</div>", raw_code);
            }

            // ----------------------------------------------------------------
            // Document metadata
            // ----------------------------------------------------------------
            LatexNode::Title(t)  => { ctx.doc_title  = Nodes::render(t, ctx); }
            LatexNode::Author(a) => { ctx.doc_author = Nodes::render(a, ctx); }
            LatexNode::Date(d)   => { ctx.doc_date   = Nodes::render(d, ctx); }

            LatexNode::MakeTitle => {
                let _ = write!(
                    buf,
                    "<div class=\"title-block\">\n  <h1>{}</h1>\n  <div class=\"author\">{}</div>\n  ",
                    ctx.doc_title, ctx.doc_author
                );
                if !ctx.doc_date.is_empty() {
                    let _ = write!(buf, "<div class=\"date\">{}</div>", ctx.doc_date);
                }
                buf.push_str("</div>");
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
            }

            LatexNode::FancyClear => {
                ctx.header_left   = String::new();
                ctx.header_center = String::new();
                ctx.header_right  = String::new();
                ctx.footer_left   = String::new();
                ctx.footer_center = String::new();
                ctx.footer_right  = String::new();
            }

            LatexNode::ThePage =>
                buf.push_str("<span class=\"thepage\" aria-hidden=\"true\"></span>"),

            // ----------------------------------------------------------------
            // Font declarations  \itshape \bfseries \ttfamily …
            // ----------------------------------------------------------------
            LatexNode::FontDecl { style, nodes } => {
                let (open, close): (&str, &str) = match style.as_str() {
                    "itshape" | "slshape"  => ("<em>", "</em>"),
                    "bfseries"             => ("<strong>", "</strong>"),
                    "ttfamily"             => ("<code>", "</code>"),
                    "sffamily"             => ("<span style=\"font-family:sans-serif\">", "</span>"),
                    "rmfamily"             => ("<span style=\"font-family:serif\">", "</span>"),
                    "upshape"              => ("<span style=\"font-style:normal\">", "</span>"),
                    "scshape"              => ("<span style=\"font-variant:small-caps\">", "</span>"),
                    "normalfont"           => ("<span style=\"font-style:normal;font-weight:normal\">", "</span>"),
                    _                      => ("", ""),
                };
                buf.push_str(open);
                Nodes::write(nodes, ctx, buf);
                buf.push_str(close);
            }

            // ----------------------------------------------------------------
            // \definecolor — register CSS value in context, no output
            // ----------------------------------------------------------------
            LatexNode::DefineColor { name, css } => {
                ctx.color_defs.insert(name.clone(), css.clone());
            }

            // ----------------------------------------------------------------
            // \color{x} — scoped text color
            // ----------------------------------------------------------------
            LatexNode::ColorDecl { color, nodes } => {
                let css = ctx.resolve_color(color);
                let _ = write!(buf, "<span style=\"color:{}\">", css);
                Nodes::write(nodes, ctx, buf);
                buf.push_str("</span>");
            }

            // ----------------------------------------------------------------
            // \parbox{w}{content}
            // ----------------------------------------------------------------
            LatexNode::Parbox { width, nodes } => {
                let _ = write!(buf, "<div style=\"display:inline-block;vertical-align:top;width:{}\">", width);
                Nodes::write(nodes, ctx, buf);
                buf.push_str("</div>");
            }

            // ----------------------------------------------------------------
            // \raisebox{lift}{content}
            // ----------------------------------------------------------------
            LatexNode::Raisebox { lift, nodes } => {
                let _ = write!(buf, "<span style=\"position:relative;bottom:{}\">", lift);
                Nodes::write(nodes, ctx, buf);
                buf.push_str("</span>");
            }

            // ----------------------------------------------------------------
            // Counter display  \arabic{c} \roman{c} …
            // ----------------------------------------------------------------
            LatexNode::CounterValue { style, counter } => {
                let n = ctx.counter_value(counter);
                match style.as_str() {
                    "arabic"   => { let _ = write!(buf, "{}", n); }
                    "roman"    => buf.push_str(&Self::to_roman(n)),
                    "Roman"    => buf.push_str(&Self::to_roman(n).to_uppercase()),
                    "alph"     => buf.push((b'a' + ((n.saturating_sub(1)) % 26) as u8) as char),
                    "Alph"     => buf.push((b'A' + ((n.saturating_sub(1)) % 26) as u8) as char),
                    "fnsymbol" => buf.push_str(match n {
                        1 => "*", 2 => "†", 3 => "‡", 4 => "§",
                        5 => "¶", 6 => "‖", 7 => "**", 8 => "††", _ => "?",
                    }),
                    _ => { let _ = write!(buf, "{}", n); }
                }
            }

            // ----------------------------------------------------------------
            // \nameref, \hyperref, \hypertarget, \hyperlink, \phantomsection
            // ----------------------------------------------------------------
            LatexNode::NameRef(label) => {
                let target = ctx.labels.get(label).cloned().unwrap_or_default();
                let _ = write!(buf, "<a href=\"#item-{}\" class=\"nameref\">{}</a>", target, label);
            }

            LatexNode::HyperRef { label, text } => {
                let target = ctx.labels.get(label).cloned().unwrap_or_default();
                let _ = write!(buf, "<a href=\"#item-{}\" class=\"hyperref\">", target);
                Nodes::write(text, ctx, buf);
                buf.push_str("</a>");
            }

            LatexNode::HyperTarget { name, nodes } => {
                let _ = write!(buf, "<span id=\"ht-{}\">", name);
                Nodes::write(nodes, ctx, buf);
                buf.push_str("</span>");
            }

            LatexNode::HyperLink { name, nodes } => {
                let _ = write!(buf, "<a href=\"#ht-{}\" class=\"hyperlink\">", name);
                Nodes::write(nodes, ctx, buf);
                buf.push_str("</a>");
            }

            LatexNode::PhantomSection => {
                ctx.phantom_id += 1;
                let _ = write!(buf, "<span id=\"phantom-{}\" aria-hidden=\"true\"></span>", ctx.phantom_id);
            }

            // ----------------------------------------------------------------
            // \linespread / \onehalfspacing / \doublespacing
            // ----------------------------------------------------------------
            LatexNode::LineSpread(factor) => {
                let _ = write!(
                    buf,
                    "<style>:root {{ --latex-baselineskip: {}; }} \
                     .document-container p, .document-container li {{ line-height: {}; }}</style>",
                    factor, factor
                );
            }
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