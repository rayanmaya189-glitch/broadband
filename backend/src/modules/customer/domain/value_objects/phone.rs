//! Phone value object.
//!
//! An immutable value object representing a validated phone number.

use serde::{Deserialize, Serialize};

/// Phone value object with validation.
///
/// # Invariants
/// - Must not be empty
/// - Must contain only digits, +, -, (, ), and spaces
/// - Minimum length: 7 characters
/// - Maximum length: 20 characters
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Phone(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhoneError {
    Empty,
    TooShort(usize),
    TooLong(usize),
    InvalidCharacters(String),
}

impl std::fmt::Display for PhoneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PhoneError::Empty => write!(f, "Phone number cannot be empty"),
            PhoneError::TooShort(len) => write!(f, "Phone number too short: {} characters (min 7)", len),
            PhoneError::TooLong(len) => write!(f, "Phone number too long: {} characters (max 20)", len),
            PhoneError::InvalidCharacters(ch) => write!(f, "Phone number contains invalid characters: {}", ch),
        }
    }
}

impl std::error::Error for PhoneError {}

impl Phone {
    const MIN_LENGTH: usize = 7;
    const MAX_LENGTH: usize = 20;

    /// Create a new Phone value object with validation.
    pub fn new(phone: &str) -> Result<Self, PhoneError> {
        let trimmed = phone.trim().to_string();

        if trimmed.is_empty() {
            return Err(PhoneError::Empty);
        }

        if trimmed.len() < Self::MIN_LENGTH {
            return Err(PhoneError::TooShort(trimmed.len()));
        }

        if trimmed.len() > Self::MAX_LENGTH {
            return Err(PhoneError::TooLong(trimmed.len()));
        }

        // Check for valid characters: digits, +, -, (, ), space
        let invalid_chars: String = trimmed
            .chars()
            .filter(|c| !c.is_ascii_digit() && *c != '+' && *c != '-' && *c != '(' && *c != ')' && *c != ' ')
            .collect();

        if !invalid_chars.is_empty() {
            return Err(PhoneError::InvalidCharacters(invalid_chars));
        }

        Ok(Self(trimmed))
    }

    /// Get the phone number as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Get only the digits from the phone number.
    pub fn digits_only(&self) -> String {
        self.0.chars().filter(|c| c.is_ascii_digit()).collect()
    }

    /// Check if this appears to be an international number (starts with +).
    pub fn is_international(&self) -> bool {
        self.0.starts_with('+')
    }
}

impl std::fmt::Display for Phone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Phone> for String {
    fn from(phone: Phone) -> Self {
        phone.0
    }
}

impl TryFrom<&str> for Phone {
    type Error = PhoneError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_phone() {
        let phone = Phone::new("+1-234-567-8900").unwrap();
        assert_eq!(phone.as_str(), "+1-234-567-8900");
        assert!(phone.is_international());
    }

    #[test]
    fn test_phone_digits_only() {
        let phone = Phone::new("+1-234-567-8900").unwrap();
        assert_eq!(phone.digits_only(), "12345678900");
    }

    #[test]
    fn test_empty_phone() {
        assert!(matches!(Phone::new(""), Err(PhoneError::Empty)));
    }

    #[test]
    fn test_too_short_phone() {
        assert!(matches!(Phone::new("12345"), Err(PhoneError::TooShort(_))));
    }

    #[test]
    fn test_too_long_phone() {
        let long_phone = "1".repeat(25);
        assert!(matches!(Phone::new(&long_phone), Err(PhoneError::TooLong(_))));
    }

    #[test]
    fn test_invalid_characters() {
        assert!(matches!(Phone::new("123-456-7890abc"), Err(PhoneError::InvalidCharacters(_))));
    }

    #[test]
    fn test_phone_trim() {
        let phone = Phone::new("  1234567890  ").unwrap();
        assert_eq!(phone.as_str(), "1234567890");
    }
}
