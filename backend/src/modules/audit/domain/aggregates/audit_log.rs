use crate::modules::audit::domain::value_objects::{AuditAction, AuditId, AuditResult};

/// AuditLog aggregate root - represents an immutable audit trail entry
#[derive(Debug, Clone)]
pub struct AuditLog {
    pub id: AuditId,
    pub user_id: Option<i64>,
    pub user_email: Option<String>,
    pub user_role: Option<String>,
    pub action: AuditAction,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub result: AuditResult,
    pub old_data: Option<serde_json::Value>,
    pub new_data: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

/// Domain errors for AuditLog aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum AuditDomainError {
    AuditNotFound(i64),
    InvalidAction,
    CannotModifyAuditLog,
}

impl std::fmt::Display for AuditDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AuditNotFound(id) => write!(f, "Audit log {} not found", id),
            Self::InvalidAction => write!(f, "Invalid audit action"),
            Self::CannotModifyAuditLog => write!(f, "Audit logs are immutable and cannot be modified"),
        }
    }
}

impl std::error::Error for AuditDomainError {}

impl AuditLog {
    pub fn new(
        user_id: Option<i64>,
        user_email: Option<String>,
        user_role: Option<String>,
        action: AuditAction,
        resource_type: Option<String>,
        resource_id: Option<String>,
        ip_address: Option<String>,
        user_agent: Option<String>,
        result: AuditResult,
    ) -> Self {
        Self {
            id: AuditId::new(0),
            user_id,
            user_email,
            user_role,
            action,
            resource_type,
            resource_id,
            ip_address,
            user_agent,
            result,
            old_data: None,
            new_data: None,
            metadata: None,
        }
    }

    pub fn with_changes(mut self, old: Option<serde_json::Value>, new: Option<serde_json::Value>) -> Self {
        self.old_data = old;
        self.new_data = new;
        self
    }

    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn is_denied(&self) -> bool {
        self.result == AuditResult::Denied
    }

    pub fn is_granted(&self) -> bool {
        self.result == AuditResult::Granted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_audit_log() {
        let log = AuditLog::new(
            Some(1), Some("admin@aeroxe.com".to_string()), Some("super_admin".to_string()),
            AuditAction::Login, Some("user".to_string()), Some("1".to_string()),
            Some("10.0.1.50".to_string()), Some("Mozilla/5.0".to_string()),
            AuditResult::Granted,
        );
        assert!(log.is_granted());
        assert!(!log.is_denied());
    }
}
