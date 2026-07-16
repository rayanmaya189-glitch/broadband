use async_trait::async_trait;
use sea_orm::{DatabaseConnection};
use crate::shared::errors::AppError;

pub type InstallationOrderModel = crate::modules::installation::domain::entities::installation_order::Model;

#[async_trait]
pub trait InstallationServiceTrait: Send + Sync {
    async fn list_orders(
        &self,
        db: &DatabaseConnection,
        branch_id: Option<i64>,
    ) -> Result<Vec<InstallationOrderModel>, AppError>;

    async fn create_order(
        &self,
        db: &DatabaseConnection,
        customer_id: i64,
        branch_id: i64,
        subscription_id: Option<i64>,
    ) -> Result<InstallationOrderModel, AppError>;

    async fn schedule_order(
        &self,
        db: &DatabaseConnection,
        id: i64,
        scheduled_date: chrono::NaiveDate,
    ) -> Result<InstallationOrderModel, AppError>;

    async fn complete_order(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<InstallationOrderModel, AppError>;
}
