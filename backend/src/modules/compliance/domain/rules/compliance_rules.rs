//! Compliance business rules.
//!
//! Domain rules that enforce compliance requirements.

use crate::modules::compliance::domain::aggregates::consent::consent::ConsentError;
use crate::modules::compliance::domain::aggregates::kyc_verification::kyc_verification::KycError;

/// Validate KYC submission.
pub fn validate_kyc_submission(
    id_proof_type: &str,
    id_proof_number: &str,
) -> Result<(), KycError> {
    if id_proof_type.trim().is_empty() {
        return Err(KycError::Validation(
            "ID proof type is required".to_string(),
        ));
    }

    if id_proof_number.trim().is_empty() {
        return Err(KycError::Validation(
            "ID proof number is required".to_string(),
        ));
    }

    let valid_types = ["aadhaar", "pan", "passport", "driving_license", "voter_id"];
    if !valid_types.contains(&id_proof_type.to_lowercase().as_str()) {
        return Err(KycError::Validation(format!(
            "Invalid ID proof type. Must be one of: {}",
            valid_types.join(", ")
        )));
    }

    Ok(())
}

/// Validate consent type.
pub fn validate_consent_type(consent_type: &str) -> Result<(), ConsentError> {
    let valid_types = [
        "data_processing",
        "marketing",
        "third_party_sharing",
        "cookies",
        "location_tracking",
    ];

    if !valid_types.contains(&consent_type.to_lowercase().as_str()) {
        return Err(ConsentError::InvalidConsentType(consent_type.to_string()));
    }

    Ok(())
}

/// Check GDPR compliance for data processing.
pub fn check_gdpr_compliance(
    has_data_consent: bool,
    _has_marketing_consent: bool,
    data_retention_days: u32,
) -> Result<(), String> {
    // GDPR requires explicit consent for data processing
    if !has_data_consent {
        return Err("Data processing consent is required under GDPR".to_string());
    }

    // Check retention period is reasonable (max 365 days without explicit justification)
    if data_retention_days > 365 {
        return Err("Data retention period exceeds 365 days without justification".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_kyc_submission_valid() {
        assert!(validate_kyc_submission("aadhaar", "1234-5678-9012").is_ok());
    }

    #[test]
    fn test_validate_kyc_submission_empty_type() {
        assert!(validate_kyc_submission("", "1234").is_err());
    }

    #[test]
    fn test_validate_kyc_submission_invalid_type() {
        assert!(validate_kyc_submission("invalid", "1234").is_err());
    }

    #[test]
    fn test_check_gdpr_compliance_valid() {
        assert!(check_gdpr_compliance(true, true, 30).is_ok());
    }

    #[test]
    fn test_check_gdpr_compliance_no_consent() {
        assert!(check_gdpr_compliance(false, true, 30).is_err());
    }

    #[test]
    fn test_check_gdpr_compliance_long_retention() {
        assert!(check_gdpr_compliance(true, true, 400).is_err());
    }
}
