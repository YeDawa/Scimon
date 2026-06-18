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

    pub fn folder_nav(&self, misc: &Misc, folders: &BTreeSet<String>, prefix: &str, has_scripts: bool, scripts_active: bool) -> String {
        if folders.is_empty() && !has_scripts {
            return String::new();
        }

        let mut nav = String::from("<nav class=\"folders\">");
        let root_active = if prefix.is_empty() && !scripts_active { " class=\"active\"" } else { "" };
        nav.push_str(&format!("<a{} href=\"/\">{} root</a>", root_active, Icons.icon("home")));

        for folder in folders {
            let depth = folder.matches('/').count();
            let name = folder.rsplit('/').next().unwrap_or(folder);
            
            let href = format!(
                "/{}/",
                folder.split('/').map(|s| misc.percent_encode(s)).collect::<Vec<_>>().join("/")
            );

            let active = if prefix == folder && !scripts_active { " class=\"active\"" } else { "" };
            nav.push_str(&format!(
                "<a{} style=\"padding-left:{:.1}rem\" href=\"{}\">{} {}/</a>",
                active,
                0.6 + depth as f64 * 0.9,
                href,
                Icons.icon("folder"),
                misc.html_escape(name)
            ));
        }

        if has_scripts {
            let active = if scripts_active { " class=\"active\"" } else { "" };
            nav.push_str(&format!(
                "<div class=\"separator\"></div><a{} href=\"{}\">{} Scripts</a>",
                active,
                Server::SCRIPTS_ROUTE,
                Icons.icon("file-code")
            ));
        }

        nav.push_str("</nav>");
        nav
    }

}