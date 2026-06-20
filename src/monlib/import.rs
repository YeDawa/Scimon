use std::env;

use crate::{
    args_cli::Flags,
    syntax::vars::Vars,
    monlib::pull::MonlibPull,

    consts::{
        addons::Addons,
        folders::Folders,
    },

    ui::{
        ui_base::UI,
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },
};

pub struct MonlibImport;

impl MonlibImport {

    fn has_api_key() -> bool {
        dotenvy::from_path(
            Folders::APP_FOLDER.join(".env")
        ).ok();

        env::var(Addons::MONLIB_API_ENV)
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false)
    }

    // Resolves every `import "package"` from the list and runs each Monlib
    // package before the main one, in declaration order.
    pub async fn run(&self, contents: &str, flags: &Flags) {
        let imports = Vars.get_imports(contents);

        if imports.is_empty() {
            return;
        }

        UI::section_header("imports", "normal");

        if !Self::has_api_key() {
            ErrorsAlerts::generic(&format!(
                "{} is not set; cannot resolve imported packages from Monlib.",
                Addons::MONLIB_API_ENV
            ));

            return;
        }

        for package in imports {
            SuccessAlerts::import(&package);
            let _ = MonlibPull.pull(&package, flags).await;
        }
    }

}
