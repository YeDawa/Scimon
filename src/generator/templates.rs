pub struct Templates;

impl Templates {

    pub fn generic(&self, html_content: &str) -> String {
        format!(r#"
            <html>
            <head>
                <meta charset="utf-8">
                <style>
                    body {{ font-family: 'Helvetica', serif; }}
                </style>
            </head>
            <body>{}</body>
            </html>
        "#, html_content)
    }

    pub fn markdown(&self, css_style: &str, html_content: &str) -> String {
        format!(
            "<!DOCTYPE html>
            <html lang='en'>
            <head>
                <meta charset='UTF-8'>
                <meta name='viewport' content='width=device-width, initial-scale=1.0'>
                <style>{}</style>
            </head>
            <body>
                <article class='markdown-body'>{}</article>
            </body>
            </html>",
            css_style, html_content
        )
    }

    pub fn latex(content: &str) -> String {
        format!(
            r##"<!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                
                <link rel="preconnect" href="https://fonts.googleapis.com">
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
                <link href="https://fonts.googleapis.com/css2?family=Fira+Code:wght@400;500&family=Lora:ital,wght@0,400;0,600;1,400&display=swap" rel="stylesheet">
                
                <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/atom-one-dark.min.css">
                <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js"></script>

                <script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
                <script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
                
                <style>
                    /* Typography & Base */
                    body {{
                        background-color: #f4f6f8;
                        font-family: 'Lora', 'Computer Modern', Georgia, serif;
                        font-size: 18px;
                        line-height: 1.7;
                        color: #2c3e50;
                        margin: 0;
                        padding: 40px 20px;
                    }}
                    
                    .document-container {{
                        background-color: #ffffff;
                        max-width: 850px;
                        margin: 0 auto;
                        padding: 60px 80px;
                    }}
                    
                    /* Headings & Title */
                    .title-block {{ text-align: center; margin-bottom: 60px; padding-bottom: 25px; border-bottom: 1px solid #eaeaea; }}
                    .title-block h1 {{ font-size: 2.6em; margin: 0 0 15px 0; color: #1a252f; font-weight: 600; line-height: 1.2; }}
                    .title-block .author {{ font-size: 1.25em; color: #7f8c8d; font-style: italic; }}

                    h2 {{ border-bottom: 2px solid #f0f2f5; padding-bottom: 8px; color: #1a252f; margin-top: 50px; font-weight: 600; }}
                    h3 {{ color: #2c3e50; margin-top: 35px; font-weight: 600; }}
                    
                    /* Lists & TOC */
                    ul, ol {{ background: #fbfcfc; padding: 20px 20px 20px 50px; border-radius: 0 8px 8px 0; margin: 20px 0; }}
                    .toc {{ background: #f8f9fa; padding: 25px; border-radius: 6px; }}
                    .toc ul {{ border: none; background: transparent; padding: 0; margin: 0; list-style: none; }}
                    .toc li {{ margin-bottom: 8px; font-size: 0.95em; }}
                    .toc a {{ text-decoration: none; color: #2980b9; transition: color 0.2s; }}
                    .toc a:hover {{ color: #1abc9c; text-decoration: underline; }}
                    
                    /* Math */
                    .math-inline {{ font-family: 'Cambria Math', 'Times New Roman', serif; font-style: italic; background: #fdf2e9; padding: 2px 6px; border-radius: 4px; white-space: nowrap; color: #d35400; }}
                    .math-block {{ font-family: 'Cambria Math', 'Times New Roman', serif; font-size: 1.3em; text-align: center; margin: 35px 0; padding: 20px; background: #fdfefe; overflow-x: auto; font-style: italic; position: relative; }}
                    .eq-number {{ position: absolute; right: 20px; top: 50%; transform: translateY(-50%); font-size: 0.8em; font-style: normal; color: #95a5a6; }}
                    sup, sub {{ font-size: 0.75em; line-height: 0; position: relative; vertical-align: baseline; }}
                    sup {{ top: -0.5em; }} sub {{ bottom: -0.25em; }}
                    .latex-frac {{
                        display: inline-flex;
                        flex-direction: column;
                        vertical-align: middle;
                        text-align: center;
                        margin: 0 0.2em;
                        font-size: 0.9em;
                    }}
                    .frac-num {{ border-bottom: 1px solid currentColor; padding: 0 0.2em; }}
                    .frac-den {{ padding: 0 0.2em; }}

                    /* Matrix */
                    .latex-matrix-wrap {{
                        display: inline-flex;
                        align-items: center;
                        vertical-align: middle;
                    }}
                    .matrix-delim {{
                        font-size: 2em;
                        line-height: 1;
                        font-weight: 200;
                        font-family: 'Cambria Math', 'Times New Roman', serif;
                    }}
                    .latex-matrix {{
                        display: inline-grid;
                        column-gap: 0.6em;
                        row-gap: 0.3em;
                        margin: 0 0.1em;
                        text-align: center;
                        align-items: baseline;
                    }}
                    /* inline-flex keeps <sub>/<sup> anchored to their base character.
                       display:contents caused them to become independent grid items. */
                    .matrix-cell {{
                        display: inline-flex;
                        align-items: baseline;
                        justify-content: center;
                    }}

                    /* Code Blocks (Syntax Highlighting) */
                    pre {{ margin: 30px; border-radius: 8px; }}
                    pre code {{ font-family: 'Fira Code', 'Courier New', monospace; font-size: 0.9em; padding: 20px !important; border-radius: 8px; line-height: 1.6; }}
                    .code-block {{ margin: 30px 10px; padding-left: 5px; padding-right: 5px; border-radius: 8px; overflow: hidden; }}
                    
                    /* Visuals & Cross-Refs */
                    .latex-image {{ display: block; margin: 40px auto 15px; max-width: 100%; border-radius: 8px; }}
                    .caption {{ text-align: center; font-size: 0.9em; color: #7f8c8d; margin-bottom: 35px; font-style: italic; }}
                    .cross-ref {{ color: #e67e22; font-weight: 600; text-decoration: none; padding: 0 2px; transition: color 0.2s; }}
                    .cross-ref:hover {{ color: #d35400; text-decoration: underline; }}
                    
                    /* Tables (LaTeX Booktabs Style) */
                    .latex-table {{ border-collapse: collapse; margin: 40px auto; font-size: 0.95em; min-width: 70%; background: #fff; }}
                    .latex-table th {{ padding: 15px; border-top: 2px solid #2c3e50; border-bottom: 1px solid #2c3e50; font-weight: 600; text-align: center; color: #1a252f; }}
                    .latex-table td {{ padding: 12px 15px; border-bottom: 1px solid #ecf0f1; text-align: center; }}
                    .latex-table tr:last-child td {{ border-bottom: 2px solid #2c3e50; }}
                    .latex-table tr:hover {{ background-color: #f8f9fa; }}
                    
                    /* Citations & Bibliography */
                    .cite {{ color: #27ae60; text-decoration: none; font-weight: 600; padding: 0 2px; }}
                    .cite:hover {{ text-decoration: underline; }}
                    .bibliography {{ background: #fbfcfc; padding: 25px 40px; border-radius: 6px; margin-top: 40px; }}

                    /* Font Sizes */
                    .font-tiny {{ font-size: 0.6em; }}
                    .font-scriptsize {{ font-size: 0.7em; }}
                    .font-footnotesize {{ font-size: 0.8em; }}
                    .font-small {{ font-size: 0.85em; }}
                    .font-normalsize {{ font-size: 1em; }}
                    .font-large {{ font-size: 1.2em; }}
                    .font-Large {{ font-size: 1.4em; }}
                    .font-LARGE {{ font-size: 1.8em; }}
                    .font-huge {{ font-size: 2.0em; }}
                    .font-Huge {{ font-size: 2.5em; }}
                    .font-HUGE {{ font-size: 3.0em; }}

                    /* Footnotes */
                    .footnote-rule {{ margin-top: 60px; border: none; border-top: 1px solid #eaeaea; }}
                    .footnote-list {{ font-size: 0.85em; color: #666; padding-left: 25px; }}
                    .footnote-ref a {{ color: #e67e22; font-weight: 600; text-decoration: none; font-size: 0.8em; vertical-align: super; }}
                    .footnote-back {{ color: #aaa; text-decoration: none; margin-left: 4px; }}

                    /* Abstract */
                    .abstract {{ background: #f8f9fa; border-left: 4px solid #2980b9; padding: 20px 30px; margin: 30px 0; border-radius: 0 6px 6px 0; }}
                    .abstract-title {{ margin: 0 0 10px; font-size: 1em; color: #2980b9; }}

                    /* Theorems */
                    .latex-theorem {{ background: #f9f9f9; border-left: 4px solid #8e44ad; padding: 15px 20px; margin: 20px 0; border-radius: 0 6px 6px 0; }}
                    .latex-proof {{ border-left-color: #27ae60; }}

                    /* Math text / operators */
                    .math-text {{ font-style: normal; font-family: 'Lora', Georgia, serif; }}
                    .math-op {{ font-style: normal; font-family: 'Cambria Math', 'Times New Roman', serif; margin-right: 0.15em; }}

                    /* Bibliography title */
                    .bib-title {{ border-bottom: 2px solid #f0f2f5; padding-bottom: 8px; color: #1a252f; margin-top: 50px; font-size: 1.5em; font-weight: 600; }}

                    /* Blockquote */
                    blockquote.latex-quote, .latex-quote {{
                        border-left: 4px solid #bdc3c7;
                        margin: 20px 0 20px 20px;
                        padding: 10px 20px;
                        color: #555;
                        font-style: italic;
                        background: #fafafa;
                        border-radius: 0 6px 6px 0;
                    }}

                    /* Horizontal rule */
                    hr.latex-hr {{ border: none; border-top: 1px solid #bdc3c7; margin: 30px 0; }}

                    /* Date in title block */
                    .title-block .date {{ font-size: 1em; color: #95a5a6; margin-top: 8px; }}

                    /* Paragraph command */
                    .latex-paragraph {{ margin-top: 1em; }}

                    /* Starred sections (no numbering) */
                    .section-star {{ border-bottom: 2px solid #f0f2f5; padding-bottom: 8px; color: #1a252f; margin-top: 50px; font-weight: 600; }}

                    /* Math align */
                    .math-align {{ text-align: left; }}

                    /* Math intertext */
                    .math-intertext {{ font-style: normal; font-size: 0.95em; margin: 10px 0; text-align: left; }}

                    /* Responsive */
                    @media (max-width: 768px) {{
                        .document-container {{ padding: 30px 20px; }}
                    }}
                </style>
                
                <script type="module">
                    import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.esm.min.mjs';
                    mermaid.initialize({{ startOnLoad: true, theme: 'default' }});
                </script>
            </head>
            <body>
                <div class="document-container">{}</div>
                
                <script>
                    // -------------------------------------------------------
                    // pageref: resolve \pageref{{}} after full layout.
                    // Runs after body so DOM + fonts are ready.
                    // -------------------------------------------------------
                    (function () {{
                        var PAGE_HEIGHT_PX = 1122; // A4 @ 96 dpi
                        var BODY_OFFSET    = 40;   // body padding-top

                        function absTop(el) {{
                            var r = el.getBoundingClientRect();
                            return r.top + (window.scrollY || document.documentElement.scrollTop);
                        }}

                        function pageOf(el) {{
                            return Math.floor(Math.max(0, absTop(el) - BODY_OFFSET) / PAGE_HEIGHT_PX) + 1;
                        }}

                        function findTarget(id) {{
                            // 1. Direct id match  (item-1, label-sec:foo, …)
                            var el = document.getElementById(id);
                            if (el) return el;
                            // 2. If id starts with "item-", also try "label-" variant
                            if (id.startsWith("item-")) {{
                                return document.getElementById("label-" + id.slice(5));
                            }}
                            // 3. If id starts with "label-", also try "item-" variant
                            if (id.startsWith("label-")) {{
                                return document.getElementById("item-" + id.slice(6));
                            }}
                            return null;
                        }}

                        function resolvePageRefs() {{
                            document.querySelectorAll("[data-ref]").forEach(function (ref) {{
                                var id     = ref.getAttribute("data-ref");
                                var target = findTarget(id);
                                if (target) {{
                                    ref.textContent = String(pageOf(target));
                                }} else {{
                                    ref.textContent = "??";
                                    console.warn("[pageref] target not found:", id);
                                }}
                            }});
                        }}

                        if (document.fonts && document.fonts.ready) {{
                            document.fonts.ready.then(resolvePageRefs);
                        }} else {{
                            window.addEventListener("load", resolvePageRefs);
                        }}

                        var t;
                        window.addEventListener("resize", function () {{
                            clearTimeout(t);
                            t = setTimeout(resolvePageRefs, 150);
                        }});

                        document.querySelectorAll("[data-ref]").forEach(function(el) {{
                            var id = el.getAttribute("data-ref");
                            var target = document.getElementById(id);
                            console.log("data-ref:", id, "→ target:", target, "→ absTop:", target ? target.getBoundingClientRect().top : "NOT FOUND");
                        }});

                        window.resolvePageRefs = resolvePageRefs;
                    }})();
                </script>
            </body>
            </html>"##, content
        )
    }

}