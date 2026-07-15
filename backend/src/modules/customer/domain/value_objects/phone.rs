use serde::{Deserialize, Serialize};
use std::fmt;

/// Phone value object with validation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Phone(String);

impl Phone {
    /// Create a new Phone with validation
    pub fn new(phone: &str) -> Result<Self, crate::modules::customer::domain::aggregates::customer::CustomerDomainError> {
        if !Self::is_valid(phone) {
            return Err(crate::modules::customer::domain::aggregates::customer::CustomerDomainError::InvalidPhone);
        }
        Ok(Self(phone.to_string()))
    }

    /// Validate phone format
    fn is_valid(phone: &str) -> bool {
        if phone.is_empty() || phone.len() > 20 {
            return false;
        }

        // Remove common prefixes
        let cleaned = phone.trim_start_matches('+').trim_start_matches("00");

        // Check if remaining characters are digits or common separators
        let valid_chars = |c: char| c.is_ascii_digit() || c == '-' || c == ' ' || c == '(' || c == ')';
        if !cleaned.chars().all(valid_chars) {
            return false;
        }

        // Extract digits only
        let digits: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();
        
        // Indian phone numbers: 10 digits (mobile) or 10-12 digits (with area code)
        // International: 7-15 digits
        digits.len() >= 7 && digits.len() <= 15
    }

    /// Get the phone as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Get digits only
    pub fn digits_only(&self) -> String {
        self.0.chars().filter(|c| c.is_ascii_digit()).collect()
    }

    /// Check if this is an Indian phone number
    pub fn is_indian(&self) -> bool {
        let digits = self.digits_only();
        // Indian mobile numbers start with 6, 7, 8, or 9 and are 10 digits
        // Or with country code 91 and then 10 digits
        if digits.len() == 10 && matches!(digits.chars().next(), Some('6'..='9')) {
            true
        } else if digits.len() == 12 && digits.starts_with("91") {
            true
        } else {
            false
        }
    }

    /// Format as international number
    pub fn to_international(&self) -> String {
        if self.is_indian() {
            let digits = self.digits_only();
            if digits.len() == 10 {
                format!("+91{}", digits)
            } else {
                format!("+{}", digits)
            }
        } else {
            format!("+{}", self.digits_only())
        }
    }
}

impl fmt::Display for Phone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Phone> for String {
    fn from(phone: Phone) -> Self {
        phone.0
    }
}

impl AsRef<str> for Phone {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_phones() {
        assert!(Phone::new("+919876543210").is_ok());
        assert!(Phone::new("9876543210").is_ok());
        assert!(Phone::new("+1-555-123-4567").is_ok());
        assert!(Phone::new("555 123 4567").is_ok());
    }

    #[test]
    fn test_invalid_phones() {
        assert!(Phone::new("").is_err());
        assert!(Phone::new("123").is_err()); // Too short
        assert!(Phone::new("abc12345678").is_err()); // Invalid chars
    }

    #[test]
    fn test_indian_detection() {
        let phone = Phone::new("+919876543210").unwrap();
        assert!(phone.is_indian());
        
        let phone = Phone::new("9876543210").unwrap();
        assert!(phone.is_indian());
        
        let phone = Phone::new("+15551234567").unwrap();
        assert!(!phone.is_indian());
    }

    #[test]
    fn test_international_format() {
        let phone = Phone::new("9876543210").unwrap();
        assert_eq!(phone.to_international(), "+919876543210");
    }

    #[test]
    fn test_digits_only() {
        let phone = Phone::new("+91-98765-43210").unwrap();
        assert_eq!(phone.digits_only(), "919876543210");
    }
}
