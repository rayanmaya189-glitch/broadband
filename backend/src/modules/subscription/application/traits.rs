use crate::shared::errors::AppError;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;

pub type SubscriptionModel = crate::modules::subscription::domain::entities::subscription::Model;

#[async_trait]
pub trait SubscriptionServiceTrait: Send + Sync {
    async fn list_subscriptions(
        &self,
        db: &DatabaseConnection,
        branch_id: Option<i64>,
        page: u64,
        limit: u64,
    ) -> Result<(Vec<SubscriptionModel>, u64), AppError>;

    async fn get_subscription(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<SubscriptionModel, AppError>;

    async fn create_subscription(
        &self,
        db: &DatabaseConnection,
        customer_id: i64,
        branch_id: i64,
        plan_id: i64,
        billing_period_months: i32,
    ) -> Result<SubscriptionModel, AppError>;

    async fn cancel_subscription(
        &self,
        db: &DatabaseConnection,
        id: i64,
        reason: &str,
    ) -> Result<SubscriptionModel, AppError>;

    async fn suspend_subscription(
        &self,
        db: &DatabaseConnection,
        id: i64,
        reason: &str,
    ) -> Result<SubscriptionModel, AppError>;
}
