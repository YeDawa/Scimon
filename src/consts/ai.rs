pub struct AI;

impl AI {

    // OpenRouter (AI generated markdown)
    pub const OPENROUTER_API_ENV: &'static str = "OPENROUTER_API_KEY";
    pub const OPENROUTER_API_REQUEST: &'static str = "https://openrouter.ai/api/v1/chat/completions";
    pub const OPENROUTER_DEFAULT_MODEL: &'static str = "openai/gpt-4o-mini";

    // OpenAI (ChatGPT API)
    pub const OPENAI_API_ENV: &'static str = "OPENAI_API_KEY";
    pub const OPENAI_API_REQUEST: &'static str = "https://api.openai.com/v1/chat/completions";
    pub const OPENAI_DEFAULT_MODEL: &'static str = "gpt-4o-mini";

    // Google Gemini API
    pub const GEMINI_API_ENV: &'static str = "GEMINI_API_KEY";
    pub const GEMINI_API_REQUEST: &'static str = "https://generativelanguage.googleapis.com/v1beta/models/";
    pub const GEMINI_DEFAULT_MODEL: &'static str = "gemini-1.5-flash";

}
