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
           r#"<!DOCTYPE html>
            <html lang="pt-BR">
            <head>
                <meta charset="UTF-8">
                <style>
                    body {{ font-family: 'Computer Modern', serif; font-size: 18px; line-height: 1.6; padding: 40px; max-width: 800px; margin: 0 auto; color: #333; }}
                    
                    .title-block {{ text-align: center; margin-bottom: 50px; padding-bottom: 20px; border-bottom: 2px solid #eee; }}
                    .title-block h1 {{ font-size: 2.2em; margin: 0 0 10px 0; color: #111; }}
                    .title-block .author {{ font-size: 1.2em; color: #555; font-style: italic; }}

                    h2 {{ border-bottom: 1px solid #ccc; padding-bottom: 5px; color: #111; margin-top: 40px; }}
                    h3 {{ color: #222; margin-top: 30px; }}
                    
                    /* Lists & TOC */
                    ul, ol {{ background: #fdfdfd; padding: 15px 40px; border: 1px solid #eee; border-radius: 5px; }}
                    .toc {{ background: #f9f9f9; padding: 20px; border-left: 4px solid #333; border-radius: 5px; margin: 15px 0; }}
                    .toc ul {{ border: none; background: transparent; padding: 0; list-style: none; }}
                    .toc li {{ margin-bottom: 5px; }}
                    .toc a {{ text-decoration: none; color: #0056b3; }}
                    .toc a:hover {{ text-decoration: underline; }}
                    
                    /* Math */
                    .math-inline {{ font-family: 'Cambria Math', 'Times New Roman', serif; font-style: italic; background: #f7f7f7; padding: 2px 6px; border-radius: 3px; white-space: nowrap; }}
                    .math-block {{ font-family: 'Cambria Math', 'Times New Roman', serif; font-size: 1.3em; text-align: center; margin: 30px 0; padding: 15px; background: #fcfcfc; border-left: 4px solid #0056b3; overflow-x: auto; font-style: italic; position: relative; }}
                    .eq-number {{ position: absolute; right: 20px; top: 50%; transform: translateY(-50%); font-size: 0.8em; font-style: normal; color: #555; }}
                    sup, sub {{ font-size: 0.75em; line-height: 0; position: relative; vertical-align: baseline; }}
                    sup {{ top: -0.5em; }} sub {{ bottom: -0.25em; }}
                    .latex-frac {{ display: inline-block; vertical-align: middle; margin: 0 0.2em; text-align: center; }}
                    .frac-num {{ display: block; border-bottom: 1px solid #333; padding: 0 0.2em; line-height: 1.2; }}
                    .frac-den {{ display: block; padding: 0 0.2em; line-height: 1.2; }}
                    
                    /* Visuals & Cross-Refs */
                    .latex-image {{ display: block; margin: 30px auto 10px; max-width: 100%; border-radius: 8px; box-shadow: 0 4px 15px rgba(0,0,0,0.1); }}
                    .caption {{ text-align: center; font-size: 0.9em; color: #666; margin-bottom: 30px; }}
                    .cross-ref {{ color: #d9534f; font-weight: bold; text-decoration: none; padding: 0 2px; }}
                    .cross-ref:hover {{ text-decoration: underline; }}
                    
                    .code-block {{ background: #282c34; color: #abb2bf; padding: 20px; border-radius: 8px; overflow-x: auto; font-family: 'Courier New', monospace; font-size: 0.9em; box-shadow: inset 0 2px 4px rgba(0,0,0,0.2); }}
                    .latex-table {{ border-collapse: collapse; margin: 20px auto; font-size: 0.95em; min-width: 50%; box-shadow: 0 0 20px rgba(0, 0, 0, 0.05); }}
                    .latex-table td {{ padding: 12px 15px; border: 1px solid #ddd; text-align: center; }}
                    .latex-table tr:nth-of-type(even) {{ background-color: #f9f9f9; }}
                    
                    .cite {{ color: #0056b3; text-decoration: none; font-weight: bold; padding: 0 2px; }}
                    .bibliography {{ background: #fdfdfd; padding: 20px 40px; border: 1px solid #eee; border-radius: 5px; }}
                </style>
                <script type="module">
                    import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.esm.min.mjs';
                    mermaid.initialize({{ startOnLoad: true, theme: 'default' }});
                </script>
            </head>
            <body>
                {}
            </body>
            </html>"#, content
        )
    }

}