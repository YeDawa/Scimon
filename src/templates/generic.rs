pub struct TemplateGeneric;

impl TemplateGeneric {

    pub fn base(&self, css_style: &str, html_content: &str) -> String {
        format!(r#"
            <html>
            <head>
                <meta charset="utf-8">
                <style>{}</style>
            </head>
            <body class='markdown-body'>{}</body>
            </html>
        "#, css_style, html_content)
    }

}