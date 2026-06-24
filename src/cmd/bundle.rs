use glob::glob;
use regex::Regex;

use tar::{
    Header,
    Archive,
    Builder,
    EntryType,
};

use flate2::{
    Compression,
    read::GzDecoder,
    write::GzEncoder,
};

use std::{
    env,
    io::Write,
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
    cmd::monset::Monset,
    consts::bundler::Bundler,
    configs::package::Package,
    syntax::blocks::readme_block::ReadMeBlock,

    ui::{
        ui_base::UI,
        success_alerts::SuccessAlerts,
    },
};

pub struct Bundle;

impl Bundle {

    pub fn pack(&self, mon_path: &str) -> Result<PathBuf, Box<dyn Error>> {
        UI::section_header("Packing", "normal");

        let mon = Path::new(mon_path);
        if !mon.is_file() {
            return Err(format!("File not found: {}", mon_path).into());
        }

        let dir = mon.parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));

        let contents = fs::read_to_string(mon)?;

        let mon_name = mon.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "list.mon".to_string());

        let stem = mon.file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "package".to_string());

        let name = Package.name(mon_path).unwrap_or(stem);
        let bundle_name = match Package.version(mon_path) {
            Some(version) => format!("{}-{}.scmon", name, version),
            None => format!("{}.scmon", name),
        };

        let output = PathBuf::from(&bundle_name);
        let encoder = GzEncoder::new(File::create(&output)?, Compression::default());
        let mut tar = Builder::new(encoder);

        // The list lives at the archive root; `.entry` records which list is the
        // one to execute when the bundle is run or installed.
        tar.append_path_with_name(mon, &mon_name)?;
        Self::append_entry(&mut tar, &mon_name)?;
        let mut count = 1;

        let manifest = dir.join("package.yml");
        if manifest.is_file() {
            tar.append_path_with_name(&manifest, "package.yml")?;
            count += 1;
        }

        for (path, rel) in Self::detect_assets(dir, &contents) {
            if rel == mon_name || rel == "package.yml" {
                continue;
            }

            tar.append_path_with_name(&path, &rel)?;
            count += 1;
        }

        tar.into_inner()?.finish()?;
        SuccessAlerts::packed(&bundle_name, count);

        Ok(output)
    }

    // Installs a `.scmon` bundle: extracts it into a folder named after it and
    // immediately runs the entry list.
    pub async fn install(&self, bundle: &str, flags: &Flags) -> Result<(), Box<dyn Error>> {
        UI::section_header("Installing", "normal");

        let (dest, entry) = Self::unpack(bundle)?;
        SuccessAlerts::installed(&dest.to_string_lossy());

        Self::run_entry(&dest, &entry, flags).await
    }

    // Runs a `.scmon` bundle directly (`scimon run app.scmon`): extracts it and
    // immediately executes the entry list, like running a plain `.mon` file.
    pub async fn run(&self, bundle: &str, flags: &Flags) -> Result<(), Box<dyn Error>> {
        UI::section_header("Running", "normal");

        let (dest, entry) = Self::unpack(bundle)?;
        Self::run_entry(&dest, &entry, flags).await
    }

    // Extracts a bundle into a folder named after it and resolves the entry list
    // recorded in `.entry` (falling back to the first `.mon` found).
    fn unpack(bundle: &str) -> Result<(PathBuf, String), Box<dyn Error>> {
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

        let entry = Self::read_entry(&dest)
            .filter(|name| dest.join(name).is_file())
            .or_else(|| Self::find_list(&dest))
            .ok_or_else(|| format!("No entry list found in {}", bundle))?;

        Ok((dest, entry))
    }

    // Runs the entry list from inside the package directory so its assets resolve
    // at their original relative paths.
    async fn run_entry(dest: &Path, entry: &str, flags: &Flags) -> Result<(), Box<dyn Error>> {
        let previous = env::current_dir()?;
        env::set_current_dir(dest)?;

        let monset = Monset::new(entry);
        let _ = monset.downloads(flags).await;
        let _ = monset.run_code(flags).await;
        ReadMeBlock.render_block_and_save_file(entry, flags).await;

        env::set_current_dir(previous)?;

        Ok(())
    }

    // Writes the `.entry` pointer (the entry list's file name) into the archive.
    fn append_entry<W: Write>(tar: &mut Builder<W>, entry: &str) -> Result<(), Box<dyn Error>> {
        let data = entry.as_bytes();

        let mut header = Header::new_gnu();
        header.set_size(data.len() as u64);
        header.set_mode(0o644);
        header.set_entry_type(EntryType::Regular);
        header.set_cksum();

        tar.append_data(&mut header, ".entry", data)?;

        Ok(())
    }

    // Reads the `.entry` pointer written at pack time.
    fn read_entry(dir: &Path) -> Option<String> {
        fs::read_to_string(dir.join(".entry"))
            .ok()
            .map(|text| text.trim().to_string())
            .filter(|text| !text.is_empty())
    }

    fn detect_assets(dir: &Path, contents: &str) -> Vec<(PathBuf, String)> {
        let pattern = Regex::new(Bundler::ASSETS_PATTERNS).unwrap();

        let mut seen = HashSet::new();
        let mut assets = Vec::new();

        for caps in pattern.captures_iter(contents) {
            let value = caps.get(1)
                .or_else(|| caps.get(2))
                .map(|m| m.as_str().trim())
                .unwrap_or("");

            if value.is_empty()
                || value.starts_with("http://")
                || value.starts_with("https://")
                || value.contains("..")
            {
                continue;
            }

            if value.contains('*') || value.contains('?') || value.contains('[') {
                let glob_pattern = dir.join(value).to_string_lossy().to_string();

                if let Ok(paths) = glob(&glob_pattern) {
                    for entry in paths.flatten().filter(|p| p.is_file()) {
                        if let Ok(rel) = entry.strip_prefix(dir) {
                            let rel = rel.to_string_lossy().replace('\\', "/");
                            if seen.insert(rel.clone()) {
                                assets.push((entry, rel));
                            }
                        }
                    }
                }

                continue;
            }

            let path = dir.join(value);
            if path.is_file() {
                let rel = value.replace('\\', "/");
                if seen.insert(rel.clone()) {
                    assets.push((path, rel));
                }
            }
        }

        assets
    }

    // Returns the file name of the first `.mon` list found at the root of `dir`.
    fn find_list(dir: &Path) -> Option<String> {
        fs::read_dir(dir).ok()?
            .flatten()
            .map(|entry| entry.path())
            .find(|path| path.extension().map(|ext| ext == "mon").unwrap_or(false))
            .and_then(|path| path.file_name().map(|n| n.to_string_lossy().to_string()))
    }

}
