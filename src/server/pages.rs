use crate::server::components::Components;

pub struct Pages;

impl Pages {

    fn render(&self, code: &str, heading: &str) -> String {
        let components = Components;

        format!(
            "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\">\
             <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\
             <link rel=\"icon\" href=\"https://scimon.dev/favicon.ico\">\
             <title>Scimon: {code}</title><style>{style}</style>{early}</head>\
             <body><div class=\"layout\">\
             <aside class=\"sidebar\">\
             <a class=\"logo\" href=\"/\">{logo}</a>\
             {toggle}\
             </aside>\
             <main class=\"main\"><h1>{heading}</h1>\
             <p><a href=\"/\">Back to index</a></p></main>\
             </div>\
             <script>{theme_js}</script>\
             <script src=\"https://unpkg.com/lucide@latest\"></script>\
             <script>lucide.createIcons();</script>\
             </body></html>",
            code = code,
            heading = heading,
            style = components.style(),
            early = components.theme_early(),
            logo = components.logo(),
            toggle = components.theme_toggle(),
            theme_js = components.theme_js(),
        )
    }

    pub fn not_found(&self) -> String {
        self.render("404", "404 Not Found")
    }

    pub fn forbiden(&self) -> String {
        self.render("403", "403 Forbidden")
    }

    pub fn method_not_allowed(&self) -> String {
        self.render("405", "405 Method Not Allowed")
    }

    pub fn internal_server_error(&self) -> String {
        self.render("500", "500 Internal Server Error")
    }

}
