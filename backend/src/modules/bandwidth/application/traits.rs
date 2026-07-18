use crate::shared::errors::AppError;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;

pub type BandwidthProfileModel =
    crate::modules::bandwidth::domain::entities::bandwidth_profile::Model;
pub type BandwidthApplicationModel =
    crate::modules::bandwidth::domain::entities::bandwidth_application::Model;

#[async_trait]
pub trait BandwidthServiceTrait: Send + Sync {
    async fn list_profiles(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<BandwidthProfileModel>, AppError>;

    async fn get_profile(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<BandwidthProfileModel, AppError>;

    async fn create_profile(
        &self,
        db: &DatabaseConnection,
        name: String,
        download_mbps: i32,
        upload_mbps: i32,
    ) -> Result<BandwidthProfileModel, AppError>;

    async fn update_profile(
        &self,
        db: &DatabaseConnection,
        id: i64,
        download_mbps: Option<i32>,
        upload_mbps: Option<i32>,
    ) -> Result<BandwidthProfileModel, AppError>;

    async fn delete_profile(&self, db: &DatabaseConnection, id: i64) -> Result<(), AppError>;
}
