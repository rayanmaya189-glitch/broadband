//! SeaORM-based repository for the Installation domain.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};

use crate::common::errors::app_error::AppError;
use crate::modules::installation::model::installation_order_entity::{self, Model as InstallationOrderModel};

pub struct InstallationRepositorySeaorm<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> InstallationRepositorySeaorm<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    pub async fn list(&self, branch_id: Option<i64>, status: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<InstallationOrderModel>, i64), AppError> {
        let page_size = per_page as u64;
        let page_num = if per_page > 0 { ((page - 1).max(0) as u64 * page_size) / page_size } else { 0 };
        let mut select = installation_order_entity::Entity::find();
        if let Some(bid) = branch_id { select = select.filter(installation_order_entity::Column::BranchId.eq(bid)); }
        if let Some(s) = status { select = select.filter(installation_order_entity::Column::Status.eq(s)); }
        let total = select.clone().count(self.db).await?;
        let orders = select.order_by_desc(installation_order_entity::Column::CreatedAt).paginate(self.db, page_size).fetch_page(page_num).await?;
        Ok((orders, total as i64))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<InstallationOrderModel>, AppError> {
        Ok(installation_order_entity::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn create(&self, customer_id: i64, branch_id: i64, subscription_id: Option<i64>, installation_type: &str) -> Result<InstallationOrderModel, AppError> {
        let now = chrono::Utc::now();
        let active = installation_order_entity::ActiveModel {
            customer_id: Set(customer_id),
            branch_id: Set(branch_id),
            subscription_id: Set(subscription_id),
            status: Set("pending".to_owned()),
            installation_type: Set(installation_type.to_owned()),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn schedule(&self, id: i64, scheduled_date: chrono::NaiveDate, time_slot: &str, technician_id: Option<i64>) -> Result<InstallationOrderModel, AppError> {
        let existing = installation_order_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Installation order not found".into()))?;
        let mut active = existing.into_active_model();
        active.status = Set("scheduled".to_owned());
        active.scheduled_date = Set(Some(scheduled_date));
        active.scheduled_time_slot = Set(Some(time_slot.to_owned()));
        if let Some(tid) = technician_id { active.assigned_technician_id = Set(Some(tid)); }
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn start(&self, id: i64) -> Result<InstallationOrderModel, AppError> {
        let existing = installation_order_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Installation order not found".into()))?;
        let mut active = existing.into_active_model();
        active.status = Set("in_progress".to_owned());
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn complete(&self, id: i64, fiber_length: Option<i32>, onu_power: Option<f64>, equipment: Option<serde_json::Value>, notes: Option<&str>) -> Result<InstallationOrderModel, AppError> {
        let existing = installation_order_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Installation order not found".into()))?;
        let mut active = existing.into_active_model();
        active.status = Set("completed".to_owned());
        active.completed_at = Set(Some(chrono::Utc::now().into()));
        if let Some(fl) = fiber_length { active.fiber_drop_length_meters = Set(Some(fl)); }
        if let Some(op) = onu_power { active.onu_power_dbm = Set(Some(op)); }
        if let Some(e) = equipment { active.equipment_issued = Set(Some(e)); }
        if let Some(n) = notes { active.notes = Set(Some(n.to_owned())); }
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn cancel(&self, id: i64) -> Result<InstallationOrderModel, AppError> {
        let existing = installation_order_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Installation order not found".into()))?;
        let mut active = existing.into_active_model();
        active.status = Set("cancelled".to_owned());
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn add_photo(&self, id: i64, photo_url: &str) -> Result<InstallationOrderModel, AppError> {
        let existing = installation_order_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Installation order not found".into()))?;
        let mut active = existing.into_active_model();
        let mut photos = active.photos.clone().unwrap_or_default();
        photos.push(photo_url.to_owned());
        active.photos = Set(Some(photos));
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn get_my_assignments(&self, technician_id: i64) -> Result<Vec<InstallationOrderModel>, AppError> {
        let orders = installation_order_entity::Entity::find()
            .filter(installation_order_entity::Column::AssignedTechnicianId.eq(technician_id))
            .filter(installation_order_entity::Column::Status.is_in(vec!["scheduled", "in_progress"]))
            .order_by_asc(installation_order_entity::Column::ScheduledDate)
            .all(self.db).await?;
        Ok(orders)
    }
}
