use regex::Regex;

use crate::security::rules::rules::Rules;

pub struct TsRules;

impl TsRules {
    
    pub fn rules(&self, script_content: &str) -> (Vec<String>, bool) {
        let rules = vec![
            // --- System & Execution Threats ---
            Rules {
                name: "Windows Registry Access",
                description: "Attempt to modify the Windows registry via external packages.",
                // Captura tanto require('winreg') quanto import 'winreg' ou import { ... } from 'winreg'
                pattern: Regex::new(r#"(?i)((require\(['"]|from\s+['"])winreg['"]|reg\s+add|reg\.exe)"#).unwrap(),
            },
            Rules {
                name: "Arbitrary Code Execution",
                description: "Attempt to execute arbitrary commands using child_process, eval, or Function constructor.",
                // Mantém a busca pela palavra child_process pura, o que bloqueia 'import child_process' automaticamente
                pattern: Regex::new(r"(?i)(child_process|exec\(|execSync\(|spawn\(|eval\(|new\s+Function\()").unwrap(),
            },
            
            // --- Network & Data Exfiltration Threats ---
            Rules {
                name: "Unauthorized Network Access",
                description: "Attempt to open sockets, use native http/net modules, or make fetch requests.",
                // Captura imports de rede modernos e clássicos
                pattern: Regex::new(r#"(?i)((require\(['"]|from\s+['"])(http|https|net|dgram|axios|node-fetch)['"]|fetch\(|XMLHttpRequest|curl\s+|wget\s+|nc\s+)"#).unwrap(),
            },
            
            // --- File System Threats ---
            Rules {
                name: "Destructive File Operations",
                description: "Attempt to forcefully delete files using fs modules or shell commands.",
                pattern: Regex::new(r"(?i)(fs\.rm|fs\.rmSync|fs\.rmdir|fs\.unlink|rm\s+-rf|del\s+/f\s+/s\s+/q)").unwrap(),
            },
            Rules {
                name: "File Permission Tampering",
                description: "Attempt to change file ownership or execution permissions via fs.",
                pattern: Regex::new(r"(?i)(fs\.chmod|fs\.chown|chmod\s+(\+x|777)|chown\s+)").unwrap(),
            },

            // --- Evasion & Obfuscation Threats ---
            Rules {
                name: "Code Obfuscation",
                description: "Attempt to hide payloads using Base64 buffers, zlib, or string evaluation.",
                pattern: Regex::new(r"(?i)(Buffer\.from\(|zlib\.|atob\(|btoa\(|unescape\()").unwrap(),
            },

            // --- Privilege & Credential Threats ---
            Rules {
                name: "Privilege Escalation",
                description: "Attempt to execute commands as root or change process UID.",
                pattern: Regex::new(r"(?i)(\bsudo\s+|\bsu\s+-|process\.setuid\(|RunAs)").unwrap(),
            },
            Rules {
                name: "Environment Credential Theft",
                description: "Attempt to read sensitive environment variables (API keys, passwords) via process.env.",
                pattern: Regex::new(r"(?i)(process\.env)").unwrap(),
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