//! Subscription repository trait.

use async_trait::async_trait;

use crate::common::errors::app_error::AppError;
use crate::modules::subscription::domain::aggregates::subscription::subscription::Subscription;

/// Repository trait for subscription operations.
#[async_trait]
pub trait SubscriptionRepositoryTrait: Send + Sync {
    /// Find subscription by ID.
    async fn find_by_id(&self, id: i64) -> Result<Option<Subscription>, AppError>;

    /// Save a subscription.
    async fn save(&self, subscription: &mut Subscription) -> Result<(), AppError>;

    /// Update a subscription.
    async fn update(&self, subscription: &Subscription) -> Result<(), AppError>;

    /// List subscriptions for a customer.
    async fn list_by_customer(&self, customer_id: i64) -> Result<Vec<Subscription>, AppError>;
}
