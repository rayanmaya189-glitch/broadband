use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};

use crate::modules::discovery::domain::entities::{discovery_result, discovery_scan};
use crate::shared::errors::AppError;

pub struct DiscoveryRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> DiscoveryRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_scan_by_id(
        &self,
        id: i64,
    ) -> Result<Option<discovery_scan::Model>, AppError> {
        Ok(discovery_scan::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn list_scans_by_branch(
        &self,
        branch_id: i64,
    ) -> Result<Vec<discovery_scan::Model>, AppError> {
        Ok(discovery_scan::Entity::find()
            .filter(discovery_scan::Column::BranchId.eq(branch_id))
            .order_by_desc(discovery_scan::Column::CreatedAt)
            .all(self.db)
            .await?)
    }

    pub async fn create_scan(
        &self,
        branch_id: i64,
        name: String,
        scan_type: String,
        target_subnets: Option<serde_json::Value>,
    ) -> Result<discovery_scan::Model, AppError> {
        let now = chrono::Utc::now();
        let model = discovery_scan::ActiveModel {
            branch_id: Set(branch_id),
            name: Set(name),
            scan_type: Set(scan_type),
            target_subnets: Set(target_subnets),
            is_active: Set(true),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(model.insert(self.db).await?)
    }

    pub async fn update_scan(
        &self,
        model: discovery_scan::Model,
        is_active: Option<bool>,
        last_scan_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<discovery_scan::Model, AppError> {
        let mut active: discovery_scan::ActiveModel = model.into();
        if let Some(v) = is_active {
            active.is_active = Set(v);
        }
        if let Some(v) = last_scan_at {
            active.last_scan_at = Set(Some(v));
        }
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(self.db).await?)
    }

    // ── Results ───────────────────────────────────────────────────────

    pub async fn find_result_by_id(
        &self,
        id: i64,
    ) -> Result<Option<discovery_result::Model>, AppError> {
        Ok(discovery_result::Entity::find_by_id(id)
            .one(self.db)
            .await?)
    }

    pub async fn list_results_by_scan(
        &self,
        scan_id: i64,
    ) -> Result<Vec<discovery_result::Model>, AppError> {
        Ok(discovery_result::Entity::find()
            .filter(discovery_result::Column::ScanId.eq(scan_id))
            .order_by_desc(discovery_result::Column::CreatedAt)
            .all(self.db)
            .await?)
    }

    pub async fn list_pending_results(&self) -> Result<Vec<discovery_result::Model>, AppError> {
        Ok(discovery_result::Entity::find()
            .filter(discovery_result::Column::Status.eq("pending"))
            .order_by_desc(discovery_result::Column::CreatedAt)
            .all(self.db)
            .await?)
    }

    pub async fn create_result(
        &self,
        scan_id: i64,
        discovered_ip: String,
        discovered_mac: Option<String>,
        sys_name: Option<String>,
        vendor: Option<String>,
        model: Option<String>,
        firmware_version: Option<String>,
        management_protocol: Option<String>,
    ) -> Result<discovery_result::Model, AppError> {
        let now = chrono::Utc::now();
        let dm = discovery_result::ActiveModel {
            scan_id: Set(scan_id),
            discovered_ip: Set(discovered_ip),
            discovered_mac: Set(discovered_mac),
            sys_name: Set(sys_name),
            vendor: Set(vendor),
            model: Set(model),
            firmware_version: Set(firmware_version),
            management_protocol: Set(management_protocol),
            status: Set("pending".to_string()),
            discovered_at: Set(now),
            created_at: Set(now),
            ..Default::default()
        };
        Ok(dm.insert(self.db).await?)
    }

    pub async fn approve_result(
        &self,
        model: discovery_result::Model,
        reviewer_id: i64,
        matched_device_id: Option<i64>,
    ) -> Result<discovery_result::Model, AppError> {
        let mut active: discovery_result::ActiveModel = model.into();
        active.status = Set("approved".to_string());
        active.reviewed_by = Set(Some(reviewer_id));
        active.matched_device_id = Set(matched_device_id);
        Ok(active.update(self.db).await?)
    }

    pub async fn reject_result(
        &self,
        model: discovery_result::Model,
        reviewer_id: i64,
    ) -> Result<discovery_result::Model, AppError> {
        let mut active: discovery_result::ActiveModel = model.into();
        active.status = Set("rejected".to_string());
        active.reviewed_by = Set(Some(reviewer_id));
        Ok(active.update(self.db).await?)
    }
}
