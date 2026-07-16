use serde::{Deserialize, Serialize};
use std::fmt;

/// Permission value object with module.resource.action format
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Permission {
    pub name: String,
    pub module: String,
    pub action: String,
}

impl Permission {
    pub fn new(name: String, module: String, action: String) -> Self {
        Self { name, module, action }
    }

    pub fn from_string(permission_str: &str) -> Self {
        let parts: Vec<&str> = permission_str.splitn(3, '.').collect();
        let module = parts.first().unwrap_or(&"").to_string();
        let action = parts.last().unwrap_or(&"").to_string();
        Self {
            name: permission_str.to_string(),
            module,
            action,
        }
    }

    /// Check if this permission matches a target permission (supports wildcards)
    pub fn matches_wildcard(&self, target: &str) -> bool {
        let pattern_parts: Vec<&str> = self.name.split('.').collect();
        let target_parts: Vec<&str> = target.split('.').collect();

        if pattern_parts.len() != target_parts.len() {
            return false;
        }

        pattern_parts
            .iter()
            .zip(target_parts.iter())
            .all(|(p, t)| p == &"*" || p == t)
    }

    pub fn as_str(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
