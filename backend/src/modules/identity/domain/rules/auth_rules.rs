/// Authentication business rules and invariants
pub struct AuthRules;

impl AuthRules {
    /// Maximum failed login attempts before lockout
    pub const MAX_FAILED_ATTEMPTS: i32 = 5;

    /// Account lockout duration in minutes
    pub const LOCKOUT_DURATION_MINUTES: i64 = 30;

    /// Minimum password length
    pub const MIN_PASSWORD_LENGTH: usize = 8;

    /// Access token expiry (24 hours)
    pub const ACCESS_TOKEN_EXPIRY_HOURS: i64 = 24;

    /// Refresh token expiry (7 days)
    pub const REFRESH_TOKEN_EXPIRY_DAYS: i64 = 7;

    /// Maximum active sessions per user
    pub const MAX_SESSIONS_PER_USER: usize = 5;

    /// OTP expiry in seconds (5 minutes)
    pub const OTP_EXPIRY_SECONDS: u64 = 300;

    /// OTP rate limit: max OTPs per phone per hour
    pub const OTP_RATE_LIMIT_PER_HOUR: u32 = 5;

    /// Check if password meets strength requirements
    pub fn is_strong_password(password: &str) -> bool {
        if password.len() < Self::MIN_PASSWORD_LENGTH {
            return false;
        }
        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        has_uppercase && has_lowercase && has_digit
    }

    /// Check if phone number is valid for Indian numbers
    pub fn is_valid_indian_phone(phone: &str) -> bool {
        let digits: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();
        if digits.len() == 10 {
            matches!(digits.chars().next(), Some('6'..='9'))
        } else if digits.len() == 12 && digits.starts_with("91") {
            true
        } else {
            false
        }
    }
}
