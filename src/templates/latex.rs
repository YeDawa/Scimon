pub struct TemplateLaTex;

impl TemplateLaTex {

    pub fn base(&self, content: &str, header_footer: &str, css_style: &str, js_script: &str) -> String {
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
                <script>
                    window.MathJax = {{
                        loader: {{
                            load: ['[tex]/mhchem', '[tex]/physics']
                        }},
                        tex: {{
                            inlineMath: [['\\(', '\\)'], ['$', '$']],
                            displayMath: [['\\[', '\\]'], ['$$', '$$']],
                            processEscapes: true,
                            packages: {{'[+]': ['mhchem', 'physics']}}
                        }},
                        options: {{
                            skipHtmlTags: ['script', 'noscript', 'style', 'textarea', 'pre']
                        }}
                    }};
                </script>

                <script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-chtml.js"></script>
                <style>{2}</style>
                <script type="module">
                    import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.esm.min.mjs';
                    mermaid.initialize({{ startOnLoad: true, theme: 'default' }});
                </script>
            </head>
            <body>
                <div class="document-container">{1}{0}</div>
                <script>{3}</script>
            </body>
            </html>"##, content, header_footer, css_style, js_script
        )
    }

}