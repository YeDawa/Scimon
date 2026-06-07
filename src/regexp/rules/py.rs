pub struct Py;

impl Py {

    pub const REGEX_WINDOWS_REGISTRY: &str = r"(?i)(import\s+winreg|winreg\.SetValue|reg\s+add|reg\.exe)";
    pub const REGEX_ARBITRARY_CODE: &str = r"(?i)(os\.system\(|subprocess\.|eval\(|exec\()";
    pub const REGEX_NETWORK_ACCESS: &str = r"(?i)(import\s+(requests|urllib|socket|http\.client)|curl\s+|wget\s+|nc\s+)";
    pub const REGEX_DESTRUCTIVE_FILE: &str = r"(?i)(shutil\.rmtree\(|os\.remove\(|os\.unlink\(|rm\s+-rf|del\s+/f\s+/s\s+/q)";
    pub const REGEX_FILE_PERMISSION: &str = r"(?i)(os\.chmod\(|os\.chown\(|chmod\s+(\+x|777)|chown\s+)";
    pub const REGEX_OBFUSCATION: &str = r"(?i)(base64\.b64decode\(|zlib\.decompress\(|__import__\(|getattr\()";
    pub const REGEX_PRIVILEGE_ESCALATION: &str = r"(?i)(\bsudo\s+|\bsu\s+-|os\.setuid\(|RunAs)";
    pub const REGEX_CREDENTIAL_THEFT: &str = r"(?i)(os\.environ|os\.getenv\()";

}