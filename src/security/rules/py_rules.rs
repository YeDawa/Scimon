use regex::Regex;

use crate::{
    regexp::rules::py::Py,
    security::rules::rules::Rules,
};

pub struct PyRules;

impl PyRules {
    
    pub fn rules(&self, script_content: &str) -> (Vec<String>, bool) {
        let rules = vec![
            // --- System & Execution Threats ---
            Rules {
                name: "Windows Registry Access",
                description: "Attempt to modify the Windows registry.",
                pattern: Regex::new(Py::REGEX_WINDOWS_REGISTRY).unwrap(),
            },
            Rules {
                name: "Arbitrary Code Execution",
                description: "Attempt to execute arbitrary commands on the OS.",
                pattern: Regex::new(Py::REGEX_ARBITRARY_CODE).unwrap(),
            },
            
            // --- Network & Data Exfiltration Threats ---
            Rules {
                name: "Unauthorized Network Access",
                description: "Attempt to open sockets, send data to external servers, or download unverified payloads.",
                pattern: Regex::new(Py::REGEX_NETWORK_ACCESS).unwrap(),
            },
            
            // --- File System Threats ---
            Rules {
                name: "Destructive File Operations",
                description: "Attempt to forcefully delete critical system files or entire directories.",
                pattern: Regex::new(Py::REGEX_DESTRUCTIVE_FILE).unwrap(),
            },
            Rules {
                name: "File Permission Tampering",
                description: "Attempt to change file ownership or execution permissions (e.g., making a script executable).",
                pattern: Regex::new(Py::REGEX_FILE_PERMISSION).unwrap(),
            },

            // --- Evasion & Obfuscation Threats ---
            Rules {
                name: "Code Obfuscation",
                description: "Attempt to hide malicious payloads using encoding, compression, or dynamic attribute access.",
                pattern: Regex::new(Py::REGEX_OBFUSCATION).unwrap(),
            },

            // --- Privilege & Credential Threats ---
            Rules {
                name: "Privilege Escalation",
                description: "Attempt to execute commands as root or administrator.",
                pattern: Regex::new(Py::REGEX_PRIVILEGE_ESCALATION).unwrap(),
            },
            Rules {
                name: "Environment Credential Theft",
                description: "Attempt to read sensitive environment variables (API keys, passwords).",
                pattern: Regex::new(Py::REGEX_CREDENTIAL_THEFT).unwrap(),
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
