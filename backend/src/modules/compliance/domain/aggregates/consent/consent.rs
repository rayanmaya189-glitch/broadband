//! Consent aggregate root.
//!
//! Manages customer consent for data processing, marketing, and third-party sharing.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::shared::events::EventEnvelope;

/// Consent aggregate root.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consent {
    pub id: i64,
    pub customer_id: i64,
    pub consent_type: ConsentType,
    pub status: ConsentStatus,
    pub granted_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub purpose: String,
    pub version: String,
    pub ip_address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Types of consent.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConsentType {
    /// Consent for processing personal data
    DataProcessing,
    /// Consent for marketing communications
    Marketing,
    /// Consent for sharing data with third parties
    ThirdPartySharing,
    /// Consent for cookies and tracking
    Cookies,
    /// Consent for location tracking
    LocationTracking,
}

impl ConsentType {
    /// Convert to string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DataProcessing => "data_processing",
            Self::Marketing => "marketing",
            Self::ThirdPartySharing => "third_party_sharing",
            Self::Cookies => "cookies",
            Self::LocationTracking => "location_tracking",
        }
    }

    /// Parse from string.
    pub fn from_str(s: &str) -> Result<Self, ConsentError> {
        match s.to_lowercase().as_str() {
            "data_processing" | "data" => Ok(Self::DataProcessing),
            "marketing" => Ok(Self::Marketing),
            "third_party_sharing" | "third_party" => Ok(Self::ThirdPartySharing),
            "cookies" => Ok(Self::Cookies),
            "location_tracking" | "location" => Ok(Self::LocationTracking),
            _ => Err(ConsentError::InvalidConsentType(s.to_string())),
        }
    }
}

/// Consent status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConsentStatus {
    Granted,
    Revoked,
    Expired,
    Pending,
}

impl ConsentStatus {
    /// Convert to string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Granted => "granted",
            Self::Revoked => "revoked",
            Self::Expired => "expired",
            Self::Pending => "pending",
        }
    }
}

/// Domain events for Consent aggregate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsentEvent {
    Granted {
        consent_id: i64,
        customer_id: i64,
        consent_type: String,
    },
    Revoked {
        consent_id: i64,
        customer_id: i64,
        consent_type: String,
        reason: Option<String>,
    },
}

/// Consent domain errors.
#[derive(Debug, Error)]
pub enum ConsentError {
    #[error("Invalid consent type: {0}")]
    InvalidConsentType(String),

    #[error("Consent already {0}")]
    AlreadyInState(String),

    #[error("Consent expired")]
    Expired,

    #[error("Validation error: {0}")]
    Validation(String),
}

impl Consent {
    /// Create a new consent record.
    pub fn grant(
        id: i64,
        customer_id: i64,
        consent_type: ConsentType,
        purpose: String,
        version: String,
        ip_address: Option<String>,
    ) -> Result<Self, ConsentError> {
        let now = Utc::now();

        Ok(Self {
            id,
            customer_id,
            consent_type,
            status: ConsentStatus::Granted,
            granted_at: Some(now),
            revoked_at: None,
            expires_at: None,
            purpose,
            version,
            ip_address,
            created_at: now,
            updated_at: now,
        })
    }

    /// Revoke consent.
    pub fn revoke(&mut self, reason: Option<&str>) -> Result<ConsentEvent, ConsentError> {
        if self.status == ConsentStatus::Revoked {
            return Err(ConsentError::AlreadyInState("revoked".to_string()));
        }

        if self.status == ConsentStatus::Expired {
            return Err(ConsentError::Expired);
        }

        self.status = ConsentStatus::Revoked;
        self.revoked_at = Some(Utc::now());
        self.updated_at = Utc::now();

        Ok(ConsentEvent::Revoked {
            consent_id: self.id,
            customer_id: self.customer_id,
            consent_type: self.consent_type.as_str().to_string(),
            reason: reason.map(|s| s.to_string()),
        })
    }

    /// Check if consent is currently valid.
    pub fn is_valid(&self) -> bool {
        if self.status != ConsentStatus::Granted {
            return false;
        }

        if let Some(expires_at) = self.expires_at {
            if Utc::now() > expires_at {
                return false;
            }
        }

        true
    }

    /// Create a domain event for consent granted.
    pub fn grant_event(&self) -> EventEnvelope<ConsentEvent> {
        EventEnvelope::new(
            "compliance.consent.granted.v1".to_string(),
            1,
            "compliance-service".to_string(),
            ConsentEvent::Granted {
                consent_id: self.id,
                customer_id: self.customer_id,
                consent_type: self.consent_type.as_str().to_string(),
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grant_consent() {
        let consent = Consent::grant(
            1,
            100,
            ConsentType::DataProcessing,
            "Data processing for service delivery".to_string(),
            "1.0".to_string(),
            None,
        ).unwrap();

        assert_eq!(consent.id, 1);
        assert_eq!(consent.customer_id, 100);
        assert_eq!(consent.consent_type, ConsentType::DataProcessing);
        assert_eq!(consent.status, ConsentStatus::Granted);
        assert!(consent.is_valid());
    }

    #[test]
    fn test_revoke_consent() {
        let mut consent = Consent::grant(
            1,
            100,
            ConsentType::Marketing,
            "Marketing communications".to_string(),
            "1.0".to_string(),
            None,
        ).unwrap();

        let event = consent.revoke(Some("Customer requested")).unwrap();
        assert_eq!(consent.status, ConsentStatus::Revoked);
        assert!(!consent.is_valid());
        assert!(consent.revoked_at.is_some());

        match event {
            ConsentEvent::Revoked { reason, .. } => {
                assert_eq!(reason, Some("Customer requested".to_string()));
            }
            _ => panic!("Expected Revoked event"),
        }
    }

    #[test]
    fn test_revoke_already_revoked_fails() {
        let mut consent = Consent::grant(
            1,
            100,
            ConsentType::Marketing,
            "Marketing".to_string(),
            "1.0".to_string(),
            None,
        ).unwrap();

        consent.revoke(None).unwrap();
        let result = consent.revoke(None);
        assert!(matches!(result, Err(ConsentError::AlreadyInState(_))));
    }

    #[test]
    fn test_consent_type_conversion() {
        assert_eq!(ConsentType::from_str("data_processing").unwrap(), ConsentType::DataProcessing);
        assert_eq!(ConsentType::from_str("marketing").unwrap(), ConsentType::Marketing);
        assert!(ConsentType::from_str("invalid").is_err());
    }

    #[test]
    fn test_consent_is_valid() {
        let consent = Consent::grant(
            1,
            100,
            ConsentType::DataProcessing,
            "Purpose".to_string(),
            "1.0".to_string(),
            None,
        ).unwrap();

        assert!(consent.is_valid());
    }
}
