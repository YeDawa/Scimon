pub struct Bundler;

impl Bundler {

    // Monlib Package Manager
    pub const MONLIB_PACKAGE_MANAGER_KEYS: [&str; 6] = ["name", "description", "author", "license", "privacy", "homepage"];

    // Package metadata for Monlib
    pub const MANIFEST: &'static str = "\
# Scimon package metadata
name: \"my-package\"
description: \"A Scimon package.\"
author: \"Your Name\"
license: \"MIT\"
privacy: \"Public\"
homepage: \"https://example.com\"
";

    // The entry file template for a Scimon package.
    pub const ENTRY_PACKAGE: &'static str = "\
// main.mon — package entry file
path \"downloads/\"

downloads {}
";

}
