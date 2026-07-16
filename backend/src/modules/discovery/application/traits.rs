use async_trait::async_trait;
use sea_orm::{DatabaseConnection};
use crate::shared::errors::AppError;

pub type DiscoveryScanModel = crate::modules::discovery::domain::entities::discovery_scan::Model;
pub type DiscoveryResultModel = crate::modules::discovery::domain::entities::discovery_result::Model;

#[async_trait]
pub trait DiscoveryServiceTrait: Send + Sync {
    async fn list_scans(
        &self,
        db: &DatabaseConnection,
        branch_id: Option<i64>,
    ) -> Result<Vec<DiscoveryScanModel>, AppError>;

    async fn create_scan(
        &self,
        db: &DatabaseConnection,
        branch_id: i64,
        name: String,
        scan_type: String,
    ) -> Result<DiscoveryScanModel, AppError>;

    async fn list_results(
        &self,
        db: &DatabaseConnection,
        scan_id: i64,
    ) -> Result<Vec<DiscoveryResultModel>, AppError>;

    async fn approve_result(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<DiscoveryResultModel, AppError>;
}
