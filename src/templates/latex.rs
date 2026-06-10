pub struct TemplateLaTex;

impl TemplateLaTex {

    pub fn base(&self, content: &str, header_footer: &str, css_style: &str) -> String {
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
                
                <style>{2}</style>
                
                <script type="module">
                    import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.esm.min.mjs';
                    mermaid.initialize({{ startOnLoad: true, theme: 'default' }});
                </script>
            </head>
            <body>
                <div class="document-container">{1}{0}</div>
                
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

                    // ---- \thepage: fill each .thepage span with its page number ----
                    (function () {{
                        var PAGE_HEIGHT_PX = 1122;
                        var BODY_OFFSET    = 40;

                        function pageOf(el) {{
                            var top = 0;
                            var e = el;
                            while (e) {{ top += e.offsetTop || 0; e = e.offsetParent; }}
                            return Math.floor(Math.max(0, top - BODY_OFFSET) / PAGE_HEIGHT_PX) + 1;
                        }}

                        function fillPageNums() {{
                            document.querySelectorAll(".thepage").forEach(function(el) {{
                                el.textContent = String(pageOf(el));
                            }});
                        }}

                        if (document.fonts && document.fonts.ready) {{
                            document.fonts.ready.then(fillPageNums);
                        }} else {{
                            window.addEventListener("load", fillPageNums);
                        }}

                        window.addEventListener("resize", function () {{ fillPageNums(); }});
                    }})();
                </script>
            </body>
            </html>"##, content, header_footer, css_style
        )
    }

}