//! KYC verification aggregate root.
//!
//! Manages Know Your Customer verification process.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::shared::events::EventEnvelope;

/// KYC verification aggregate root.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KycVerification {
    pub id: i64,
    pub customer_id: i64,
    pub verification_type: KycType,
    pub status: KycStatus,
    pub id_proof_type: Option<String>,
    pub id_proof_number: Option<String>,
    pub document_url: Option<String>,
    pub verified_by: Option<i64>,
    pub verified_at: Option<DateTime<Utc>>,
    pub rejection_reason: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Types of KYC verification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KycType {
    /// Basic identity verification
    Identity,
    /// Address verification
    Address,
    /// Income verification
    Income,
    /// Full KYC verification
    Full,
}

impl KycType {
    /// Convert to string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::Address => "address",
            Self::Income => "income",
            Self::Full => "full",
        }
    }

    /// Parse from string.
    pub fn from_str(s: &str) -> Result<Self, KycError> {
        match s.to_lowercase().as_str() {
            "identity" => Ok(Self::Identity),
            "address" => Ok(Self::Address),
            "income" => Ok(Self::Income),
            "full" => Ok(Self::Full),
            _ => Err(KycError::InvalidKycType(s.to_string())),
        }
    }
}

/// KYC verification status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KycStatus {
    Pending,
    InReview,
    Verified,
    Rejected,
    Expired,
}

impl KycStatus {
    /// Convert to string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::InReview => "in_review",
            Self::Verified => "verified",
            Self::Rejected => "rejected",
            Self::Expired => "expired",
        }
    }
}

/// Domain events for KYC verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KycEvent {
    Submitted {
        kyc_id: i64,
        customer_id: i64,
        verification_type: String,
    },
    Verified {
        kyc_id: i64,
        customer_id: i64,
        verified_by: i64,
    },
    Rejected {
        kyc_id: i64,
        customer_id: i64,
        reason: String,
    },
}

/// KYC domain errors.
#[derive(Debug, Error)]
pub enum KycError {
    #[error("Invalid KYC type: {0}")]
    InvalidKycType(String),

    #[error("Invalid status transition: {0}")]
    InvalidTransition(String),

    #[error("Validation error: {0}")]
    Validation(String),
}

impl KycVerification {
    /// Create a new KYC verification request.
    pub fn submit(
        id: i64,
        customer_id: i64,
        verification_type: KycType,
        id_proof_type: Option<String>,
        id_proof_number: Option<String>,
        document_url: Option<String>,
    ) -> Result<Self, KycError> {
        let now = Utc::now();

        Ok(Self {
            id,
            customer_id,
            verification_type,
            status: KycStatus::Pending,
            id_proof_type,
            id_proof_number,
            document_url,
            verified_by: None,
            verified_at: None,
            rejection_reason: None,
            expires_at: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// Mark KYC as in review.
    pub fn start_review(&mut self) -> Result<(), KycError> {
        if self.status != KycStatus::Pending {
            return Err(KycError::InvalidTransition(format!(
                "Cannot start review from {}",
                self.status.as_str()
            )));
        }

        self.status = KycStatus::InReview;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Verify KYC.
    pub fn verify(&mut self, verified_by: i64) -> Result<KycEvent, KycError> {
        if self.status != KycStatus::InReview {
            return Err(KycError::InvalidTransition(format!(
                "Cannot verify from {}",
                self.status.as_str()
            )));
        }

        self.status = KycStatus::Verified;
        self.verified_by = Some(verified_by);
        self.verified_at = Some(Utc::now());
        self.updated_at = Utc::now();

        Ok(KycEvent::Verified {
            kyc_id: self.id,
            customer_id: self.customer_id,
            verified_by,
        })
    }

    /// Reject KYC.
    pub fn reject(&mut self, reason: &str) -> Result<KycEvent, KycError> {
        if self.status != KycStatus::InReview {
            return Err(KycError::InvalidTransition(format!(
                "Cannot reject from {}",
                self.status.as_str()
            )));
        }

        if reason.trim().is_empty() {
            return Err(KycError::Validation(
                "Rejection reason is required".to_string(),
            ));
        }

        self.status = KycStatus::Rejected;
        self.rejection_reason = Some(reason.to_string());
        self.updated_at = Utc::now();

        Ok(KycEvent::Rejected {
            kyc_id: self.id,
            customer_id: self.customer_id,
            reason: reason.to_string(),
        })
    }

    /// Check if KYC is valid (verified and not expired).
    pub fn is_valid(&self) -> bool {
        if self.status != KycStatus::Verified {
            return false;
        }

        if let Some(expires_at) = self.expires_at {
            if Utc::now() > expires_at {
                return false;
            }
        }

        true
    }

    /// Create a domain event for KYC submission.
    pub fn submission_event(&self) -> EventEnvelope<KycEvent> {
        EventEnvelope::new(
            "compliance.kyc.submitted.v1".to_string(),
            1,
            "compliance-service".to_string(),
            KycEvent::Submitted {
                kyc_id: self.id,
                customer_id: self.customer_id,
                verification_type: self.verification_type.as_str().to_string(),
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_submit_kyc() {
        let kyc = KycVerification::submit(
            1,
            100,
            KycType::Identity,
            Some("aadhaar".to_string()),
            Some("1234-5678-9012".to_string()),
            None,
        ).unwrap();

        assert_eq!(kyc.id, 1);
        assert_eq!(kyc.customer_id, 100);
        assert_eq!(kyc.status, KycStatus::Pending);
    }

    #[test]
    fn test_verify_kyc() {
        let mut kyc = KycVerification::submit(
            1,
            100,
            KycType::Identity,
            None,
            None,
            None,
        ).unwrap();

        kyc.start_review().unwrap();
        let event = kyc.verify(1).unwrap();

        assert_eq!(kyc.status, KycStatus::Verified);
        assert!(kyc.is_valid());

        match event {
            KycEvent::Verified { verified_by, .. } => {
                assert_eq!(verified_by, 1);
            }
            _ => panic!("Expected Verified event"),
        }
    }

    #[test]
    fn test_reject_kyc() {
        let mut kyc = KycVerification::submit(
            1,
            100,
            KycType::Identity,
            None,
            None,
            None,
        ).unwrap();

        kyc.start_review().unwrap();
        let event = kyc.reject("Invalid document").unwrap();

        assert_eq!(kyc.status, KycStatus::Rejected);
        assert!(!kyc.is_valid());

        match event {
            KycEvent::Rejected { reason, .. } => {
                assert_eq!(reason, "Invalid document");
            }
            _ => panic!("Expected Rejected event"),
        }
    }

    #[test]
    fn test_invalid_transition() {
        let mut kyc = KycVerification::submit(
            1,
            100,
            KycType::Identity,
            None,
            None,
            None,
        ).unwrap();

        // Cannot verify from pending
        let result = kyc.verify(1);
        assert!(matches!(result, Err(KycError::InvalidTransition(_))));
    }
}
