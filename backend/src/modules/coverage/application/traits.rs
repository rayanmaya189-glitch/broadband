use crate::shared::errors::AppError;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;

pub type CoverageAreaModel = crate::modules::coverage::domain::entities::coverage_area::Model;
pub type CoveragePincodeModel = crate::modules::coverage::domain::entities::coverage_pincode::Model;

#[async_trait]
pub trait CoverageServiceTrait: Send + Sync {
    async fn list_areas(&self, db: &DatabaseConnection)
        -> Result<Vec<CoverageAreaModel>, AppError>;

    async fn create_area(
        &self,
        db: &DatabaseConnection,
        name: String,
        branch_id: i64,
        geo_boundary: Option<serde_json::Value>,
    ) -> Result<CoverageAreaModel, AppError>;

    async fn list_pincodes(
        &self,
        db: &DatabaseConnection,
        area_id: Option<i64>,
    ) -> Result<Vec<CoveragePincodeModel>, AppError>;

    async fn add_pincode(
        &self,
        db: &DatabaseConnection,
        area_id: i64,
        pincode: String,
    ) -> Result<CoveragePincodeModel, AppError>;
}
