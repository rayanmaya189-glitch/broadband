/// Notification business rules and invariants
pub struct NotificationRules;

impl NotificationRules {
    /// Maximum retry attempts
    pub const MAX_RETRIES: i32 = 3;

    /// Maximum notification body length (SMS)
    pub const SMS_MAX_LENGTH: usize = 160;

    /// Maximum email body length
    pub const EMAIL_MAX_LENGTH: usize = 50000;

    /// Rate limit: max notifications per recipient per hour
    pub const RATE_LIMIT_PER_HOUR: u32 = 10;

    /// Check if channel is valid for recipient type
    pub fn is_channel_valid_for_recipient(channel: &str, recipient_type: &str) -> bool {
        matches!(
            (channel, recipient_type),
            ("email", "customer")
                | ("email", "user")
                | ("sms", "customer")
                | ("sms", "user")
                | ("whatsapp", "customer")
                | ("push", "customer")
                | ("push", "user")
                | ("in_app", _)
        )
    }

    /// Calculate retry delay with exponential backoff
    pub fn retry_delay_seconds(retry_count: i32) -> u64 {
        10u64.pow(retry_count as u32)
    }
}
