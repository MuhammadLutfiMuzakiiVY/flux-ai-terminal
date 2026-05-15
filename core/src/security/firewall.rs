//! Intelligent Command Firewall
//!
//! Deeply analyzes every command before execution using heuristic patterns
//! and security rules. Prevents destructive operations and protects system integrity.

use crate::security::SafetyAction;
use regex::Regex;

pub struct CommandFirewall {
    block_patterns: Vec<Regex>,
    warn_patterns: Vec<Regex>,
}

impl CommandFirewall {
    pub fn new() -> Self {
        let block = vec![
            r"rm\s+-rf\s+/",          // Recursive root delete
            r"dd\s+if=",              // Raw disk writing
            r"mkfs\.",                // Filesystem formatting
            r":\(\)\{:\|:&\};:",      // Fork bomb
            r"chmod\s+-R\s+777\s+/",  // Insecure root permissions
            r"mv\s+/\s+/dev/null",     // Moving root to null
            r">\s*/dev/sd[a-z]",      // Direct disk redirection
            r"(wget|curl)\s+.*\s+\|\s*sh", // Pipe to shell (insecure install)
            r"shadow",                // Accessing shadow file
            r"/etc/passwd",           // Writing to passwd
        ];

        let warn = vec![
            r"rm\s+-rf",
            r"sudo\s+rm",
            r"chmod\s+777",
            r"kill\s+-9",
            r"shutdown",
            r"reboot",
            r"passwd",
        ];

        Self {
            block_patterns: block.iter().map(|p| Regex::new(p).unwrap()).collect(),
            warn_patterns: warn.iter().map(|p| Regex::new(p).unwrap()).collect(),
        }
    }

    pub fn analyze(&self, command: &str) -> SafetyAction {
        for re in &self.block_patterns {
            if re.is_match(command) {
                return SafetyAction::Block;
            }
        }
        for re in &self.warn_patterns {
            if re.is_match(command) {
                return SafetyAction::Warn;
            }
        }
        SafetyAction::Allow
    }
}
