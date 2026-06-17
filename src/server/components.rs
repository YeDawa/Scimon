use crate::consts::server::Server;

pub struct Components;

impl Components {

    pub fn logo(&self) -> String {
        format!("<img src=\"{}\" alt=\"SciMon\" height=\"40\">", Server::LOGO_ROUTE)
    }
    
    pub fn theme_toggle(&self) -> String {
        Server::THEME_TOGGLE.to_string()
    }

    pub fn lightbox(&self) -> String {
        Server::LIGHTBOX_HTML.to_string()
    }

    pub fn lightbox_js(&self) -> String {
        Server::LIGHTBOX_JS.to_string()
    }

    pub fn theme_js(&self) -> String {
        Server::THEME_JS.to_string()
    }

    pub fn style(&self) -> String {
        Server::STYLE.to_string()
    }

    pub fn theme_early(&self) -> String {
        Server::THEME_EARLY.to_string()
    }

}