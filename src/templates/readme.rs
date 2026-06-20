pub struct TemplateReadMe;

impl TemplateReadMe {

    pub fn base(&self, page_title: &str, markdown_content: &str) -> String {
        format!(
            "<!doctype html>
            <html lang='en'>
            <head>
                <meta charset='UTF-8'>
                <title>{0}</title>
                <meta name='viewport' content='width=device-width, initial-scale=1.0'>
                <meta name='theme-color' content='#0c0c10'>
                <meta name='color-scheme' content='dark'>

                <!-- Favicon -->
                <link rel='icon' href='https://i.imgur.com/aHe6qpf.png' sizes='32x32'>

                <!-- Stylesheets -->
                <link rel='stylesheet' href='https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.2/css/all.min.css' crossorigin='anonymous'>
                <link rel='stylesheet' href='https://static.monlib.net/bundle.css'>
                <link rel='stylesheet' href='https://static.monlib.net/prism.css'>
            </head>
            <body>
                <div class='modal-mask' id='bibTextMaskModal'></div>

                <header>
                    <div class='label' id='headerLabel'></div>
                    <nav class='nav' aria-label='Page actions'>
                        <button type='button' class='nav-btn is-hidden' id='scrollToTopBtn' title='Back to top' aria-label='Back to top'>
                            <i class='fa-solid fa-chevron-up' aria-hidden='true'></i>
                        </button>
                        <button type='button' class='nav-btn is-hidden' id='scrollToRefsBtn' title='Citations' aria-label='Citations'>
                            <i class='fa-solid fa-book-open' aria-hidden='true'></i>
                        </button>
                        <button type='button' class='nav-btn is-hidden' id='scrollToDocsBtn' title='Documents' aria-label='Documents'>
                            <i class='fa-solid fa-file-lines' aria-hidden='true'></i>
                        </button>
                    </nav>
                </header>

                <div class='modal' id='bibTextModal'>
                    <div class='modal-content'>
                        <pre><code id='bibTextCode'></code></pre>
                    </div>
                </div>

                <div class='markdown-body'>{1}</div>

                <!-- Scripts -->
                <script src='https://cdn.jsdelivr.net/npm/citation-js'></script>
                <script src='https://cdn.jsdelivr.net/npm/mathjax@3.2.2/es5/tex-mml-chtml.min.js'></script>
                <script src='https://cdnjs.cloudflare.com/ajax/libs/mermaid/10.9.0/mermaid.min.js'></script>
                <script src='https://static.monlib.net/prism.js'></script>
                <script src='https://static.monlib.net/bundle.js'></script>
            </body>
            </html>", page_title, markdown_content
        )
    }

}