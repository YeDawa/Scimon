pub struct Bundler;

impl Bundler {

    // Monlib Package Manager
    pub const MONLIB_PACKAGE_MANAGER_KEYS: [&str; 6] = [
        "name", "description", "author", "license", "privacy", "homepage"
    ];

    // The entry file template for a Scimon package.
    pub const ENTRY_PACKAGE: &'static str = "\
// main.mon — package entry file
print \"hello, world!\"";

}
