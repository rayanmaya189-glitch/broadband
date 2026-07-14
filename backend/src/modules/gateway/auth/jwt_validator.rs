//! JWT token validator.
//!
//! Validates JWT tokens from incoming requests and extracts claims.

use serde::{Deserialize, Serialize};

/// JWT claims extracted from a validated token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    /// Subject (user ID)
    pub sub: i64,
    /// Expiration time (Unix timestamp)
    pub exp: u64,
    /// Issued at (Unix timestamp)
    pub iat: u64,
    /// User's email
    pub email: String,
    /// User's roles
    pub roles: Vec<String>,
    /// Branch ID (if applicable)
    pub branch_id: Option<i64>,
}

/// JWT validation errors.
#[derive(Debug, Clone)]
pub enum JwtError {
    TokenExpired,
    InvalidSignature,
    MalformedToken,
    MissingClaims,
    InvalidIssuer,
}

impl std::fmt::Display for JwtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JwtError::TokenExpired => write!(f, "Token has expired"),
            JwtError::InvalidSignature => write!(f, "Invalid token signature"),
            JwtError::MalformedToken => write!(f, "Malformed token"),
            JwtError::MissingClaims => write!(f, "Missing required claims"),
            JwtError::InvalidIssuer => write!(f, "Invalid token issuer"),
        }
    }
}

impl std::error::Error for JwtError {}

/// JWT validator.
pub struct JwtValidator {
    secret: String,
    issuer: String,
}

impl JwtValidator {
    /// Create a new JWT validator.
    pub fn new(secret: &str, issuer: &str) -> Self {
        Self {
            secret: secret.to_string(),
            issuer: issuer.to_string(),
        }
    }

    /// Validate a JWT token and extract claims.
    pub fn validate(&self, token: &str) -> Result<JwtClaims, JwtError> {
        use jsonwebtoken::{decode, decode_header,  DecodingKey, Validation};

        let header = decode_header(token).map_err(|_| JwtError::MalformedToken)?;

        let mut validation = Validation::new(header.alg);
        validation.set_issuer(&[&self.issuer]);

        let token_data = decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => JwtError::TokenExpired,
            jsonwebtoken::errors::ErrorKind::InvalidSignature => JwtError::InvalidSignature,
            _ => JwtError::MalformedToken,
        })?;

        Ok(token_data.claims)
    }

    /// Check if a user has a specific role.
    pub fn has_role(claims: &JwtClaims, role: &str) -> bool {
        claims.roles.iter().any(|r| r == role)
    }

    /// Check if a user has any of the specified roles.
    pub fn has_any_role(claims: &JwtClaims, roles: &[&str]) -> bool {
        claims.roles.iter().any(|r| roles.contains(&r.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_error_display() {
        assert_eq!(JwtError::TokenExpired.to_string(), "Token has expired");
        assert_eq!(JwtError::InvalidSignature.to_string(), "Invalid token signature");
    }

    #[test]
    fn test_has_role() {
        let claims = JwtClaims {
            sub: 1,
            exp: 9999999999,
            iat: 1000000000,
            email: "test@example.com".to_string(),
            roles: vec!["admin".to_string(), "user".to_string()],
            branch_id: None,
        };

        assert!(JwtValidator::has_role(&claims, "admin"));
        assert!(JwtValidator::has_role(&claims, "user"));
        assert!(!JwtValidator::has_role(&claims, "superadmin"));
    }

    #[test]
    fn test_has_any_role() {
        let claims = JwtClaims {
            sub: 1,
            exp: 9999999999,
            iat: 1000000000,
            email: "test@example.com".to_string(),
            roles: vec!["admin".to_string()],
            branch_id: None,
        };

        assert!(JwtValidator::has_any_role(&claims, &["admin", "superadmin"]));
        assert!(!JwtValidator::has_any_role(&claims, &["user", "guest"]));
    }
}
