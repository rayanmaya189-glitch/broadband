//! SeaORM-based repository for the Discovery domain.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryFilter, QueryOrder, Set,
};

use crate::common::errors::app_error::AppError;
use crate::modules::discovery::model::discovery_scan_entity::{self, Model as DiscoveryScanModel};
use crate::modules::discovery::model::discovery_result_entity::{self, Model as DiscoveryResultModel};

pub struct DiscoveryRepositorySeaorm<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> DiscoveryRepositorySeaorm<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    pub async fn list_scans(&self, branch_id: Option<i64>) -> Result<Vec<DiscoveryScanModel>, AppError> {
        let mut select = discovery_scan_entity::Entity::find();
        if let Some(bid) = branch_id {
            select = select.filter(discovery_scan_entity::Column::BranchId.eq(bid));
        }
        let scans = select.order_by_desc(discovery_scan_entity::Column::CreatedAt).all(self.db).await?;
        Ok(scans)
    }

    pub async fn create_scan(&self, branch_id: i64, name: &str, scan_type: &str) -> Result<DiscoveryScanModel, AppError> {
        let now = chrono::Utc::now();
        let active = discovery_scan_entity::ActiveModel {
            branch_id: Set(branch_id),
            name: Set(name.to_owned()),
            scan_type: Set(scan_type.to_owned()),
            is_active: Set(true),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn update_scan_active(&self, id: i64, is_active: bool) -> Result<bool, AppError> {
        let existing = discovery_scan_entity::Entity::find_by_id(id).one(self.db).await?;
        match existing {
            Some(e) => {
                let mut active = e.into_active_model();
                active.is_active = Set(is_active);
                active.updated_at = Set(chrono::Utc::now().into());
                active.update(self.db).await?;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    pub async fn list_results(&self, status: Option<&str>, branch_id: Option<i64>) -> Result<Vec<DiscoveryResultModel>, AppError> {
        let mut select = discovery_result_entity::Entity::find();
        if let Some(s) = status {
            select = select.filter(discovery_result_entity::Column::Status.eq(s));
        }
        let results = select.order_by_desc(discovery_result_entity::Column::DiscoveredAt).all(self.db).await?;
        Ok(results)
    }

    pub async fn approve_result(&self, id: i64, reviewed_by: i64) -> Result<DiscoveryResultModel, AppError> {
        let existing = discovery_result_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Result not found".into()))?;
        if existing.status != "pending" {
            return Err(AppError::Validation("Only pending results can be approved".into()));
        }
        let mut active = existing.into_active_model();
        active.status = Set("approved".to_owned());
        active.reviewed_by = Set(Some(reviewed_by));
        active.reviewed_at = Set(Some(chrono::Utc::now().into()));
        Ok(active.update(self.db).await?)
    }

    pub async fn reject_result(&self, id: i64, reviewed_by: i64, reason: &str) -> Result<DiscoveryResultModel, AppError> {
        let existing = discovery_result_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Result not found".into()))?;
        if existing.status != "pending" {
            return Err(AppError::Validation("Only pending results can be rejected".into()));
        }
        let mut active = existing.into_active_model();
        active.status = Set("rejected".to_owned());
        active.reviewed_by = Set(Some(reviewed_by));
        active.reviewed_at = Set(Some(chrono::Utc::now().into()));
        active.rejection_reason = Set(Some(reason.to_owned()));
        Ok(active.update(self.db).await?)
    }
}
