use tar::{
    Archive,
    Builder,
};

use colored::*;
use serde_yaml::Value;

use flate2::{
    Compression,
    read::GzDecoder,
    write::GzEncoder,
};

use std::{
    env,
    io::Read,
    error::Error,
    collections::HashSet,

    fs::{
        self,
        File,
    },

    path::{
        Path,
        PathBuf,
    },
};

use crate::{
    args_cli::Flags,
    syntax::vars::Vars,
    configs::package::Package,
    syntax::blocks::readme_block::ReadMeBlock,

    ui::{
        ui_base::UI,
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },

    cmd::{
        copy::Copy,
        monset::Monset,
    },
};

pub struct Bundle;

impl Bundle {

    const ENTRY: &'static str = "main.mon";

    pub fn pack(&self) -> Result<PathBuf, Box<dyn Error>> {
        UI::section_header("Packing", "normal");

        let mon = Path::new(Self::ENTRY);
        if !mon.is_file() {
            return Err(
                format!("Entry list '{}' not found in the current directory.", Self::ENTRY).into()
            );
        }

        let dir = Path::new(".");
        let contents = fs::read_to_string(mon)?;

        let name = Package.name(Self::ENTRY)
            .map(|n| Self::slugify(&n))
            .filter(|n| !n.is_empty())
            .unwrap_or_else(|| "package".to_string());

        let bundle_name = match Package.version(Self::ENTRY).map(|v| Self::slugify(&v)).filter(|v| !v.is_empty()) {
            Some(version) => format!("{}-{}.scpkg", name, version),
            None => format!("{}.scpkg", name),
        };

        let output = PathBuf::from(&bundle_name);
        let encoder = GzEncoder::new(File::create(&output)?, Compression::default());
        let mut tar = Builder::new(encoder);

        tar.append_path_with_name(mon, Self::ENTRY)?;
        let mut count = 1;

        let manifest = dir.join("package.yml");
        if manifest.is_file() {
            tar.append_path_with_name(&manifest, "package.yml")?;
            count += 1;
        }

        if let Some(license) = Self::find_license(dir) {
            let license_name = license.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "LICENSE".to_string());

            tar.append_path_with_name(&license, &license_name)?;
            count += 1;
        }

        if let Some(readme) = Self::find_readme(dir) {
            let readme_name = readme.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "README.md".to_string());

