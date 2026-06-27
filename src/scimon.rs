use clap::Parser;
use std::error::Error;

use crate::{
    args_cli::*,
    server::serve::Serve,
    addons::scrape::Scrape,
    system::shutdown::Shutdown,
    syntax::validator::Validator,
    syntax::blocks::readme_block::ReadMeBlock,

    ui::{
        ui_base::UI,
        syntax_alerts::SyntaxAlerts,
        errors_alerts::ErrorsAlerts,
    },

    cmd::{
        copy::Copy,
        init::Init,
        bundle::Bundle,
        monset::Monset,
        compile::Compile,
    },
    
    monlib::{
        pull::MonlibPull,
        push::MonlibPush,
        sync::MonlibSync,
        logout::MonlibLogout,
    },

    configs::{
        env::Env,
        view_env::ViewEnv,
        settings::Settings,
        write_env::WriteEnv,
        configs_files::DownloadConfigsFiles,
    },
};

pub struct Scimon;

impl Scimon {

    async fn options(&self, options: &str) -> Result<(), Box<dyn Error>> {
        match options {
            "view-env" => ViewEnv.table(),
            "open-env" => Env.open_env_file()?,
            "open-settings" => Settings.open_settings_file()?,
            "write-env" => WriteEnv::new(None, None).add_env_var()?,
            "download-env" => DownloadConfigsFiles.env_file(true, true).await?,
            "download-settings" => DownloadConfigsFiles.settings_file(true, true).await?,
            _ => (),
        };

        Ok(())
    }

    fn monlib(&self, options: &str) -> Result<(), Box<dyn Error>> {
        match options {
            "login" => println!("monlib login"),
            "logout" => MonlibLogout.logout(),
            _ => (),
        };

        Ok(())
    }

    async fn sync(&self, cmd: &str) -> Result<(), Box<dyn Error>> {
        match cmd {
            "pull" => MonlibSync.pull().await?,
            "push" => MonlibSync.push().await?,
            _ => "Invalid command".to_string(),
        };

        Ok(())
    }

    pub async fn init(&self) {
        // Register the single Ctrl+C handler up front so every step can stop
        // gracefully.
        Shutdown.init();

        let (print, force_mode) = (false, false);

        if let Err(err) = DownloadConfigsFiles.env_file(print, force_mode).await {
            ErrorsAlerts::generic(&err.to_string());
        }

        if let Err(err) = DownloadConfigsFiles.settings_file(print, force_mode).await {
            ErrorsAlerts::generic(&err.to_string());
        }

        let flags = Flags::parse();
        let flags_clone = flags.clone();

        if let Some(command) = flags.command {
            match command {
                Commands::Run { file } => {
                    UI::header();

                    if file.ends_with(".scpkg") {
                        if let Err(err) = Bundle.run(&file, &flags_clone).await {
                            ErrorsAlerts::generic(&err.to_string());
                        }
                    } else {
                        let monset = Monset::new(&file);

                        let contents = monset.raw_contents().await.unwrap_or_default();

                        if let Some(err) = Validator.check(&contents) {
                            SyntaxAlerts::error(err.line, &err.content, &err.message);
                        } else {
                            let _ = monset.downloads(&flags_clone).await;
                            let _ = monset.run_code(&flags_clone).await;
                            ReadMeBlock.render_block_and_save_file(&file, &flags_clone).await;

                            if let Err(err) = Copy::new(&contents).get() {
                                ErrorsAlerts::generic(&err.to_string());
                            }

                            if let Err(err) = monset.server().await {
                                ErrorsAlerts::generic(&err.to_string());
                            }
                        }
                    }
                },

                Commands::Check { file } => {
                    UI::header();

                    let monset = Monset::new(&file);
                    let contents = monset.raw_contents().await.unwrap_or_default();
                    
                    if let Some(err) = Validator.check(&contents) {
                        SyntaxAlerts::error(err.line, &err.content, &err.message);
                        std::process::exit(1);
                    } else {
                        SyntaxAlerts::valid(&file);
                    }
                },

                Commands::Pull { package } => {
                    UI::header();
                    let _ = MonlibPull.pull(&package, &flags_clone).await;
                },

                Commands::Push => {
                    UI::header();
                    let _ = MonlibPush.push().await;
                },

                Commands::Init => {
                    UI::header();
                    if let Err(err) = Init.create().await {
                        ErrorsAlerts::generic(&err.to_string());
                    }
                },

                Commands::Pack => {
                    UI::header();
                    if let Err(err) = Bundle.pack() {
                        ErrorsAlerts::generic(&err.to_string());
                    }
                },

                Commands::Info { file } => {
                    UI::header();
                    if let Err(err) = Bundle.info(&file) {
                        ErrorsAlerts::generic(&err.to_string());
                    }
                },

                Commands::Scrape { url } => {
                    UI::header();
                    let _ = Scrape.get(&flags_clone, &url).await;
                },

                Commands::Options { options } => {
                    let _ = self.options(&options).await;
                },

                Commands::Auth { option } => {
                    let _ = self.monlib(&option);
                },

                Commands::Settings { cmd } => {
                    let _ = self.sync(&cmd).await;
                },

                Commands::Compile { file, output } => {
                    if let Err(err) = Compile::new(&file, output).compile().await {
                        ErrorsAlerts::generic(&err.to_string());
                    }
                },

                Commands::Serve { path, port } => {
                    UI::header();
                    if let Err(err) = Serve::new(path, port).run() {
                        ErrorsAlerts::generic(&err.to_string());
                    }
                },
            }
        }
    }
    
}
