use validator::Validate;

/// Validate a phone number (Indian format).
pub fn validate_phone(phone: &str) -> bool {
    let digits: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();
    // Indian mobile: 10 digits starting with 6-9, or with +91 prefix
    digits.len() == 10 || (digits.len() == 12 && digits.starts_with("91"))
}

/// Validate an email address (basic check).
pub fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.') && email.len() > 5 && email.len() < 255
}

/// Common DTOs with validation.
#[derive(Debug, Validate, serde::Deserialize)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Debug, Validate, serde::Deserialize)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 2, max = 255, message = "Name must be 2-255 characters"))]
    pub name: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    pub phone: Option<String>,
    pub branch_id: Option<i64>,
}

#[derive(Debug, Validate, serde::Deserialize)]
pub struct RefreshTokenRequest {
    #[validate(length(min = 1, message = "Refresh token is required"))]
    pub refresh_token: String,
}

#[derive(Debug, Validate, serde::Deserialize)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 8))]
    pub current_password: String,

    #[validate(length(min = 8, message = "New password must be at least 8 characters"))]
    pub new_password: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_phone() {
        assert!(validate_phone("9876543210"));
        assert!(validate_phone("+919876543210"));
        assert!(!validate_phone("1234"));
        assert!(!validate_phone("abcdefghij"));
    }

    #[test]
    fn test_validate_email() {
        assert!(validate_email("test@example.com"));
        assert!(!validate_email("invalid"));
        assert!(!validate_email("@"));
    }
}
