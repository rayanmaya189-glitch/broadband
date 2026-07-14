//! Get subscription query handler.

use crate::common::errors::app_error::AppError;
use crate::modules::subscription::domain::aggregates::subscription::subscription::Subscription;

/// Query to get a subscription by ID.
#[derive(Debug, Clone)]
pub struct GetSubscriptionQuery {
    pub subscription_id: i64,
}

/// Query handler for getting a subscription.
pub struct GetSubscriptionHandler;

impl GetSubscriptionHandler {
    pub fn execute(subscription: Option<Subscription>) -> Result<Subscription, AppError> {
        subscription.ok_or_else(|| AppError::NotFound("Subscription not found".to_string()))
    }
}
