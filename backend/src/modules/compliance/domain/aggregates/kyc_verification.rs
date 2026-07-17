use crate::modules::compliance::domain::value_objects::{KycId, KycStatus};

/// KycVerification aggregate root - represents a KYC verification record
#[derive(Debug, Clone, PartialEq)]
pub struct KycVerification {
    pub id: KycId,
    pub customer_id: i64,
    pub document_type: String,
    pub document_url: String,
    pub status: KycStatus,
    pub verified_by: Option<i64>,
    pub rejection_reason: Option<String>,
}

/// Domain errors for KycVerification aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum ComplianceDomainError {
    KycNotFound(i64),
    AlreadyVerified,
    AlreadyRejected,
    InvalidDocumentType,
}

impl std::fmt::Display for ComplianceDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KycNotFound(id) => write!(f, "KYC verification {} not found", id),
            Self::AlreadyVerified => write!(f, "KYC is already verified"),
            Self::AlreadyRejected => write!(f, "KYC is already rejected"),
            Self::InvalidDocumentType => write!(f, "Invalid document type"),
        }
    }
}

impl std::error::Error for ComplianceDomainError {}

impl KycVerification {
    pub fn new(customer_id: i64, document_type: String, document_url: String) -> Result<Self, ComplianceDomainError> {
        let valid_types = ["aadhaar", "pan", "voter_id", "driving_license", "passport"];
        if !valid_types.contains(&document_type.as_str()) {
            return Err(ComplianceDomainError::InvalidDocumentType);
        }
        Ok(Self {
            id: KycId::new(0),
            customer_id,
            document_type,
            document_url,
            status: KycStatus::Pending,
            verified_by: None,
            rejection_reason: None,
        })
    }

    pub fn verify(&mut self, verified_by: i64) -> Result<(), ComplianceDomainError> {
        if self.status == KycStatus::Verified {
            return Err(ComplianceDomainError::AlreadyVerified);
        }
        self.status = KycStatus::Verified;
        self.verified_by = Some(verified_by);
        Ok(())
    }

    pub fn reject(&mut self, reason: String) -> Result<(), ComplianceDomainError> {
        if self.status == KycStatus::Rejected {
            return Err(ComplianceDomainError::AlreadyRejected);
        }
        self.status = KycStatus::Rejected;
        self.rejection_reason = Some(reason);
        Ok(())
    }

    pub fn is_verified(&self) -> bool {
        self.status == KycStatus::Verified
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_kyc() {
        let kyc = KycVerification::new(1, "aadhaar".to_string(), "https://example.com/doc.pdf".to_string());
        assert!(kyc.is_ok());
        assert_eq!(kyc.unwrap().status, KycStatus::Pending);
    }

    #[test]
    fn test_verify_kyc() {
        let mut kyc = KycVerification::new(1, "pan".to_string(), "https://example.com/doc.pdf".to_string()).unwrap();
        kyc.verify(10).unwrap();
        assert!(kyc.is_verified());
    }

    #[test]
    fn test_invalid_document_type() {
        let kyc = KycVerification::new(1, "invalid".to_string(), "url".to_string());
        assert_eq!(kyc, Err(ComplianceDomainError::InvalidDocumentType));
    }
}
