use crate::server::components::Components;

pub struct Pages;

impl Pages {

    pub fn not_found(&self) -> String {
        format!(
            "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><title>404</title>\
             <style>{0}</style>{1}</head>\
             <body>{2}<a class=\"logo\" href=\"/\">{3}</a><h1>404 Not Found</h1>\
             <p><a href=\"/\">Back to index</a></p>\
             <script>{4}</script></body></html>",
            Components.style(),
            Components.theme_early(),
            Components.theme_toggle(),
            Components.logo(),
            Components.theme_js(),
        )
    }

    pub fn forbiden(&self) -> String {
        format!(
            "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><title>403</title>\
             <style>{0}</style>{1}</head>\
             <body>{2}<a class=\"logo\" href=\"/\">{3}</a><h1>403 Forbidden</h1>\
             <p><a href=\"/\">Back to index</a></p>\
             <script>{4}</script></body></html>",
            Components.style(),
            Components.theme_early(),
            Components.theme_toggle(),
            Components.logo(),
            Components.theme_js(),
        )
    }

    pub fn method_not_allowed(&self) -> String {
        format!(
            "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><title>405</title>\
             <style>{0}</style>{1}</head>\
             <body>{2}<a class=\"logo\" href=\"/\">{3}</a><h1>405 Method Not Allowed</h1>\
             <p><a href=\"/\">Back to index</a></p>\
             <script>{4}</script></body></html>",
            Components.style(),
            Components.theme_early(),
            Components.theme_toggle(),
            Components.logo(),
            Components.theme_js(),
        )
    }

    pub fn internal_server_error(&self) -> String {
        format!(
            "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><title>500</title>\
             <style>{0}</style>{1}</head>\
             <body>{2}<a class=\"logo\" href=\"/\">{3}</a><h1>500 Internal Server Error</h1>\
             <p><a href=\"/\">Back to index</a></p>\
             <script>{4}</script></body></html>",
            Components.style(),
            Components.theme_early(),
            Components.theme_toggle(),
            Components.logo(),
            Components.theme_js(),
        )
    }
    
}