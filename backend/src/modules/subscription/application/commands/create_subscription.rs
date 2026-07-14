//! Create subscription command handler.

use chrono::NaiveDate;

use crate::common::errors::app_error::AppError;
use crate::modules::subscription::domain::aggregates::subscription::subscription::{
    BillingCycle, Subscription,
};
use crate::modules::subscription::domain::rules::subscription_rules;

/// Command to create a subscription.
#[derive(Debug, Clone)]
pub struct CreateSubscriptionCommand {
    pub customer_id: i64,
    pub plan_id: i64,
    pub branch_id: i64,
    pub start_date: NaiveDate,
    pub billing_cycle: String,
}

/// Command handler for creating subscriptions.
pub struct CreateSubscriptionHandler;

impl CreateSubscriptionHandler {
    pub fn handle(id: i64, command: CreateSubscriptionCommand) -> Result<Subscription, AppError> {
        subscription_rules::validate_subscription_creation(command.customer_id, command.plan_id)
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let billing_cycle = match command.billing_cycle.to_lowercase().as_str() {
            "monthly" => BillingCycle::Monthly,
            "quarterly" => BillingCycle::Quarterly,
            "half_yearly" => BillingCycle::HalfYearly,
            "yearly" => BillingCycle::Yearly,
            _ => return Err(AppError::Validation("Invalid billing cycle".to_string())),
        };

        let subscription = Subscription::create(
            id,
            command.customer_id,
            command.plan_id,
            command.branch_id,
            command.start_date,
            billing_cycle,
        )
        .map_err(|e| AppError::Validation(e.to_string()))?;

        Ok(subscription)
    }
}
