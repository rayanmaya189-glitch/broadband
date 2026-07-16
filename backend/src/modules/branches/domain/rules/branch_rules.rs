/// Branch business rules and invariants
pub struct BranchRules;

impl BranchRules {
    /// Valid timezone for India operations
    pub const DEFAULT_TIMEZONE: &'static str = "Asia/Kolkata";

    /// Maximum branches per ISP owner
    pub const MAX_BRANCHES: usize = 100;

    /// Check if branch code follows naming convention
    pub fn is_valid_code(code: &str) -> bool {
        code.len() >= 2
            && code.len() <= 20
            && code
                .chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
    }

    /// Check if branch slug follows naming convention
    pub fn is_valid_slug(slug: &str) -> bool {
        !slug.is_empty()
            && slug.len() <= 100
            && slug
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
            && !slug.starts_with('-')
            && !slug.ends_with('-')
    }

    /// Check if branch can be deleted (no active customers)
    pub fn can_be_deleted(has_active_customers: bool, has_active_subscriptions: bool) -> bool {
        !has_active_customers && !has_active_subscriptions
    }
}
