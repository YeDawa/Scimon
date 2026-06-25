use colored::*;

use std::{
    fs,
    error::Error,

    io::{
        self,
        Write,
    },

    path::{
        Path,
        PathBuf,
    },
};

use crate::{
    cmd::bundle::Bundle,
    consts::bundler::Bundler,

    ui::{
        ui_base::UI,
        success_alerts::SuccessAlerts,
    },
};

pub struct Init;

impl Init {

    pub fn create(&self) -> Result<(), Box<dyn Error>> {
        UI::section_header("Init", "normal");

        let (name, manifest) = Self::prompt_manifest();
        let folder = {
            let slug = Bundle::slugify(&name);

            if slug.is_empty() {
                "package".to_string()
            } else {
                slug
            }
        };

        let dir = PathBuf::from(&folder);
        fs::create_dir_all(&dir)?;

        Self::write(&dir.join("package.yml"), &manifest)?;
        Self::write(&dir.join("main.mon"), Bundler::ENTRY_PACKAGE)?;

        println!("\n  {} cd {}", "→".dimmed(), folder.blue().bold());

        Ok(())
    }

    fn prompt_manifest() -> (String, String) {
        let name = Self::ask("Package name", "my-package");

        let fields = [
            ("name", name.clone()),
            ("version", Self::ask("Version", "0.1.0")),
            ("description", Self::ask("Description", "A Scimon package.")),
            ("author", Self::ask("Author", "")),
            ("license", Self::ask("License", "MIT")),
            ("homepage", Self::ask("Homepage", "")),
            ("privacy", Self::ask("Privacy", "Public")),
        ];

        let mut manifest = String::from("# Scimon package metadata\n");
        for (key, value) in fields {
            if value.is_empty() {
                continue;
            }

            manifest.push_str(&format!("{}: \"{}\"\n", key, value.replace('"', "'")));
        }

        (name, manifest)
    }

    fn ask(label: &str, default: &str) -> String {
        let label = format!("{}:", label).blue().bold();

        if default.is_empty() {
            print!("  {} ", label);
        } else {
            print!("  {} {} ", label, format!("({})", default).dimmed());
        }

        io::stdout().flush().ok();

        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();
        let input = input.trim();

        if input.is_empty() {
            default.to_string()
        } else {
            input.to_string()
        }
    }

    fn write(path: &Path, content: &str) -> Result<(), Box<dyn Error>> {
        let label = path.to_string_lossy().replace('\\', "/");

        if path.exists() {
            SuccessAlerts::skipped(&label);
            return Ok(());
        }

        fs::write(path, content)?;
        SuccessAlerts::created(&label);

        Ok(())
    }

}
