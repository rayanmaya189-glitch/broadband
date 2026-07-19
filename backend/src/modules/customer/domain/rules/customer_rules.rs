//! Customer domain business rules per §07-customers.md.
//! Enforces invariants for customer lifecycle, KYC, and status transitions.

use crate::shared::errors::AppError;

/// Valid customer status transitions per the state machine.
pub fn validate_status_transition(current: &str, target: &str, role: &str) -> Result<(), AppError> {
    let allowed = match (current, target) {
        ("registered", "kyc_pending") => {
            matches!(role, "sales_agent" | "customer_support" | "super_admin")
        }
        ("kyc_pending", "kyc_verified") => {
            matches!(role, "finance_manager" | "customer_support" | "super_admin")
        }
        ("kyc_verified", "installation_scheduled") => {
            matches!(
                role,
                "field_technician" | "customer_support" | "super_admin"
            )
        }
        ("installation_scheduled", "installation_in_progress") => {
            matches!(role, "field_technician" | "super_admin")
        }
        ("installation_in_progress", "active") => {
            matches!(role, "field_technician" | "super_admin")
        }
        ("active", "suspended") => {
            matches!(role, "billing_operator" | "finance_manager" | "super_admin")
        }
        ("active", "terminated") => {
            matches!(role, "customer_support" | "finance_manager" | "super_admin")
        }
        ("suspended", "active") => {
            matches!(role, "billing_operator" | "finance_manager" | "super_admin")
        }
        ("suspended", "terminated") => {
            matches!(role, "customer_support" | "finance_manager" | "super_admin")
        }
        _ => false,
    };

    if allowed {
        Ok(())
    } else {
        Err(AppError::Forbidden(format!(
            "Cannot transition customer from '{}' to '{}' with role '{}'",
            current, target, role
        )))
    }
}

/// Validate customer code format: AX-{BRANCH_CODE}-{YYYYMM}-{SEQUENCE}
pub fn validate_customer_code(code: &str) -> Result<(), AppError> {
    let parts: Vec<&str> = code.split('-').collect();
    if parts.len() != 4 {
        return Err(AppError::Validation(
            "Customer code must have format AX-{BRANCH}-{YYYYMM}-{SEQ}".into(),
        ));
    }
    if parts[0] != "AX" {
        return Err(AppError::Validation(
            "Customer code must start with 'AX'".into(),
        ));
    }
    if parts[2].len() != 6 || !parts[2].chars().all(|c| c.is_ascii_digit()) {
        return Err(AppError::Validation(
            "Customer code date segment must be YYYYMM format".into(),
        ));
    }
    Ok(())
}

/// Validate customer cannot be deleted while having active subscriptions
pub fn validate_deletion(has_active_subscriptions: bool) -> Result<(), AppError> {
    if has_active_subscriptions {
        return Err(AppError::Conflict(
            "Cannot delete customer with active subscriptions".into(),
        ));
    }
    Ok(())
}

/// Validate KYC status for activation
pub fn validate_activation(kyc_status: &str) -> Result<(), AppError> {
    if kyc_status != "verified" {
        return Err(AppError::Validation(
            "Customer cannot be activated without KYC verification".into(),
        ));
    }
    Ok(())
}

/// Validate email format
pub fn validate_email(email: &str) -> Result<(), AppError> {
    if !email.contains('@') || !email.contains('.') {
        return Err(AppError::Validation("Invalid email format".into()));
    }
    if email.len() > 255 {
        return Err(AppError::Validation(
            "Email must be at most 255 characters".into(),
        ));
    }
    Ok(())
}

/// Validate phone format (Indian numbers)
pub fn validate_phone(phone: &str) -> Result<(), AppError> {
    let cleaned: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();
    if cleaned.len() != 10 && !(cleaned.starts_with("91") && cleaned.len() == 12) {
        return Err(AppError::Validation(
            "Phone must be 10 digits or 12 digits with country code".into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_status_transition() {
        assert!(validate_status_transition("registered", "kyc_pending", "sales_agent").is_ok());
        assert!(
            validate_status_transition("kyc_pending", "kyc_verified", "finance_manager").is_ok()
        );
        assert!(validate_status_transition("active", "suspended", "billing_operator").is_ok());
        assert!(validate_status_transition("suspended", "active", "finance_manager").is_ok());
    }

    #[test]
    fn test_invalid_status_transition() {
        assert!(validate_status_transition("registered", "active", "sales_agent").is_err());
        assert!(validate_status_transition("active", "kyc_pending", "super_admin").is_err());
    }

    #[test]
    fn test_customer_code_validation() {
        assert!(validate_customer_code("AX-JLG-202607-0001").is_ok());
        assert!(validate_customer_code("INVALID").is_err());
        assert!(validate_customer_code("AX-JLG-20267-0001").is_err());
    }

    #[test]
    fn test_deletion_with_active_subscriptions() {
        assert!(validate_deletion(true).is_err());
        assert!(validate_deletion(false).is_ok());
    }

    #[test]
    fn test_activation_requires_kyc() {
        assert!(validate_activation("verified").is_ok());
        assert!(validate_activation("pending").is_err());
    }

    #[test]
    fn test_email_validation() {
        assert!(validate_email("test@example.com").is_ok());
        assert!(validate_email("invalid").is_err());
    }

    #[test]
    fn test_phone_validation() {
        assert!(validate_phone("9876543210").is_ok());
        assert!(validate_phone("919876543210").is_ok());
        assert!(validate_phone("123").is_err());
    }
}
