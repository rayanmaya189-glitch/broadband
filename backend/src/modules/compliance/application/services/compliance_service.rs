//! Compliance service.
//!
//! Orchestrates compliance operations across the compliance module.

use crate::common::errors::app_error::AppError;
use crate::modules::compliance::domain::aggregates::consent::consent::Consent;
use crate::modules::compliance::domain::aggregates::kyc_verification::kyc_verification::KycVerification;

/// Compliance service that orchestrates module operations.
pub struct ComplianceService;

impl ComplianceService {
    /// Check if a customer meets all compliance requirements.
    pub fn check_compliance(
        kyc: Option<&KycVerification>,
        consents: &[Consent],
    ) -> Result<ComplianceStatus, AppError> {
        let kyc_verified = kyc.map(|k| k.is_valid()).unwrap_or(false);
        let has_data_consent = consents.iter().any(|c| c.is_valid());

        Ok(ComplianceStatus {
            kyc_verified,
            has_data_consent,
            is_compliant: kyc_verified && has_data_consent,
        })
    }
}

/// Customer compliance status.
#[derive(Debug, Clone)]
pub struct ComplianceStatus {
    pub kyc_verified: bool,
    pub has_data_consent: bool,
    pub is_compliant: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::compliance::domain::aggregates::kyc_verification::kyc_verification::{KycType, KycStatus};
    use crate::modules::compliance::domain::aggregates::consent::consent::ConsentType;

    #[test]
    fn test_compliance_check_compliant() {
        let mut kyc = KycVerification::submit(
            1,
            100,
            KycType::Identity,
            None,
            None,
            None,
        ).unwrap();
        kyc.start_review().unwrap();
        kyc.verify(1).unwrap();

        let consent = Consent::grant(
            1,
            100,
            ConsentType::DataProcessing,
            "Data processing".to_string(),
            "1.0".to_string(),
            None,
        ).unwrap();

        let status = ComplianceService::check_compliance(Some(&kyc), &[consent]).unwrap();
        assert!(status.is_compliant);
        assert!(status.kyc_verified);
        assert!(status.has_data_consent);
    }

    #[test]
    fn test_compliance_check_not_compliant_no_kyc() {
        let consent = Consent::grant(
            1,
            100,
            ConsentType::DataProcessing,
            "Data processing".to_string(),
            "1.0".to_string(),
            None,
        ).unwrap();

        let status = ComplianceService::check_compliance(None, &[consent]).unwrap();
        assert!(!status.is_compliant);
    }

    #[test]
    fn test_compliance_check_not_compliant_no_consent() {
        let mut kyc = KycVerification::submit(
            1,
            100,
            KycType::Identity,
            None,
            None,
            None,
        ).unwrap();
        kyc.start_review().unwrap();
        kyc.verify(1).unwrap();

        let status = ComplianceService::check_compliance(Some(&kyc), &[]).unwrap();
        assert!(!status.is_compliant);
    }
}
