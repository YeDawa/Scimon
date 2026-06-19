use clap::Parser;
use std::error::Error;

use crate::{
    args_cli::*,
    server::serve::Serve,
    addons::scrape::Scrape,
    syntax::validator::Validator,
    syntax::blocks::ai_block::AiBlock,
    syntax::blocks::readme_block::ReadMeBlock,

    ui::{
        ui_base::UI,
        syntax_alerts::SyntaxAlerts,
        errors_alerts::ErrorsAlerts,
    },

    cmd::{
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
                    let monset = Monset::new(&file);

                    let contents = monset.raw_contents().await.unwrap_or_default();

                    if let Some(err) = Validator.check(&contents) {
                        SyntaxAlerts::error(err.line, &err.content, &err.message);
                    } else {
                        if Monset::has_downloads_block(&contents) {
                            let _ = monset.downloads(&flags_clone).await;
                        }

                        let _ = monset.run_code(&flags_clone).await;
                        AiBlock.generate_and_save_files(&contents).await;
                        ReadMeBlock.render_block_and_save_file(&file, &flags_clone).await;

                        if let Err(err) = monset.server().await {
                            ErrorsAlerts::generic(&err.to_string());
                        }
                    }
                },

                Commands::Pull { file } => {
                    UI::header();
                    let _ = MonlibPull.pull(&file, &flags_clone).await;
                },

                Commands::Push { file } => {
                    UI::header();
                    let _ = MonlibPush.push(&file).await;
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
