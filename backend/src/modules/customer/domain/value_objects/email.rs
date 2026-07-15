use serde::{Deserialize, Serialize};
use std::fmt;

/// Email value object with validation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    /// Create a new Email with validation
    pub fn new(
        email: &str,
    ) -> Result<Self, crate::modules::customer::domain::aggregates::customer::CustomerDomainError>
    {
        if !Self::is_valid(email) {
            return Err(crate::modules::customer::domain::aggregates::customer::CustomerDomainError::InvalidEmail);
        }
        Ok(Self(email.to_lowercase()))
    }

    /// Validate email format
    fn is_valid(email: &str) -> bool {
        // Basic email validation
        if email.is_empty() || email.len() > 255 {
            return false;
        }

        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 {
            return false;
        }

        let local = parts[0];
        let domain = parts[1];

        // Local part validation
        if local.is_empty() || local.len() > 64 {
            return false;
        }

        // Domain validation
        if domain.is_empty() || domain.len() > 253 {
            return false;
        }

        // Check for valid characters
        let valid_local_chars =
            |c: char| c.is_alphanumeric() || c == '.' || c == '_' || c == '+' || c == '-';
        let valid_domain_chars = |c: char| c.is_alphanumeric() || c == '.' || c == '-';

        if !local.chars().all(valid_local_chars) || !domain.chars().all(valid_domain_chars) {
            return false;
        }

        // Must have at least one dot in domain
        domain.contains('.')
    }

    /// Get the email as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Get the local part of the email
    pub fn local_part(&self) -> &str {
        self.0.split('@').next().unwrap_or("")
    }

    /// Get the domain part of the email
    pub fn domain(&self) -> &str {
        self.0.split('@').last().unwrap_or("")
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        assert!(Email::new("user@example.com").is_ok());
        assert!(Email::new("user.name@example.com").is_ok());
        assert!(Email::new("user+tag@example.com").is_ok());
        assert!(Email::new("user-name@example.com").is_ok());
        assert!(Email::new("user_name@example.com").is_ok());
    }

    #[test]
    fn test_invalid_email() {
        assert!(Email::new("").is_err());
        assert!(Email::new("invalid").is_err());
        assert!(Email::new("@example.com").is_err());
        assert!(Email::new("user@").is_err());
        assert!(Email::new("user@.com").is_err());
        assert!(Email::new("user@example").is_err());
    }

    #[test]
    fn test_email_normalization() {
        let email = Email::new("USER@Example.COM").unwrap();
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn test_email_parts() {
        let email = Email::new("user@example.com").unwrap();
        assert_eq!(email.local_part(), "user");
        assert_eq!(email.domain(), "example.com");
    }
}
