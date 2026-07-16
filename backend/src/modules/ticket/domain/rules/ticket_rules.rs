/// Ticket business rules and invariants
pub struct TicketRules;

impl TicketRules {
    /// Valid ticket categories
    pub const VALID_CATEGORIES: &[&str] = &[
        "connectivity", "installation", "billing", "hardware", "account", "other",
    ];

    /// Valid ticket sources
    pub const VALID_SOURCES: &[&str] = &[
        "customer", "phone", "email", "whatsapp", "agent", "system",
    ];

    /// Maximum satisfaction rating
    pub const MAX_SATISFACTION_RATING: i32 = 5;

    /// Auto-escalation threshold (minutes before SLA breach)
    pub const AUTO_ESCALATE_BEFORE_BREACH_MINUTES: i64 = 30;

    /// Check if category is valid
    pub fn is_valid_category(category: &str) -> bool {
        Self::VALID_CATEGORIES.contains(&category)
    }

    /// Check if source is valid
    pub fn is_valid_source(source: &str) -> bool {
        Self::VALID_SOURCES.contains(&source)
    }
}
