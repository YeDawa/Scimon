use walkdir::WalkDir;

use std::{
    fs,
    path::Path,
    io::Result as IoResult,
};

use crate::{
    syntax::vars::Vars,

    ui::{
        ui_base::UI,
        copy_alerts::CopyAlerts,
    },
};

pub struct Copy {
    contents: String,
}

impl Copy {

    pub fn new(contents: &str) -> Self {
        Self {
            contents: contents.to_string(),
        }
    }

    pub fn get(&self) -> IoResult<()> {
        if let Some(destination) = Vars.get_copy(&self.contents) {
            let source = Vars.get_path(&self.contents);
            let source_path = Path::new(&source);

            if !source_path.exists() {
                return Ok(());
            }

            UI::section_header("Copying files", "normal");

            let dest_root = Path::new(&destination);
            fs::create_dir_all(dest_root)?;

            for entry in WalkDir::new(source_path) {
                let entry = entry?;
                let path = entry.path();
                let relative = path.strip_prefix(source_path).unwrap();

                if relative.as_os_str().is_empty() {
                    continue;
                }

                let target = dest_root.join(relative);

                if path.is_dir() {
                    fs::create_dir_all(&target)?;
                } else {
                    if let Some(parent) = target.parent() {
                        fs::create_dir_all(parent)?;
                    }

                    fs::copy(path, &target)?;

                    CopyAlerts::copied(
                        path.to_str().unwrap_or(""),
                        target.to_str().unwrap_or(""),
                    );
                }
            }

            CopyAlerts::completed(&destination);
        }

        Ok(())
    }

}
