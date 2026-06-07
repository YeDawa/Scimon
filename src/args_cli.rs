use clap::{
    Parser, 
    Subcommand
};

#[derive(Clone)]
#[derive(Parser)]
#[command(author, version, about)]
pub struct Flags {
    #[arg(long, global = true)]
    /// Ignore PDF files
    pub no_ignore: bool,

    #[arg(long, global = true)]
    /// Disable the !open_link directive
    pub no_open_link: bool,

    #[arg(long, global = true)]
    /// Disable the !readme directive
    pub no_readme: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Clone, Subcommand)]
pub enum Commands {
    /// Execute a list of tasks or run a specific task
    Run {
        /// File or task to be executed
        file: String,
    },

    /// Get datasets of the links from the web
    Pull {
        /// File or task to be executed
        file: String,
    },

    /// Get datasets of the links from the web
    Push {
        /// File or task to be executed
        file: String,
    },

    /// Scraping the web page for list of documents
    Scrape {
        /// Url to scrape
        url: String,
    },

    /// Option's for the Scimon CLI
    Options {
        /// Options for the Scimon CLI
        options: String,
    },

    /// Monlib Authentification
    Auth {
        /// Authentification for Monlib
        option: String,
    },

    /// Sync setting's file with the Monlib
    Settings {
        /// Pull or push settings file
        cmd: String,
    },
}
