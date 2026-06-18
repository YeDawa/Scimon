use std::collections::BTreeSet;

use crate::{
    consts::server::Server,

    server::{
        misc::Misc,
        icons::Icons,
    },
};

pub struct Components;

impl Components {

    pub fn logo(&self) -> String {
        format!("<img src=\"{}\" alt=\"SciMon\" height=\"40\">", Server::LOGO_PNG)
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

    pub fn table_js(&self) -> String {
        Server::TABLE_JS.to_string()
    }

    pub fn search_js(&self) -> String {
        Server::SEARCH_JS.to_string()
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

    // Sidebar navigation. `active` is the key of the current view: "" (root),
    // a folder prefix, "scripts", or "checksum".
    pub fn folder_nav(&self, misc: &Misc, folders: &BTreeSet<String>, active: &str, has_scripts: bool) -> String {
        let cls = |on: bool| if on { " class=\"active\"" } else { "" };

        let mut nav = String::from("<nav class=\"folders\">");
        nav.push_str(&format!("<a{} href=\"/\">{} root</a>", cls(active.is_empty()), Icons.icon("home")));

        for folder in folders {
            let depth = folder.matches('/').count();
            let name = folder.rsplit('/').next().unwrap_or(folder);

            let href = format!(
                "/{}/",
                folder.split('/').map(|s| misc.percent_encode(s)).collect::<Vec<_>>().join("/")
            );

            nav.push_str(&format!(
                "<a{} style=\"padding-left:{:.1}rem\" href=\"{}\">{} {}/</a>",
                cls(active == folder),
                0.6 + depth as f64 * 0.9,
                href,
                Icons.icon("folder"),
                misc.html_escape(name)
            ));
        }

        nav.push_str("<div class=\"separator\"></div>");

        if has_scripts {
            nav.push_str(&format!(
                "<a{} href=\"{}\">{} Scripts</a>",
                cls(active == "scripts"),
                Server::SCRIPTS_ROUTE,
                Icons.icon("file-code")
            ));
        }

        nav.push_str(&format!(
            "<a{} href=\"{}\">{} Checksum</a>",
            cls(active == "checksum"),
            Server::CHECKSUM_ROUTE,
            Icons.icon("shield-check")
        ));

        nav.push_str("</nav>");
        nav
    }

    pub fn checksum_js(&self) -> String {
        Server::CHECKSUM_JS.to_string()
    }

}