extern crate chrono;

use regex::Regex;

use std::{
    error::Error,

    process::{
        Stdio,
        Command,
    },
};

use crate::{
    args_cli::Flags,
    consts::folders::Folders,
    system::plataforms::Plataforms,
    regexp::regex_core::CoreRegExp,

    ui::{
        security_alerts::SecurityAlerts,
        errors_commands_alerts::ErrorsCommandsAlerts,
    },

    utils::{
        remote::Remote,
        file::FileUtils,
    },

    security::{
        entropy::Entropy,
        security_rules::SecurityRules,
    },
};

pub struct Scripts;

impl Scripts {
    
    fn exec(&self, line: &str, program: &str) -> Result<(), Box<dyn Error>> {
        let language = Plataforms::get_bin_name(program);

        let line_cleanned = Regex::new(
            CoreRegExp::CLEAN_LINE
        ).unwrap().replace_all(
            line, ""
        ).to_string();

        let output = Command::new(&language)
            .arg(line_cleanned)
            .stdout(Stdio::piped())
            .output()?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("{}", stdout);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            ErrorsCommandsAlerts::executing(&stderr);
        }

        Ok(())
    }

    pub async fn read(&self, line_trimmed: &str, flags: &Flags) -> Result<(), Box<dyn Error>> {
        if line_trimmed.len() >= 3 {
            let script = if line_trimmed.starts_with("http") {
                let path = Folders::SCRIPTS_FOLDER.to_str().unwrap_or_default().to_string();

                FileUtils.create_path(&path);
                Remote.download(line_trimmed, &path).await?
            } else {
                line_trimmed.to_string()
            };

            if !flags.no_secure {
                if let Ok((entropy, is_high)) = Entropy.calculate(script.as_bytes()) {
                    if is_high {
                        SecurityAlerts::high_entropy(&script, entropy);
                        return Ok(());
                    }
                }

                if !SecurityRules.scan_script(&script).await? {
                    return Ok(());
                }
            }

            if script.ends_with(".py") {
                self.exec(&script, "python")?;
            } else if script.ends_with(".js") || script.ends_with(".mjs") || script.ends_with(".cjs") || script.ends_with(".jsx") {
                self.exec(&script, "node")?;
            } else if script.ends_with(".ts") || script.ends_with(".tsx") {
                self.exec(&script, "tsc")?;
            } else {
                ErrorsCommandsAlerts::unsupported(&script);
            }
        }

        Ok(())
    }

}
