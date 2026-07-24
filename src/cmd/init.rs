use colored::*;
use chrono::Local;

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
    consts::addons::Addons,
    consts::bundler::Bundler,

    ui::{
        ui_base::UI,
        success_alerts::SuccessAlerts,
    },
};

struct Answers {
    name: String,
    description: String,
    author: String,
    license: String,
    body: String,
}

pub struct Init;

impl Init {

    pub async fn create(&self) -> Result<(), Box<dyn Error>> {
        UI::section_header("Init", "normal");

        let answers = Self::prompt();

        let folder = {
            let slug = Bundle::slugify(&answers.name);
            if slug.is_empty() { "package".to_string() } else { slug }
        };

        let dir = PathBuf::from(&folder);
        fs::create_dir_all(&dir)?;

        Self::write(&dir.join("package.yml"), &answers.body)?;
        Self::write(&dir.join("main.mon"), Bundler::ENTRY_PACKAGE)?;
        Self::write(&dir.join("README.md"), &format!("# {}\n\n{}\n", answers.name, answers.description))?;

        if !answers.license.is_empty() {
            match Self::license_text(&answers.license, &answers.author).await {
                Some(text) => Self::write(&dir.join("LICENSE"), &text)?,
                None => println!(
                    "  {} could not fetch '{}' license text; skipping LICENSE",
                    "!".yellow().bold(),
                    answers.license,
                ),
            }
        }

        println!("\n  {} cd {}", "→".dimmed(), folder.blue().bold());

        Ok(())
    }

    fn prompt() -> Answers {
        let name = Self::ask("Package name", "my-package");
        let description = Self::ask("Description", "A Scimon package.");
        let version = Self::ask("Version", "0.1.0");
        let author = Self::ask("Author", "");
        let license = Self::ask("License", "MIT");
        let homepage = Self::ask("Homepage", "");
        let privacy = Self::ask("Privacy", "Public");

        let fields = [
            ("name", name.as_str()),
            ("version", version.as_str()),
            ("description", description.as_str()),
            ("author", author.as_str()),
            ("license", license.as_str()),
            ("homepage", homepage.as_str()),
            ("privacy", privacy.as_str()),
        ];

        let mut body = String::from("# Scimon package metadata\n");
        for (key, value) in fields {
            if value.is_empty() {
                continue;
            }

            body.push_str(&format!("{}: \"{}\"\n", key, value.replace('"', "'")));
        }

        Answers { name, description, author, license, body }
    }

    async fn license_text(license: &str, author: &str) -> Option<String> {
        let url = Addons::SPDX_LICENSE_TEXT.replace("%s", license);

        let response = reqwest::get(&url).await.ok()?;
        if !response.status().is_success() {
            return None;
        }

        let text = response.text().await.ok()?;

        let year = Local::now().format("%Y").to_string();
        let holder = if author.is_empty() { "the authors" } else { author };

        let text = text
            .replace("<year>", &year)
            .replace("[yyyy]", &year)
            .replace("<copyright holders>", holder)
            .replace("<copyright holder>", holder)
            .replace("[name of copyright owner]", holder)
            .replace("<name of author>", holder)
            .replace("<owner>", holder);

        Some(text)
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
