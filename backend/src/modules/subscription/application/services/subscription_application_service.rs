//! Subscription application service.

use crate::common::errors::app_error::AppError;
use crate::modules::subscription::domain::aggregates::subscription::subscription::Subscription;

/// Application service for subscription operations.
pub struct SubscriptionApplicationService;

impl SubscriptionApplicationService {
    /// Create a new subscription.
    pub async fn create_subscription(
        &self,
        id: i64,
        command: crate::modules::subscription::application::commands::create_subscription::CreateSubscriptionCommand,
    ) -> Result<Subscription, AppError> {
        crate::modules::subscription::application::commands::create_subscription::CreateSubscriptionHandler::handle(id, command)
    }

    /// Change subscription plan.
    pub async fn change_plan(
        &self,
        subscription: Subscription,
        command: crate::modules::subscription::application::commands::change_plan::ChangePlanCommand,
    ) -> Result<(), AppError> {
        crate::modules::subscription::application::commands::change_plan::ChangePlanHandler::handle(subscription, command)?;
        Ok(())
    }
}
