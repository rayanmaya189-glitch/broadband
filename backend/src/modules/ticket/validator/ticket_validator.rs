use validator::Validate;

#[derive(Debug, Validate)]
pub struct TicketValidator {
    #[validate(length(min = 1, max = 255))]
    pub subject: String,
    #[validate(length(min = 1))]
    pub description: String,
    #[validate(length(min = 1, max = 50))]
    pub category: String,
}

pub fn validate_ticket_priority(priority: &str) -> bool {
    matches!(priority, "critical" | "high" | "medium" | "low")
}

pub fn validate_ticket_status(status: &str) -> bool {
    matches!(
        status,
        "open" | "assigned" | "in_progress" | "waiting_customer"
            | "escalated" | "resolved" | "closed" | "reopened"
    )
}

pub fn validate_ticket_source(source: &str) -> bool {
    matches!(
        source,
        "customer" | "phone" | "email" | "whatsapp" | "agent" | "system"
    )
}

pub fn validate_ticket_category(category: &str) -> bool {
    matches!(
        category,
        "connectivity" | "installation" | "billing" | "hardware" | "account" | "other"
    )
}
