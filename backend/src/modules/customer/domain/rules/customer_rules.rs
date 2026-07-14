//! Customer business rules.
//!
//! Domain rules that enforce business invariants for the Customer aggregate.

use crate::modules::customer::domain::aggregates::customer::customer::CustomerError;
use crate::modules::customer::domain::value_objects::customer_status::CustomerStatus;

/// Validate customer creation parameters.
pub fn validate_customer_creation(
    first_name: &str,
    phone: &str,
    email: Option<&str>,
) -> Result<(), CustomerError> {
    // First name validation
    if first_name.trim().is_empty() {
        return Err(CustomerError::Validation(
            "First name is required".to_string(),
        ));
    }
    if first_name.len() > 100 {
        return Err(CustomerError::Validation(
            "First name must be 100 characters or less".to_string(),
        ));
    }

    // Phone validation
    if phone.trim().is_empty() {
        return Err(CustomerError::Validation(
            "Phone number is required".to_string(),
        ));
    }

    // Email validation (optional)
    if let Some(email) = email {
        if !email.contains('@') || !email.contains('.') {
            return Err(CustomerError::Validation(
                "Invalid email format".to_string(),
            ));
        }
    }

    Ok(())
}

/// Validate customer status transition.
pub fn validate_status_transition(
    current_status: &CustomerStatus,
    new_status: &CustomerStatus,
    kyc_status: &str,
    has_active_subscriptions: bool,
) -> Result<(), CustomerError> {
    // Check if transition is allowed
    if !current_status.can_transition_to(new_status) {
        return Err(CustomerError::InvalidStatusTransition(format!(
            "Cannot transition from {} to {}",
            current_status, new_status
        )));
    }

    // KYC must be verified before activation
    if *new_status == CustomerStatus::Active && kyc_status != "verified" {
        return Err(CustomerError::KycRequired);
    }

    // Cannot deactivate with active subscriptions
    if *new_status == CustomerStatus::Deactivated && has_active_subscriptions {
        return Err(CustomerError::ActiveSubscriptionsExist);
    }

    Ok(())
}

/// Validate KYC submission.
pub fn validate_kyc_submission(id_proof_type: &str) -> Result<(), CustomerError> {
    if id_proof_type.trim().is_empty() {
        return Err(CustomerError::Validation(
            "ID proof type is required".to_string(),
        ));
    }

    let valid_types = ["aadhaar", "pan", "passport", "driving_license", "voter_id"];
    if !valid_types.contains(&id_proof_type.to_lowercase().as_str()) {
        return Err(CustomerError::Validation(format!(
            "Invalid ID proof type. Must be one of: {}",
            valid_types.join(", ")
        )));
    }

    Ok(())
}

/// Validate customer code format.
pub fn validate_customer_code(code: &str) -> Result<(), CustomerError> {
    if code.trim().is_empty() {
        return Err(CustomerError::Validation(
            "Customer code cannot be empty".to_string(),
        ));
    }

    // Expected format: AX-{BRANCH_CODE}-{YYYYMM}-{SEQUENCE}
    let parts: Vec<&str> = code.split('-').collect();
    if parts.len() != 4 {
        return Err(CustomerError::Validation(
            "Customer code must be in format AX-{BRANCH}-{YYYYMM}-{SEQUENCE}".to_string(),
        ));
    }

    if parts[0] != "AX" {
        return Err(CustomerError::Validation(
            "Customer code must start with AX-".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_customer_creation_valid() {
        assert!(validate_customer_creation("John", "+1234567890", Some("john@example.com")).is_ok());
    }

    #[test]
    fn test_validate_customer_creation_empty_name() {
        assert!(validate_customer_creation("", "+1234567890", None).is_err());
    }

    #[test]
    fn test_validate_customer_creation_long_name() {
        let long_name = "a".repeat(101);
        assert!(validate_customer_creation(&long_name, "+1234567890", None).is_err());
    }

    #[test]
    fn test_validate_customer_creation_invalid_email() {
        assert!(validate_customer_creation("John", "+1234567890", Some("invalid")).is_err());
    }

    #[test]
    fn test_validate_status_transition_valid() {
        assert!(validate_status_transition(
            &CustomerStatus::Lead,
            &CustomerStatus::Prospect,
            "pending",
            false,
        )
        .is_ok());
    }

    #[test]
    fn test_validate_status_transition_kyc_required() {
        assert!(matches!(
            validate_status_transition(
                &CustomerStatus::Prospect,
                &CustomerStatus::Active,
                "pending",
                false,
            ),
            Err(CustomerError::KycRequired)
        ));
    }

    #[test]
    fn test_validate_kyc_submission_valid() {
        assert!(validate_kyc_submission("aadhaar").is_ok());
        assert!(validate_kyc_submission("pan").is_ok());
    }

    #[test]
    fn test_validate_kyc_submission_invalid() {
        assert!(validate_kyc_submission("invalid").is_err());
    }

    #[test]
    fn test_validate_customer_code_valid() {
        assert!(validate_customer_code("AX-GEN-202607-0001").is_ok());
    }

    #[test]
    fn test_validate_customer_code_invalid() {
        assert!(validate_customer_code("INVALID").is_err());
        assert!(validate_customer_code("BX-GEN-202607-0001").is_err());
    }
}
