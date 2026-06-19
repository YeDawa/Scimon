extern crate open;

use serde_yaml::Value::String as SerdeValue;

use std::{
    env,
    sync::Once,
    path::PathBuf,
    io::Error as IoError,
};

use crate::{
    consts::folders::Folders,
    configs::settings::Settings,
};

pub struct Env;

impl Env {
    
    pub fn env_var(&self, key: &str) -> String {
        let load_env: Once = Once::new();

        load_env.call_once(|| {
            dotenvy::from_path(
                Folders::APP_FOLDER.join(".env")
            ).ok();
        });
    
        env::var(key).unwrap_or_else(|_| panic!("{} not set", key))
    }
    
    pub fn open_env_file(&self) -> Result<(), IoError> {
        let app_folder = &*Folders::APP_FOLDER;
        let env_path: PathBuf = app_folder.join(".env");

        if let SerdeValue(editor) = &Settings.get(
            "general.default_text_editor", "STRING"
        ) {
            open::with(env_path, editor)?;
        }
        
        Ok(())
    }
  
}
