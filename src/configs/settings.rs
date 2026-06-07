use serde_yaml::Value::String as SerdeValue;

use std::{
    error::Error,

    fs::{
        File,
        write,
    },

    io::{
        Read,
        Error as IoError,
    },
};

use serde_yaml::{
    Value,
    from_str,
};

use crate::consts::{
    global::Global,
    folders::Folders,
};

pub struct Settings;

impl Settings {

    fn is_valid(&self, prop: &str, value: &Value, data_type: &str) -> Result<Value, Box<dyn Error>> {
        let value_type = match value {
            Value::String(_) => "STRING",
            Value::Number(_) => "INT",
            Value::Bool(_) => "BOOLEAN",
            Value::Sequence(_) => "LIST",
            _ => {
                return Err(
                    format!(
                        "Invalid type for property '{}'", prop
                    ).into()
                );
            }
        };

        if value_type != data_type {
            return Err(
                format!(
                    "The '{}' configuration is invalid. Expected type {}, but instead a {} was passed.",
                    prop, data_type, value_type
                ).into()
            );
        }

        Ok(value.clone())
    }

    fn get_value(&self, prop: &str, data_type: &str) -> Result<Value, Box<dyn Error>> {
        let mut file = File::open(&*Folders::SETTINGS_FILE)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let data: Value = from_str(&contents)?;

        let mut value = &data;
        let property_parts: Vec<&str> = prop.split('.').collect();

        for part in &property_parts {
            value = &value[part];
        }

        self.is_valid(prop, value, data_type)
    }

    pub fn get(&self, prop: &str, data_type: &str) -> Value {
        match self.get_value(prop, data_type) {
            Ok(value) => value,

            Err(e) => {
                eprintln!("{}", e);
                Value::Null
            }
        }
    }

    pub fn open_settings_file(&self) -> Result<(), IoError> {
        let app_folder = &*Folders::APP_FOLDER;
        let app_name = Global::APP_NAME;

        let settings_path = app_folder.join(
            format!("{}.yml", app_name.to_lowercase())
        );

        if let SerdeValue(editor) = self.get(
            "general.default_text_editor", "STRING"
        ) {
            open::with(settings_path, editor)?;
        }
        
        Ok(())
    }

    pub fn write_file(&self, content: &str) -> Result<(), Box<dyn Error>> {
        let app_folder = &*Folders::APP_FOLDER;
        let app_name = Global::APP_NAME;

        let settings_path = app_folder.join(
            format!("{}.yml", app_name.to_lowercase())
        );

        write(settings_path, content)?;
        Ok(())
    }

}
