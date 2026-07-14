use validator::Validate;

#[derive(Debug, Validate)]
pub struct InteractionValidator {
    #[validate(length(min = 1, max = 30))]
    pub interaction_type: String,
    #[validate(length(min = 1))]
    pub subject: String,
}

pub fn validate_interaction_type(t: &str) -> bool {
    matches!(t, "call" | "email" | "whatsapp" | "visit" | "chat" | "sms" | "note")
}

pub fn validate_sentiment(s: &str) -> bool {
    matches!(s, "positive" | "negative" | "neutral")
}
