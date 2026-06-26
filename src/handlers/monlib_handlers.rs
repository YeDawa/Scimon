use crate::configs::package::Package;

pub struct MonlibHandlers;

impl MonlibHandlers {

    pub fn validator_file(&self, run: &str) -> bool {
        Package.has_metadata(run)
    }

}
