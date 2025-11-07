pub struct Addons;

impl Addons {

    pub const DOWNLOAD_FILES_URI: &'static str = "https://raw.githubusercontent.com/YeDawa/Scimon/main/";
    pub const DEFAULT_CSS_STYLE: &'static str = "https://addons.scimon.dev/static/md-default.css";

    // Scimon
    pub const SCIMON_SCRAPE_API_ENDPOINT: &'static str = "https://addons.scimon.dev/pdfscrape?url=";
    pub const SCIMON_URLFILTER_API_ENDPOINT: &'static str = "https://addons.scimon.dev/urlfilter?url=";

    // Monlib Package Manager
    pub const MONLIB_API_ENV: &'static str = "MONLIB_API_KEY";
    pub const MONLIB_API_REQUEST: &'static str = "https://api.monlib.net/";

    pub const README_TEMPLATE_LINK: &'static str = "https://readme.scimon.dev/";

    // Chat GPT Content Class
    pub const CHATGPT_CONTENT_CLASS: &'static str = ".markdown.prose.w-full.break-words.dark\\:prose-invert.dark";

    // Download DOI using Annas Archive
    pub const ANNAS_ARCHIVE_ENDPOINT: &'static str = "https://annas-archive.org/scidb/";
    
}
