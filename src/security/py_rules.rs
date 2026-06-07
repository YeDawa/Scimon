use regex::Regex;

use crate::security::rules::Rules;

pub struct PyRules;

impl PyRules {
    
    pub fn rules(&self, script_content: &str) -> (Vec<String>, bool) {
        let rules = vec![
            // --- System & Execution Threats ---
            Rules {
                name: "Windows Registry Access",
                description: "Attempt to modify the Windows registry.",
                pattern: Regex::new(r"(?i)(import\s+winreg|winreg\.SetValue|reg\s+add|reg\.exe)").unwrap(),
            },
            Rules {
                name: "Arbitrary Code Execution",
                description: "Attempt to execute arbitrary commands on the OS.",
                pattern: Regex::new(r"(?i)(os\.system\(|subprocess\.|eval\(|exec\()").unwrap(),
            },
            
            // --- Network & Data Exfiltration Threats ---
            Rules {
                name: "Unauthorized Network Access",
                description: "Attempt to open sockets, send data to external servers, or download unverified payloads.",
                pattern: Regex::new(r"(?i)(import\s+(requests|urllib|socket|http\.client)|curl\s+|wget\s+|nc\s+)").unwrap(),
            },
            
            // --- File System Threats ---
            Rules {
                name: "Destructive File Operations",
                description: "Attempt to forcefully delete critical system files or entire directories.",
                pattern: Regex::new(r"(?i)(shutil\.rmtree\(|os\.remove\(|os\.unlink\(|rm\s+-rf|del\s+/f\s+/s\s+/q)").unwrap(),
            },
            Rules {
                name: "File Permission Tampering",
                description: "Attempt to change file ownership or execution permissions (e.g., making a script executable).",
                pattern: Regex::new(r"(?i)(os\.chmod\(|os\.chown\(|chmod\s+(\+x|777)|chown\s+)").unwrap(),
            },

            // --- Evasion & Obfuscation Threats ---
            Rules {
                name: "Code Obfuscation",
                description: "Attempt to hide malicious payloads using encoding, compression, or dynamic attribute access.",
                pattern: Regex::new(r"(?i)(base64\.b64decode\(|zlib\.decompress\(|__import__\(|getattr\()").unwrap(),
            },

            // --- Privilege & Credential Threats ---
            Rules {
                name: "Privilege Escalation",
                description: "Attempt to execute commands as root or administrator.",
                pattern: Regex::new(r"(?i)(\bsudo\s+|\bsu\s+-|os\.setuid\(|RunAs)").unwrap(),
            },
            Rules {
                name: "Environment Credential Theft",
                description: "Attempt to read sensitive environment variables (API keys, passwords).",
                pattern: Regex::new(r"(?i)(os\.environ|os\.getenv\()").unwrap(),
            },
        ];
        
        let mut violations: Vec<String> = Vec::new();
        let mut is_safe = true;

        for rule in rules {
            if rule.pattern.is_match(script_content) {
                violations.push(format!("[{}] {}", rule.name, rule.description));
                is_safe = false;
            }
        }

        (violations, is_safe)
    }

}
