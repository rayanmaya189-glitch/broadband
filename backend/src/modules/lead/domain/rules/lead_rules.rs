/// Lead business rules and invariants
pub struct LeadRules;

impl LeadRules {
    /// Valid lead sources
    pub const VALID_SOURCES: &[&str] = &[
        "landing_page",
        "whatsapp",
        "referral",
        "walk_in",
        "cold_call",
        "social_media",
        "field_visit",
    ];

    /// Valid pipeline stages in order
    pub const PIPELINE_STAGES: &[&str] = &["new", "contacted", "interested", "surveyed", "quoted"];

    /// Check if source is valid
    pub fn is_valid_source(source: &str) -> bool {
        Self::VALID_SOURCES.contains(&source)
    }
}
