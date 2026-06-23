use std::{
    io::Read,
    error::Error,

    fs::{
        self, 
        File,
    },

    path::{
        Path,
        PathBuf,
    },
};

pub struct Extract;

impl Extract {

    pub fn run(&self, archive_path: &str) -> Result<usize, Box<dyn Error>> {
        let path = Path::new(archive_path);
        let lower = archive_path.to_lowercase();

        let dest = self.dest_dir(path, &lower);
        fs::create_dir_all(&dest)?;

        if lower.ends_with(".zip") {
            self.unzip(path, &dest)
        } else if lower.ends_with(".tar.gz") || lower.ends_with(".tgz") {
            let file = File::open(path)?;
            self.untar(flate2::read::GzDecoder::new(file), &dest)
        } else if lower.ends_with(".tar") {
            self.untar(File::open(path)?, &dest)
        } else {
            Err(format!("Unsupported archive type: {}", archive_path).into())
        }
    }

    fn dest_dir(&self, path: &Path, lower: &str) -> PathBuf {
        let parent = path.parent().unwrap_or_else(|| Path::new("."));
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("archive");

        let base = if lower.ends_with(".tar.gz") {
            &name[..name.len() - 7]
        } else if lower.ends_with(".tgz") || lower.ends_with(".tar") || lower.ends_with(".zip") {
            &name[..name.len() - 4]
        } else {
            name
        };

        parent.join(base)
    }

    fn unzip(&self, path: &Path, dest: &Path) -> Result<usize, Box<dyn Error>> {
        let mut archive = zip::ZipArchive::new(File::open(path)?)?;
        let mut count = 0;

        for i in 0..archive.len() {
            let mut entry = archive.by_index(i)?;

            let Some(relative) = entry.enclosed_name() else {
                continue;
            };

            let out_path = dest.join(relative);
            if entry.is_dir() {
                fs::create_dir_all(&out_path)?;
            } else {
                if let Some(parent) = out_path.parent() {
                    fs::create_dir_all(parent)?;
                }

                let mut out = File::create(&out_path)?;
                std::io::copy(&mut entry, &mut out)?;
                count += 1;
            }
        }

        Ok(count)
    }

    fn untar<R: Read>(&self, reader: R, dest: &Path) -> Result<usize, Box<dyn Error>> {
        let mut archive = tar::Archive::new(reader);
        let mut count = 0;

        for entry in archive.entries()? {
            let mut entry = entry?;
            let is_file = entry.header().entry_type().is_file();

            if entry.unpack_in(dest)? && is_file {
                count += 1;
            }
        }

        Ok(count)
    }

}
