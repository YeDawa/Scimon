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
    syntax::vars::Vars,
    cmd::monset::Monset,
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
            Some(version) => format!("{}-{}.scpkg", name, version),
            None => format!("{}.scpkg", name),
        }.to_lowercase();

        let output = PathBuf::from(&bundle_name);
        let encoder = GzEncoder::new(File::create(&output)?, Compression::default());
        let mut tar = Builder::new(encoder);

        // The entry list lives at the archive root; `.entry` records which list
        // is the one to execute when the bundle is run or installed.
        tar.append_path_with_name(mon, &mon_name)?;
        Self::append_entry(&mut tar, &mon_name)?;
        let mut count = 1;

        // The manifest and the license describe the package.
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

        // Every `.mon` list reachable through `import` directives.
        for (path, rel) in Self::detect_lists(dir, &contents) {
            if rel == mon_name {
                continue;
            }

            tar.append_path_with_name(&path, &rel)?;
            count += 1;
        }

        tar.into_inner()?.finish()?;
        SuccessAlerts::packed(&bundle_name, count);

        Ok(output)
    }

    // Resolves the entry list to pack: the given file, or the one recorded in a
    // project-level `.entry` file (created by `scimon init`).
    pub fn resolve_entry(&self, file: Option<String>) -> Option<String> {
        file.or_else(|| Self::read_entry(Path::new(".")))
    }

    // Runs a `.scpkg` bundle directly (`scimon run app.scpkg`): extracts it and
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

    // Collects the `.mon` lists reachable from the entry through `import`
    // directives (transitively), skipping anything inside the output directory.
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

    // Finds a license file next to the entry list, if any.
    fn find_license(dir: &Path) -> Option<PathBuf> {
        const NAMES: [&str; 6] = [
            "LICENSE", "LICENSE.txt", "LICENSE.md", "license", "license.txt", "COPYING",
        ];

        NAMES.iter()
            .map(|name| dir.join(name))
            .find(|path| path.is_file())
    }

    // The list's output directory (`path` directive), normalised without a
    // trailing slash; `None` when it is the current directory.
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

    // True when a relative path sits inside the list's output directory.
    fn in_output(rel: &str, output: &Option<String>) -> bool {
        match output {
            Some(prefix) => rel == prefix || rel.starts_with(&format!("{}/", prefix)),
            None => false,
        }
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
