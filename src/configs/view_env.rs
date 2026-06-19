use std::{
    path::PathBuf,
    fs::read_to_string,
};

use crate::consts::folders::Folders;

pub struct ViewEnv;

impl ViewEnv {

    fn header_text(&self) -> (String, String) {
        ("Name".to_string(), "Value".to_string())
    }

    fn header_size(&self) -> (usize, usize) {
        let (name, value) = self.header_text();
        (name.len(), value.len())
    }
    
    pub fn table(&self) {
        let app_folder = &*Folders::APP_FOLDER;
        let env_path: PathBuf = app_folder.join(".env");
        let content = read_to_string(env_path).unwrap_or_else(|_| "".to_string());

        let (lbl_name, lbl_value) = self.header_text();
        let (mut max_name_len, mut max_value_len) = self.header_size();

        let pairs: Vec<(String, String)> = content
            .lines()
            .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
            .filter_map(|line| {
                let parts: Vec<&str> = line.splitn(2, '=').collect();

                if parts.len() == 2 {
                    let name = parts[0].trim().to_string();
                    let mut value = parts[1].trim().to_string().replace('"', "");

                    if value.is_empty() {
                        value = "None".to_string();
                    }

                    max_name_len = max_name_len.max(name.len());
                    max_value_len = max_value_len.max(value.len());

                    Some((name, value))
                } else {
                    None
                }
            }).collect();

        let table_width = max_name_len + max_value_len + 7;

        println!("{:─<1$}", "", table_width);
        println!("| {:<width_name$} | {:<width_value$} |", lbl_name, lbl_value, width_name=max_name_len, width_value=max_value_len);
        println!("{:─<1$}", "", table_width);

        for (name, value) in pairs {
            println!("| {:<width_name$} | {:<width_value$} |", name, value, width_name=max_name_len, width_value=max_value_len);
        }

        println!("{:─<1$}", "", table_width);
    }
  
}
