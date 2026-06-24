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

    #[arg(long, global = true)]
    /// Disable the secure mode for the running of code
    pub no_secure: bool,

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

    /// Validate a list's syntax without downloading or running anything
    Check {
        /// File to validate
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

    /// Pack a list and its assets into a distributable .scmon bundle
    Pack {
        /// Entry list file to pack
        file: String,
    },

    /// Install a .scmon bundle: extract it and run the entry list
    Install {
        /// Bundle file (.scmon) to install
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

    /// Compile a LaTeX file into PDF
    Compile {
        /// LaTeX file to be compiled
        file: String,

        /// Output file for the compiled PDF
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Serve generated files over HTTP
    Serve {
        /// Directory to serve (defaults to the Scimon downloads folder)
        path: Option<String>,

        /// Port to listen on
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
}
