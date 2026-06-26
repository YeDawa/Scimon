pub struct AI;

impl AI {

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
    
}
