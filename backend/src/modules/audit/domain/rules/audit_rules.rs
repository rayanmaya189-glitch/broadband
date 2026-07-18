/// Audit business rules and invariants
pub struct AuditRules;

impl AuditRules {
    /// Retention period for audit logs (7 years)
    pub const RETENTION_YEARS: i32 = 7;

    /// Maximum query results per page
    pub const MAX_PAGE_SIZE: i64 = 100;

    /// Check if action is security-sensitive
    pub fn is_security_sensitive(action: &str) -> bool {
        matches!(
            action,
            "login" | "logout" | "password_change" | "2fa_enable" | "role_assign"
        )
    }

    /// Check if resource type should be audited
    pub fn should_audit(resource_type: &str) -> bool {
        matches!(
            resource_type,
            "user" | "role" | "plan" | "invoice" | "payment" | "device" | "network"
        )
    }
}
