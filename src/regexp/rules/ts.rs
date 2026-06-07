pub struct Ts;

impl Ts {

    pub const WINDOWS_REGISTRY_ACCESS: &str = r#"(?i)((require\(['"]|from\s+['"])winreg['"]|reg\s+add|reg\.exe)"#;
    pub const ARBITRARY_CODE_EXECUTION: &str = r"(?i)(child_process|exec\(|execSync\(|spawn\(|eval\(|new\s+Function\()";
    pub const UNAUTHORIZED_NETWORK_ACCESS: &str = r#"(?i)((require\(['"]|from\s+['"])(http|https|net|dgram|axios|node-fetch)['"]|fetch\(|XMLHttpRequest|curl\s+|wget\s+|nc\s+)"#;
    pub const DESTRUCTIVE_FILE_OPERATIONS: &str = r"(?i)(fs\.rm|fs\.rmSync|fs\.rmdir|fs\.unlink|rm\s+-rf|del\s+/f\s+/s\s+/q)";
    pub const FILE_PERMISSION_TAMPERING: &str = r"(?i)(fs\.chmod|fs\.chown|chmod\s+(\+x|777)|chown\s+)";
    pub const CODE_OBFUSCATION: &str = r"(?i)(Buffer\.from\(|zlib\.|atob\(|btoa\(|unescape\()";
    pub const PRIVILEGE_ESCALATION: &str = r"(?i)(\bsudo\s+|\bsu\s+-|process\.setuid\(|RunAs)";
    pub const ENVIRONMENT_CREDENTIAL_THEFT: &str = r"(?i)(process\.env)";

}