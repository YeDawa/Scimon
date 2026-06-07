use regex::Regex;

use crate::{
    regexp::rules::js::Js,
    security::rules::rules::Rules,
};

pub struct JsRules;

impl JsRules {
    
    pub fn rules(&self, script_content: &str) -> (Vec<String>, bool) {
        let rules = vec![
            // --- System & Execution Threats ---
            Rules {
                name: "Windows Registry Access",
                description: "Attempt to modify the Windows registry via child_process or external packages.",
                pattern: Regex::new(Js::REGEX_WINDOWS_REGISTRY).unwrap(),
            },
            Rules {
                name: "Arbitrary Code Execution",
                description: "Attempt to execute arbitrary commands using child_process, eval, or Function constructor.",
                pattern: Regex::new(Js::REGEX_ARBITRARY_CODE).unwrap(),
            },
            
            // --- Network & Data Exfiltration Threats ---
            Rules {
                name: "Unauthorized Network Access",
                description: "Attempt to open sockets, use native http/net modules, or make fetch requests.",
                pattern: Regex::new(Js::REGEX_NETWORK_ACCESS).unwrap(),
            },
            
            // --- File System Threats ---
            Rules {
                name: "Destructive File Operations",
                description: "Attempt to forcefully delete files using fs modules or shell commands.",
                pattern: Regex::new(Js::REGEX_DESTRUCTIVE_FILE).unwrap(),
            },
            Rules {
                name: "File Permission Tampering",
                description: "Attempt to change file ownership or execution permissions via fs.",
                pattern: Regex::new(Js::REGEX_FILE_PERMISSION).unwrap(),
            },

            // --- Evasion & Obfuscation Threats ---
            Rules {
                name: "Code Obfuscation",
                description: "Attempt to hide payloads using Base64 buffers, zlib, or string evaluation.",
                pattern: Regex::new(Js::REGEX_OBFUSCATION).unwrap(),
            },

            // --- Privilege & Credential Threats ---
            Rules {
                name: "Privilege Escalation",
                description: "Attempt to execute commands as root or change process UID.",
                pattern: Regex::new(Js::REGEX_PRIVILEGE_ESCALATION).unwrap(),
            },
            Rules {
                name: "Environment Credential Theft",
                description: "Attempt to read sensitive environment variables (API keys, passwords) via process.env.",
                pattern: Regex::new(Js::REGEX_CREDENTIAL_THEFT).unwrap(),
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