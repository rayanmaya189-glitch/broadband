//! Change plan command handler.

use crate::common::errors::app_error::AppError;
use crate::modules::subscription::domain::aggregates::subscription::subscription::{
    Subscription, SubscriptionEvent,
};
use crate::modules::subscription::domain::rules::subscription_rules;

/// Command to change subscription plan.
#[derive(Debug, Clone)]
pub struct ChangePlanCommand {
    pub subscription_id: i64,
    pub new_plan_id: i64,
}

/// Command handler for changing plans.
pub struct ChangePlanHandler;

impl ChangePlanHandler {
    pub fn handle(
        mut subscription: Subscription,
        command: ChangePlanCommand,
    ) -> Result<SubscriptionEvent, AppError> {
        subscription_rules::validate_plan_change(subscription.plan_id, command.new_plan_id)
            .map_err(|e| AppError::Validation(e.to_string()))?;

        subscription
            .change_plan(command.new_plan_id)
            .map_err(|e| AppError::Validation(e.to_string()))
    }
}
