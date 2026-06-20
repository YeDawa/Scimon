pub struct Addons;

impl Addons {

    pub const DOWNLOAD_FILES_URI: &'static str = "https://raw.githubusercontent.com/YeDawa/Scimon/main/";
    pub const DEFAULT_CSS_STYLE: &'static str = "https://static.monlib.net/md-default.css";
    pub const DEFAULT_LATEX_CSS_STYLE: &'static str = "https://static.monlib.net/latex.css";
    pub const DEFAULT_LATEX_JS_SCRIPT: &'static str = "https://static.monlib.net/latex.js";

    // Scimon
    pub const SCIMON_SCRAPE_API_ENDPOINT: &'static str = "https://addons.scimon.dev/pdfscrape?url=";
    pub const SCIMON_URLFILTER_API_ENDPOINT: &'static str = "https://addons.scimon.dev/urlfilter?url=";

    // Monlib Package Manager
    pub const MONLIB_API_ENV: &'static str = "MONLIB_API_KEY";
    pub const MONLIB_API_REQUEST: &'static str = "https://system.monlib.net/";

    // OpenRouter (AI generated markdown)
    pub const OPENROUTER_API_ENV: &'static str = "OPENROUTER_API_KEY";
    pub const OPENROUTER_API_REQUEST: &'static str = "https://openrouter.ai/api/v1/chat/completions";
    pub const OPENROUTER_DEFAULT_MODEL: &'static str = "openai/gpt-4o-mini";

    // Chat GPT Content Class
    pub const CHATGPT_CONTENT_CLASS: &'static str = "section[data-turn=\"assistant\"]";

    // Chat GPT Content H4 Class
    pub const CHATGPT_CONTENT_H4_CLASS: &'static str = "<div class=\"relative h-6 w-6\"></div></div>";

    // Chat GPT Content Class Alternative (Reasoning Text)
    pub const CHATGPT_CONTENT_CLASS_ALT: &'static str = r#"(?is)<div class="w-full leading-relaxed font-normal text-token-text-tertiary">.*?</div>\s*"#;

    // Gemini Content Class
    pub const GEMINI_CONTENT_CLASS: &'static str = "div.response-container-content";

    // Security entropy threshold
    pub const MAX_SAFE_ENTROPY: f64 = 5.96;
    
}
