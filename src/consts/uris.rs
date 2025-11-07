pub struct Uris;

impl Uris {

    pub const PROVIDERS_DOMAINS: [&'static str; 8] = [
        "wikipedia.org",
        "wikisource.org",
        "raw.githubusercontent.com",
        "gitlab.com",
        "bitbucket.org",
        "codeberg.org",
        "chatgpt.com",
        "sci-hub.se",
    ];

    pub const SCIHUB_PROXY_REQUEST_PDF: &'static str = "https://tools.monlib.net/scihub?url=";
    pub const WIKIPEDIA_API_REQUEST_PDF: &'static str = "https://en.wikipedia.org/api/rest_v1/page/pdf/";
    pub const WIKISOURCE_API_REQUEST_PDF: &'static str = "https://en.wikisource.org/api/rest_v1/page/pdf/";

}
