//! Subscription business rules.

use crate::modules::subscription::domain::aggregates::subscription::subscription::SubscriptionError;

/// Validate subscription creation parameters.
pub fn validate_subscription_creation(
    customer_id: i64,
    plan_id: i64,
) -> Result<(), SubscriptionError> {
    if customer_id <= 0 {
        return Err(SubscriptionError::Validation(
            "Customer ID must be positive".to_string(),
        ));
    }

    if plan_id <= 0 {
        return Err(SubscriptionError::Validation(
            "Plan ID must be positive".to_string(),
        ));
    }

    Ok(())
}

/// Check if plan change is allowed.
pub fn validate_plan_change(
    current_plan_id: i64,
    new_plan_id: i64,
) -> Result<(), SubscriptionError> {
    if current_plan_id == new_plan_id {
        return Err(SubscriptionError::Validation(
            "New plan must be different from current plan".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_subscription_creation_valid() {
        assert!(validate_subscription_creation(100, 200).is_ok());
    }

    #[test]
    fn test_validate_subscription_creation_invalid_customer() {
        assert!(validate_subscription_creation(0, 200).is_err());
    }

    #[test]
    fn test_validate_plan_change_valid() {
        assert!(validate_plan_change(100, 200).is_ok());
    }

    #[test]
    fn test_validate_plan_change_same_plan() {
        assert!(validate_plan_change(100, 100).is_err());
    }
}
