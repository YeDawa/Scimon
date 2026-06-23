pub struct BlocksRegExp;

impl BlocksRegExp {
    
    pub const GET_README_BLOCK: [&'static str; 2] = [
        r"(?i)readme\s*\{",
        r"\}",
    ];

    pub const GET_AI_BLOCK: &'static str = r"(?im)^\s*ai\s*\{";

    pub const GET_AI_ENTRY: &'static str = r#"(?i)^"(?P<prompt>(?:[^"\\]|\\.)*)"\s+as\s+"(?P<file>[^"]+)"(?:\s+with\s+"(?P<model>[^"]+)")?"#;

    pub const GET_PATH_VAR: &'static str = r#"(?i)path\s*"([^"]+)""#;

    pub const GET_OPEN_VAR: &'static str = r#"(?i)open\s*"([^"]+)""#;
    
    pub const GET_STYLE_VAR: &'static str = r#"(?i)style\s*"([^"]+)""#;
    
    pub const GET_PRINT_VAR: &'static str = r#"(?i)print\s*"([^"]+)""#;
    
    pub const GET_README_VAR: &'static str = r#"(?i)readme\s*"([^"]+)""#;
    
    pub const GET_COVERS_VAR: &'static str = r#"(?i)covers\s*"([^"]+)""#;
    
    pub const GET_QRCODE_VAR: &'static str = r#"(?i)qrcode\s*"([^"]+)""#;

    pub const GET_SERVER_VAR: &'static str = r#"(?i)server\s*"([^"]+)""#;

    pub const GET_COMPRESS_VAR: &'static str = r#"(?i)compress\s*"([^"]+)""#;

    pub const GET_COPY_VAR: &'static str = r#"(?i)copy\s*"([^"]+)""#;

    pub const GET_IMPORT_VAR: &'static str = r#"(?i)import\s*"([^"]+)""#;

    pub const GET_INCLUDE_VAR: &'static str = r#"(?i)^\s*include\s+"([^"]+)"\s*$"#;

    pub const GET_VAR_DEF: &'static str = r#"(?im)^\s*@var\s+([A-Za-z_][A-Za-z0-9_]*)\s+"([^"]*)"\s*$"#;

    pub const GET_VAR_USAGE: &'static str = r"\$\{\s*([A-Za-z_][A-Za-z0-9_]*)\s*\}";

    pub const GET_RANGE: &'static str = r"\{([^{}]+?)\.\.(\d+)\}";

    pub const GET_FOR_LOOP: &'static str = r"(?m)^\s*for\s+([A-Za-z_][A-Za-z0-9_]*)\s+in\s+(\[[^\]]*\]|\{[^{}]*\})\s*\{";

    pub const GET_FN_DEF: &'static str = r"@fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(([^)]*)\)\s*\{";

    pub const GET_FN_CALL: &'static str = r"@([A-Za-z_][A-Za-z0-9_]*)\s*\(([^)]*)\)";

    pub const GET_MATH_VAR: &'static str = r#"math\s+['"]([^'"]+)['"]\s*>\s*(\S+)"#;

    pub const GET_MERGE_VAR: &'static str = r#"merge\s+['"]([^'"]+)['"]\s*>\s*['"]?([^'"\s]+)['"]?"#;

    pub const GET_CONVERT_VAR: &'static str = r#"convert\s+['"]([^'"]+)['"]\s*>\s*['"]?([^'"\s]+)['"]?"#;

    pub const GET_PATTERNS_MONLIB_VARS: [&'static str; 7] = [
        r#"(?m)^\s*@name\s+"[^"]+""#,
        r#"(?m)^\s*@version\s+"[^"]+""#,
        r#"(?m)^\s*@description\s+"[^"]+""#,
        r#"(?m)^\s*@author\s+"[^"]+""#,
        r#"(?m)^\s*@license\s+"[^"]+""#,
        r#"(?m)^\s*@privacy\s+"[^"]+""#,
        r#"(?m)^\s*@homepage\s+"[^"]+""#,
    ];
     
}
