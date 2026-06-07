pub struct Js;

impl Js {
    
    pub const REGEX_WINDOWS_REGISTRY: &str = r#"(?i)(require\(['"]winreg['"]\)|reg\s+add|reg\.exe)"#;
    pub const REGEX_ARBITRARY_CODE: &str = r#"(?i)(child_process|exec\(|execSync\(|spawn\(|eval\(|new\s+Function\()"#;
    pub const REGEX_NETWORK_ACCESS: &str = r#"(?i)(require\(['"](http|https|net|dgram|axios|node-fetch)['"]\)|fetch\(|XMLHttpRequest|curl\s+|wget\s+|nc\s+)"#;
    pub const REGEX_DESTRUCTIVE_FILE: &str = r#"(?i)(fs\.rm|fs\.rmSync|fs\.rmdir|fs\.unlink|rm\s+-rf|del\s+/f\s+/s\s+/q)"#;
    pub const REGEX_FILE_PERMISSION: &str = r#"(?i)(fs\.chmod|fs\.chown|chmod\s+(\+x|777)|chown\s+)"#;
    pub const REGEX_OBFUSCATION: &str = r#"(?i)(Buffer\.from\(|zlib\.|atob\(|btoa\(|unescape\()"#;
    pub const REGEX_PRIVILEGE_ESCALATION: &str = r#"(?i)(\bsudo\s+|\bsu\s+-|process\.setuid\(|RunAs)"#;
    pub const REGEX_CREDENTIAL_THEFT: &str = r#"(?i)(process\.env)"#;

}