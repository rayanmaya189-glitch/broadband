use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set};
use crate::shared::errors::AppError;
use crate::modules::discovery::domain::entities::{DiscoveryScan, DiscoveryResult, DiscoveryScanActiveModel};

pub struct DiscoveryService;

impl DiscoveryService {
    pub async fn list_scans(db: &DatabaseConnection) -> Result<Vec<crate::modules::discovery::domain::entities::discovery_scan::Model>, AppError> {
        Ok(DiscoveryScan::find().all(db).await?)
    }

    pub async fn create_scan(db: &DatabaseConnection, branch_id: i64, name: String, scan_type: String) -> Result<crate::modules::discovery::domain::entities::discovery_scan::Model, AppError> {
        let now = chrono::Utc::now();
        let scan = DiscoveryScanActiveModel {
            branch_id: Set(branch_id), name: Set(name), scan_type: Set(scan_type),
            is_active: Set(true), created_at: Set(now), updated_at: Set(now), ..Default::default()
        };
        Ok(scan.insert(db).await?)
    }

    pub async fn list_results(db: &DatabaseConnection) -> Result<Vec<crate::modules::discovery::domain::entities::discovery_result::Model>, AppError> {
        Ok(DiscoveryResult::find().all(db).await?)
    }

    pub async fn approve_result(db: &DatabaseConnection, id: i64, reviewed_by: i64) -> Result<crate::modules::discovery::domain::entities::discovery_result::Model, AppError> {
        let result = DiscoveryResult::find_by_id(id).one(db).await?.ok_or_else(|| AppError::NotFound(format!("Result {} not found", id)))?;
        let mut active = <crate::modules::discovery::domain::entities::discovery_result::Entity as sea_orm::EntityTrait>::ActiveModel::from(result);
        active.status = Set("approved".to_string());
        active.reviewed_by = Set(Some(reviewed_by));
        Ok(active.update(db).await?)
    }
}
