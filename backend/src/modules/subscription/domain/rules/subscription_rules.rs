//! Subscription domain business rules per §10-subscriptions.md.
//! Enforces invariants for subscription lifecycle, billing periods, and plan changes.

use crate::shared::errors::AppError;

/// Valid subscription status transitions
pub fn validate_status_transition(current: &str, target: &str) -> Result<(), AppError> {
    let allowed = match (current, target) {
        ("pending", "active") => true,
        ("active", "suspended") => true,
        ("active", "cancelled") => true,
        ("active", "expired") => true,
        ("suspended", "active") => true,
        ("suspended", "cancelled") => true,
        ("cancelled", "pending") => true, // Re-subscription
        _ => false,
    };

    if allowed {
        Ok(())
    } else {
        Err(AppError::Conflict(format!(
            "Cannot transition subscription from '{}' to '{}'",
            current, target
        )))
    }
}

/// Validate subscription creation rules
pub fn validate_creation(
    _customer_has_active: bool,
    plan_is_active: bool,
    customer_status: &str,
) -> Result<(), AppError> {
    if !plan_is_active {
        return Err(AppError::Validation(
            "Cannot subscribe to an inactive plan".into(),
        ));
    }
    if customer_status != "active" && customer_status != "kyc_verified" {
        return Err(AppError::Validation(
            "Customer must be active or KYC verified to subscribe".into(),
        ));
    }
    Ok(())
}

/// Validate upgrade/downgrade eligibility
pub fn validate_plan_change(
    current_status: &str,
    current_plan_id: i64,
    new_plan_id: i64,
) -> Result<(), AppError> {
    if current_status != "active" {
        return Err(AppError::Validation(
            "Only active subscriptions can be upgraded/downgraded".into(),
        ));
    }
    if current_plan_id == new_plan_id {
        return Err(AppError::Validation(
            "New plan must be different from current plan".into(),
        ));
    }
    Ok(())
}

/// Determine if a plan change is an upgrade or downgrade based on price
pub fn classify_plan_change(
    current_price: rust_decimal::Decimal,
    new_price: rust_decimal::Decimal,
) -> &'static str {
    if new_price > current_price {
        "upgrade"
    } else {
        "downgrade"
    }
}

/// Validate billing period
pub fn validate_billing_period(months: i32) -> Result<(), AppError> {
    match months {
        1 | 3 | 6 | 12 => Ok(()),
        _ => Err(AppError::Validation(
            "Billing period must be 1, 3, 6, or 12 months".into(),
        )),
    }
}

/// Calculate pro-rata amount for mid-cycle changes
pub fn calculate_prorata(
    monthly_price: rust_decimal::Decimal,
    days_in_month: i32,
    days_remaining: i32,
) -> rust_decimal::Decimal {
    if days_in_month <= 0 {
        return rust_decimal::Decimal::ZERO;
    }
    let daily_rate = monthly_price / rust_decimal::Decimal::from(days_in_month);
    (daily_rate * rust_decimal::Decimal::from(days_remaining)).round_dp(2)
}

/// Validate auto-renewal can be toggled
pub fn validate_auto_renewal_toggle(
    current_status: &str,
    auto_renew: bool,
) -> Result<(), AppError> {
    if !auto_renew && current_status == "cancelled" {
        return Err(AppError::Validation(
            "Cannot enable auto-renewal for cancelled subscription".into(),
        ));
    }
    Ok(())
}

/// Validate cancellation rules
pub fn validate_cancellation(
    current_status: &str,
    _has_pending_payment: bool,
) -> Result<(), AppError> {
    if current_status == "cancelled" {
        return Err(AppError::Conflict(
            "Subscription is already cancelled".into(),
        ));
    }
    if current_status == "expired" {
        return Err(AppError::Conflict(
            "Cannot cancel an expired subscription".into(),
        ));
    }
    Ok(())
}

/// Validate suspension rules
pub fn validate_suspension(current_status: &str, reason: &str) -> Result<(), AppError> {
    if current_status != "active" {
        return Err(AppError::Validation(
            "Only active subscriptions can be suspended".into(),
        ));
    }
    if reason.is_empty() {
        return Err(AppError::Validation("Suspension reason is required".into()));
    }
    Ok(())
}

/// Validate reactivation rules
pub fn validate_reactivation(
    current_status: &str,
    outstanding_balance: rust_decimal::Decimal,
) -> Result<(), AppError> {
    if current_status != "suspended" {
        return Err(AppError::Validation(
            "Only suspended subscriptions can be reactivated".into(),
        ));
    }
    if outstanding_balance > rust_decimal::Decimal::ZERO {
        return Err(AppError::Validation(
            "Outstanding balance must be cleared before reactivation".into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn test_valid_status_transitions() {
        assert!(validate_status_transition("pending", "active").is_ok());
        assert!(validate_status_transition("active", "suspended").is_ok());
        assert!(validate_status_transition("suspended", "active").is_ok());
        assert!(validate_status_transition("active", "cancelled").is_ok());
    }

    #[test]
    fn test_invalid_status_transitions() {
        assert!(validate_status_transition("cancelled", "active").is_err());
        assert!(validate_status_transition("expired", "active").is_err());
    }

    #[test]
    fn test_billing_period_validation() {
        assert!(validate_billing_period(1).is_ok());
        assert!(validate_billing_period(3).is_ok());
        assert!(validate_billing_period(6).is_ok());
        assert!(validate_billing_period(12).is_ok());
        assert!(validate_billing_period(2).is_err());
    }

    #[test]
    fn test_prorata_calculation() {
        let monthly = Decimal::from(1000);
        let result = calculate_prorata(monthly, 30, 15);
        assert_eq!(result, Decimal::new(500, 0));
    }

    #[test]
    fn test_plan_change_classification() {
        assert_eq!(
            classify_plan_change(Decimal::from(500), Decimal::from(800)),
            "upgrade"
        );
        assert_eq!(
            classify_plan_change(Decimal::from(800), Decimal::from(500)),
            "downgrade"
        );
    }

    #[test]
    fn test_cancellation_validation() {
        assert!(validate_cancellation("active", false).is_ok());
        assert!(validate_cancellation("cancelled", false).is_err());
        assert!(validate_cancellation("expired", false).is_err());
    }

    #[test]
    fn test_suspension_validation() {
        assert!(validate_suspension("active", "non-payment").is_ok());
        assert!(validate_suspension("suspended", "non-payment").is_err());
        assert!(validate_suspension("active", "").is_err());
    }

    #[test]
    fn test_reactivation_validation() {
        assert!(validate_reactivation("suspended", Decimal::ZERO).is_ok());
        assert!(validate_reactivation("active", Decimal::ZERO).is_err());
        assert!(validate_reactivation("suspended", Decimal::from(500)).is_err());
    }
}
