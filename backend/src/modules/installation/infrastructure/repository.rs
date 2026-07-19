use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};

use crate::modules::installation::domain::entities::installation_order;
use crate::shared::errors::AppError;

pub struct InstallationRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> InstallationRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<installation_order::Model>, AppError> {
        Ok(installation_order::Entity::find_by_id(id)
            .one(self.db)
            .await?)
    }

    pub async fn list_installations(
        &self,
        branch_id: Option<i64>,
        status: Option<&str>,
        technician_id: Option<i64>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<installation_order::Model>, AppError> {
        let mut query = installation_order::Entity::find();
        if let Some(bid) = branch_id {
            query = query.filter(installation_order::Column::BranchId.eq(bid));
        }
        if let Some(s) = status {
            query = query.filter(installation_order::Column::Status.eq(s));
        }
        if let Some(tid) = technician_id {
            query = query.filter(installation_order::Column::AssignedTechnicianId.eq(tid));
        }
        Ok(query
            .order_by_desc(installation_order::Column::CreatedAt)
            .limit(limit as u64)
            .offset(offset as u64)
            .all(self.db)
            .await?)
    }

    pub async fn count_installations(&self, branch_id: Option<i64>) -> Result<i64, AppError> {
        let mut query = installation_order::Entity::find();
        if let Some(bid) = branch_id {
            query = query.filter(installation_order::Column::BranchId.eq(bid));
        }
        Ok(query.count(self.db).await? as i64)
    }

    pub async fn create_installation(
        &self,
        customer_id: i64,
        branch_id: i64,
        subscription_id: Option<i64>,
        installation_type: Option<String>,
    ) -> Result<installation_order::Model, AppError> {
        let now = chrono::Utc::now();
        let model = installation_order::ActiveModel {
            customer_id: Set(customer_id),
            branch_id: Set(branch_id),
            subscription_id: Set(subscription_id),
            status: Set("pending".to_string()),
            installation_type: Set(installation_type),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(model.insert(self.db).await?)
    }

    pub async fn schedule_installation(
        &self,
        model: installation_order::Model,
        technician_id: i64,
        scheduled_date: chrono::NaiveDate,
        time_slot: Option<String>,
    ) -> Result<installation_order::Model, AppError> {
        let mut active: installation_order::ActiveModel = model.into();
        active.status = Set("scheduled".to_string());
        active.assigned_technician_id = Set(Some(technician_id));
        active.scheduled_date = Set(Some(scheduled_date));
        active.scheduled_time_slot = Set(time_slot);
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(self.db).await?)
    }

    pub async fn start_installation(
        &self,
        model: installation_order::Model,
    ) -> Result<installation_order::Model, AppError> {
        let mut active: installation_order::ActiveModel = model.into();
        active.status = Set("in_progress".to_string());
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(self.db).await?)
    }

    pub async fn complete_installation(
        &self,
        model: installation_order::Model,
        equipment_issued: Option<serde_json::Value>,
        fiber_drop_length: Option<i32>,
        onu_power_dbm: Option<sea_orm::prelude::Decimal>,
        notes: Option<String>,
    ) -> Result<installation_order::Model, AppError> {
        let mut active: installation_order::ActiveModel = model.into();
        active.status = Set("completed".to_string());
        active.completed_at = Set(Some(chrono::Utc::now()));
        if let Some(v) = equipment_issued {
            active.equipment_issued = Set(Some(v));
        }
        if let Some(v) = fiber_drop_length {
            active.fiber_drop_length_meters = Set(Some(v));
        }
        if let Some(v) = onu_power_dbm {
            active.onu_power_dbm = Set(Some(v));
        }
        if let Some(v) = notes {
            active.notes = Set(Some(v));
        }
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(self.db).await?)
    }

    pub async fn cancel_installation(
        &self,
        model: installation_order::Model,
        reason: Option<String>,
    ) -> Result<installation_order::Model, AppError> {
        let mut active: installation_order::ActiveModel = model.into();
        active.status = Set("cancelled".to_string());
        if let Some(v) = reason {
            active.notes = Set(Some(v));
        }
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(self.db).await?)
    }
}
