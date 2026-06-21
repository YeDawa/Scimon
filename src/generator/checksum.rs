use walkdir::WalkDir;

use std::{
    fs::File,
    path::Path,
    error::Error,

    io::{
        Read,
        Write,
        Result as IoResult,
    },
};

use sha2::{
    Digest,
    Sha256,
};

use crate::{
    syntax::vars::Vars,
    utils::file::FileUtils,
    
    ui::{
        ui_base::UI, 
        checksum_alerts::ChecksumAlerts,
    }
};

pub struct Checksum {
    contents: Option<String>,
}

impl Checksum {

    pub fn new(contents: Option<String>) -> Self {
        Self {
            contents,
        }
    }

    pub fn hash(&self, file_path: &str) -> Result<String, Box<dyn Error>> {
        let mut file = File::open(file_path)?;
        let mut hasher = Sha256::new();

        let mut buffer = [0; 1024];
        
        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 { break; }
            hasher.update(&buffer[..bytes_read]);
        }
    
        let hash = hasher.finalize();
        let hash_string: String = hash.iter()
            .map(|byte| format!("{:02x}", byte))
            .collect();

        Ok(hash_string)
    }
    
    pub fn files(&self) -> IoResult<()> {
        UI::section_header("Hashes files", "normal");

        let contents = self.contents.as_deref().unwrap_or("");
        let folder_path = Vars.get_path(contents);

        let checksum_filename = format!(
            "{}{}", folder_path, FileUtils.replace_extension(
                &folder_path.replace("/", ""), "sha256"
            )
        );

        let mut checksum_file = File::create(&checksum_filename)?;

        for entry in WalkDir::new(&folder_path) {
            let entry = entry?;
            let path = entry.path();
            let name = path.strip_prefix(Path::new(&folder_path)).unwrap();

            let is_target = path.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| {
                    let ext = ext.to_lowercase();
                    ext == "pdf" || ext == "epub"
                })
                .unwrap_or(false);

            if is_target {
                let file = name.to_str().unwrap();
                let path = path.to_str().unwrap();
                let hash = self.hash(path).unwrap();

                writeln!(checksum_file, "{}  {}", hash, file)?;
                ChecksumAlerts::hash(file, &hash);
            }
        }

        ChecksumAlerts::checksum_file(&checksum_filename);
        Ok(())
    }

}
