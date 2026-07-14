//! Manage consent command handler.
//!
//! Handles consent grant and revocation for compliance.

use crate::common::errors::app_error::AppError;
use crate::modules::compliance::domain::aggregates::consent::consent::{
    Consent, ConsentEvent, ConsentType,
};

/// Command to grant consent.
#[derive(Debug, Clone)]
pub struct GrantConsentCommand {
    pub customer_id: i64,
    pub consent_type: String,
    pub purpose: String,
    pub version: String,
    pub ip_address: Option<String>,
}

/// Command to revoke consent.
#[derive(Debug, Clone)]
pub struct RevokeConsentCommand {
    pub consent_id: i64,
    pub reason: Option<String>,
}

/// Command handler for granting consent.
pub struct GrantConsentHandler;

impl GrantConsentHandler {
    /// Handle the grant consent command.
    pub fn handle(
        id: i64,
        command: GrantConsentCommand,
    ) -> Result<Consent, AppError> {
        let consent_type = ConsentType::from_str(&command.consent_type)
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let consent = Consent::grant(
            id,
            command.customer_id,
            consent_type,
            command.purpose,
            command.version,
            command.ip_address,
        )
        .map_err(|e| AppError::Validation(e.to_string()))?;

        Ok(consent)
    }
}

/// Command handler for revoking consent.
pub struct RevokeConsentHandler;

impl RevokeConsentHandler {
    /// Handle the revoke consent command.
    pub fn handle(
        mut consent: Consent,
        command: RevokeConsentCommand,
    ) -> Result<ConsentEvent, AppError> {
        consent
            .revoke(command.reason.as_deref())
            .map_err(|e| AppError::Validation(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grant_consent_handler() {
        let command = GrantConsentCommand {
            customer_id: 100,
            consent_type: "data_processing".to_string(),
            purpose: "Service delivery".to_string(),
            version: "1.0".to_string(),
            ip_address: None,
        };

        let consent = GrantConsentHandler::handle(1, command).unwrap();
        assert_eq!(consent.customer_id, 100);
        assert!(consent.is_valid());
    }

    #[test]
    fn test_revoke_consent_handler() {
        let mut consent = Consent::grant(
            1,
            100,
            ConsentType::Marketing,
            "Marketing".to_string(),
            "1.0".to_string(),
            None,
        )
        .unwrap();

        let command = RevokeConsentCommand {
            consent_id: 1,
            reason: Some("Customer requested".to_string()),
        };

        let event = RevokeConsentHandler::handle(consent, command).unwrap();
        match event {
            ConsentEvent::Revoked { reason, .. } => {
                assert_eq!(reason, Some("Customer requested".to_string()));
            }
            _ => panic!("Expected Revoked event"),
        }
    }
}
