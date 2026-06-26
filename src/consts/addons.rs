pub struct Addons;

impl Addons {

    pub const DOWNLOAD_FILES_URI: &'static str = "https://raw.githubusercontent.com/YeDawa/Scimon/main/";
    pub const DEFAULT_CSS_STYLE: &'static str = "https://static.monlib.net/md-default.css";
    pub const DEFAULT_LATEX_CSS_STYLE: &'static str = "https://static.monlib.net/latex.css";
    pub const DEFAULT_LATEX_JS_SCRIPT: &'static str = "https://static.monlib.net/latex.js";

    // Scimon
    pub const SCIMON_URLFILTER_API_ENDPOINT: &'static str = "https://monlib.net/external?url=";
    pub const SCIMON_SCRAPE_API_ENDPOINT: &'static str = "https://addons.scimon.dev/pdfscrape?url=";

    // SPDX license texts (%s = license identifier, e.g. MIT, Apache-2.0)
    pub const SPDX_LICENSE_TEXT: &'static str = "https://raw.githubusercontent.com/spdx/license-list-data/main/text/%s.txt";

    // Monlib Package Manager
    pub const MONLIB_API_ENV: &'static str = "MONLIB_API_KEY";
    pub const MONLIB_API_REQUEST: &'static str = "https://system.monlib.net/";

    // Security entropy threshold
    pub const MAX_SAFE_ENTROPY: f64 = 5.96;
    
}
