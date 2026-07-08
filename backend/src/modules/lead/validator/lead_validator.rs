use validator::Validate;

#[derive(Debug, Validate)]
pub struct LeadValidator {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(min = 1, max = 20))]
    pub phone: String,
}

pub fn validate_lead_source(source: &str) -> bool {
    matches!(
        source,
        "landing_page" | "whatsapp" | "referral" | "walk_in"
            | "cold_call" | "social_media" | "field_visit"
    )
}

pub fn validate_lead_status(status: &str) -> bool {
    matches!(
        status,
        "new" | "contacted" | "interested" | "surveyed"
            | "quoted" | "converted" | "lost"
    )
}

pub fn validate_activity_type(activity_type: &str) -> bool {
    matches!(
        activity_type,
        "call" | "whatsapp" | "visit" | "email" | "note" | "status_change"
    )
}