            tar.append_path_with_name(&readme, &readme_name)?;
            count += 1;
        }

        for (path, rel) in Self::detect_lists(dir, &contents) {
            if rel == Self::ENTRY {
                continue;
            }

            tar.append_path_with_name(&path, &rel)?;
            count += 1;
        }

        tar.into_inner()?.finish()?;
        SuccessAlerts::packed(&bundle_name, count);

        Ok(output)
    }

    pub fn info(&self, bundle: &str) -> Result<(), Box<dyn Error>> {
        UI::section_header("Package Info", "normal");

        let bundle_path = Path::new(bundle);
        if !bundle_path.is_file() {
            return Err(format!("Bundle not found: {}", bundle).into());
        }

        let mut archive = Archive::new(GzDecoder::new(File::open(bundle_path)?));

        let mut files = Vec::new();
        let mut manifest = String::new();

        for item in archive.entries()? {
            let mut item = item?;
            let name = item.path()?.to_string_lossy().replace('\\', "/");

            if name == "package.yml" {
                item.read_to_string(&mut manifest)?;
            }

            files.push(name);
        }

        let data: Value = serde_yaml::from_str(&manifest).unwrap_or(Value::Null);

        for (label, key) in [
            ("Name", "name"),
            ("Version", "version"),
            ("Description", "description"),
            ("Author", "author"),
            ("License", "license"),
            ("Homepage", "homepage"),
        ] {
            let value = Self::manifest_field(&data, key).or_else(|| {
                if key == "author" { Self::manifest_field(&data, "authors") } else { None }
            });

            if let Some(value) = value {
                Self::row(label, &value);
            }
        }

        Self::row("Entry", Self::ENTRY);

        Self::row("Files", &files.len().to_string());
        for file in &files {
            println!("    {} {}", "-".dimmed(), file);
        }

        Ok(())
    }

    fn row(label: &str, value: &str) {
        let label = format!("{:<12}", format!("{}:", label));
        println!("  {} {}", label.blue().bold(), value);
    }

    fn manifest_field(data: &Value, key: &str) -> Option<String> {
        let value = &data[key];

        if let Some(text) = value.as_str() {
            return Some(text.to_string());
        }

        if let Some(items) = value.as_sequence() {
            let joined: Vec<String> = items.iter()
                .filter_map(|item| item.as_str().map(|s| s.to_string()))
                .collect();

            if !joined.is_empty() {
                return Some(joined.join(", "));
            }
        }

        None
    }

    pub async fn run(&self, bundle: &str, flags: &Flags) -> Result<(), Box<dyn Error>> {
        UI::section_header("Running", "normal");

        let dest = Self::unpack(bundle)?;
        Self::run_entry(&dest, Self::ENTRY, flags).await
    }

    fn unpack(bundle: &str) -> Result<PathBuf, Box<dyn Error>> {
        let bundle_path = Path::new(bundle);
        if !bundle_path.is_file() {
            return Err(format!("Bundle not found: {}", bundle).into());
        }

        let dest_name = bundle_path.file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "package".to_string());

        let dest = PathBuf::from(&dest_name);
        fs::create_dir_all(&dest)?;

        let mut archive = Archive::new(GzDecoder::new(File::open(bundle_path)?));
        archive.unpack(&dest)?;

        if !dest.join(Self::ENTRY).is_file() {
            return Err(
                format!("Bundle '{}' has no '{}' entry list.", bundle, Self::ENTRY).into()
            );
        }

        Ok(dest)
    }

    async fn run_entry(dest: &Path, entry: &str, flags: &Flags) -> Result<(), Box<dyn Error>> {
        let previous = env::current_dir()?;
        env::set_current_dir(dest)?;

        let monset = Monset::new(entry);
        let contents = monset.raw_contents().await.unwrap_or_default();

        let _ = monset.downloads(flags).await;
        let _ = monset.run_code(flags).await;
        ReadMeBlock.render_block_and_save_file(entry, flags).await;

        if let Err(err) = Copy::new(&contents).get() {
            ErrorsAlerts::generic(&err.to_string());
        }

        if let Err(err) = monset.server().await {
            ErrorsAlerts::generic(&err.to_string());
        }

        env::set_current_dir(previous)?;

        Ok(())
    }

    fn detect_lists(dir: &Path, contents: &str) -> Vec<(PathBuf, String)> {
        let output = Self::output_prefix(contents);

        let mut seen = HashSet::new();
        let mut lists = Vec::new();
        let mut stack = Vars.get_imports(contents);

        while let Some(source) = stack.pop() {
            if source.starts_with("http://")
                || source.starts_with("https://")
                || source.contains("..")
                || !source.ends_with(".mon")
            {
                continue;
            }

            let rel = source.replace('\\', "/");
            if Self::in_output(&rel, &output) {
                continue;
            }

            let path = dir.join(&source);
            if !path.is_file() || !seen.insert(rel.clone()) {
                continue;
            }

            if let Ok(child) = fs::read_to_string(&path) {
                stack.extend(Vars.get_imports(&child));
            }

            lists.push((path, rel));
        }

        lists
    }

    pub fn slugify(value: &str) -> String {
        let mut slug = String::new();
        let mut prev_dash = false;

        for ch in value.trim().chars() {
            if ch.is_ascii_alphanumeric() || ch == '.' || ch == '_' {
                slug.push(ch.to_ascii_lowercase());
                prev_dash = false;
            } else if !prev_dash {
                slug.push('-');
                prev_dash = true;
            }
        }

        slug.trim_matches('-').to_string()
    }

    fn find_license(dir: &Path) -> Option<PathBuf> {
        const NAMES: [&str; 6] = [
            "LICENSE", "LICENSE.txt", "LICENSE.md", "license", "license.txt", "COPYING",
        ];

        NAMES.iter()
            .map(|name| dir.join(name))
            .find(|path| path.is_file())
    }

    fn find_readme(dir: &Path) -> Option<PathBuf> {
        const NAMES: [&str; 4] = [
            "README.md", "README", "README.txt", "readme.md",
        ];

        NAMES.iter()
            .map(|name| dir.join(name))
            .find(|path| path.is_file())
    }

    fn output_prefix(contents: &str) -> Option<String> {
        let path = Vars.get_path(contents);
        let normalised = path.trim()
            .trim_end_matches(|c| c == '/' || c == '\\')
            .replace('\\', "/");

        if normalised.is_empty() || normalised == "." {
            None
        } else {
            Some(normalised)
        }
    }

    fn in_output(rel: &str, output: &Option<String>) -> bool {
        match output {
            Some(prefix) => rel == prefix || rel.starts_with(&format!("{}/", prefix)),
            None => false,
        }
    }

}
