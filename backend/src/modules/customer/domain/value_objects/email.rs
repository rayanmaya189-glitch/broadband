//! Email value object.
//!
//! An immutable value object representing a validated email address.

use serde::{Deserialize, Serialize};

/// Email value object with validation.
///
/// # Invariants
/// - Must contain exactly one `@` symbol
/// - Local part must not be empty
/// - Domain part must not be empty
/// - Maximum length: 255 characters
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Email(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmailError {
    Empty,
    NoAtSymbol,
    MultipleAtSymbols,
    EmptyLocalPart,
    EmptyDomainPart,
    TooLong(usize),
    InvalidFormat(String),
}

impl std::fmt::Display for EmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmailError::Empty => write!(f, "Email cannot be empty"),
            EmailError::NoAtSymbol => write!(f, "Email must contain an '@' symbol"),
            EmailError::MultipleAtSymbols => write!(f, "Email must contain exactly one '@' symbol"),
            EmailError::EmptyLocalPart => write!(f, "Email local part cannot be empty"),
            EmailError::EmptyDomainPart => write!(f, "Email domain part cannot be empty"),
            EmailError::TooLong(len) => write!(f, "Email too long: {} characters (max 255)", len),
            EmailError::InvalidFormat(msg) => write!(f, "Invalid email format: {}", msg),
        }
    }
}

impl std::error::Error for EmailError {}

impl Email {
    /// Maximum allowed email length.
    const MAX_LENGTH: usize = 255;

    /// Create a new Email value object with validation.
    pub fn new(email: &str) -> Result<Self, EmailError> {
        let trimmed = email.trim().to_lowercase();

        if trimmed.is_empty() {
            return Err(EmailError::Empty);
        }

        if trimmed.len() > Self::MAX_LENGTH {
            return Err(EmailError::TooLong(trimmed.len()));
        }

        let at_count = trimmed.matches('@').count();
        if at_count == 0 {
            return Err(EmailError::NoAtSymbol);
        }
        if at_count > 1 {
            return Err(EmailError::MultipleAtSymbols);
        }

        let parts: Vec<&str> = trimmed.split('@').collect();
        let local = parts[0];
        let domain = parts[1];

        if local.is_empty() {
            return Err(EmailError::EmptyLocalPart);
        }
        if domain.is_empty() {
            return Err(EmailError::EmptyDomainPart);
        }

        // Basic domain validation
        if !domain.contains('.') {
            return Err(EmailError::InvalidFormat(
                "Domain must contain at least one dot".to_string(),
            ));
        }

        Ok(Self(trimmed))
    }

    /// Get the email as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Get the local part (before @).
    pub fn local_part(&self) -> &str {
        self.0.split('@').next().unwrap_or("")
    }

    /// Get the domain part (after @).
    pub fn domain(&self) -> &str {
        self.0.split('@').nth(1).unwrap_or("")
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}

impl TryFrom<&str> for Email {
    type Error = EmailError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        let email = Email::new("test@example.com").unwrap();
        assert_eq!(email.as_str(), "test@example.com");
        assert_eq!(email.local_part(), "test");
        assert_eq!(email.domain(), "example.com");
    }

    #[test]
    fn test_email_case_normalization() {
        let email = Email::new("Test@Example.COM").unwrap();
        assert_eq!(email.as_str(), "test@example.com");
    }

    #[test]
    fn test_email_trim() {
        let email = Email::new("  test@example.com  ").unwrap();
        assert_eq!(email.as_str(), "test@example.com");
    }

    #[test]
    fn test_empty_email() {
        assert!(matches!(Email::new(""), Err(EmailError::Empty)));
    }

    #[test]
    fn test_no_at_symbol() {
        assert!(matches!(Email::new("testexample.com"), Err(EmailError::NoAtSymbol)));
    }

    #[test]
    fn test_multiple_at_symbols() {
        assert!(matches!(Email::new("test@exam@ple.com"), Err(EmailError::MultipleAtSymbols)));
    }

    #[test]
    fn test_empty_local_part() {
        assert!(matches!(Email::new("@example.com"), Err(EmailError::EmptyLocalPart)));
    }

    #[test]
    fn test_empty_domain_part() {
        assert!(matches!(Email::new("test@"), Err(EmailError::EmptyDomainPart)));
    }

    #[test]
    fn test_domain_without_dot() {
        assert!(matches!(Email::new("test@examplecom"), Err(EmailError::InvalidFormat(_))));
    }

    #[test]
    fn test_too_long_email() {
        let long_email = format!("{}@example.com", "a".repeat(250));
        assert!(matches!(Email::new(&long_email), Err(EmailError::TooLong(_))));
    }
}
