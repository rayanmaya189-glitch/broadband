use crate::common::utils::validators::{validate_email, validate_phone};

/// Validate user registration input.
pub fn validate_registration(email: &str, phone: &str, password: &str) -> Result<(), String> {
    if !validate_email(email) {
        return Err("Invalid email format".into());
    }
    if !validate_phone(phone) {
        return Err("Invalid phone number".into());
    }
    if password.len() < 8 {
        return Err("Password must be at least 8 characters".into());
    }
    Ok(())
}

/// Validate password strength.
pub fn validate_password_strength(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        return Err("Password must be at least 8 characters".into());
    }
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err("Password must contain an uppercase letter".into());
    }
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err("Password must contain a lowercase letter".into());
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err("Password must contain a number".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_registration() {
        assert!(validate_registration("test@example.com", "9876543210", "password123").is_ok());
        assert!(validate_registration("invalid", "9876543210", "password123").is_err());
    }

    #[test]
    fn test_validate_password_strength() {
        assert!(validate_password_strength("StrongPass1").is_ok());
        assert!(validate_password_strength("weak").is_err());
    }
}
