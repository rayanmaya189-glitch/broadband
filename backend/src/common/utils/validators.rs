/// Validate a phone number (Indian format).
pub fn validate_phone(phone: &str) -> bool {
    let digits: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();
    digits.len() == 10 || (digits.len() == 12 && digits.starts_with("91"))
}

/// Validate an email address (basic check).
pub fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.') && email.len() > 5 && email.len() < 255
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
