//! SeaORM-based repository for the Lead domain.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};

use crate::common::errors::app_error::AppError;
use crate::modules::lead::model::lead_entity::{self, Model as LeadModel};
use crate::modules::lead::model::lead_activity_entity::{self, Model as LeadActivityModel};

pub struct LeadRepositorySeaorm<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> LeadRepositorySeaorm<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    pub async fn list(&self, branch_id: Option<i64>, status: Option<&str>, source: Option<&str>, assigned_to: Option<i64>, page: i64, per_page: i64) -> Result<(Vec<LeadModel>, i64), AppError> {
        let page_size = per_page as u64;
        let page_num = if per_page > 0 { ((page - 1).max(0) as u64 * page_size) / page_size } else { 0 };
        let mut select = lead_entity::Entity::find();
        if let Some(bid) = branch_id { select = select.filter(lead_entity::Column::BranchId.eq(bid)); }
        if let Some(s) = status { select = select.filter(lead_entity::Column::Status.eq(s)); }
        if let Some(src) = source { select = select.filter(lead_entity::Column::Source.eq(src)); }
        if let Some(at) = assigned_to { select = select.filter(lead_entity::Column::AssignedTo.eq(at)); }
        let total = select.clone().count(self.db).await?;
        let leads = select.order_by_desc(lead_entity::Column::CreatedAt).paginate(self.db, page_size).fetch_page(page_num).await?;
        Ok((leads, total as i64))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<LeadModel>, AppError> {
        Ok(lead_entity::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn create(&self, branch_id: i64, name: &str, phone: &str, email: Option<&str>, source: &str, interested_plan_id: Option<i64>, estimated_install_date: Option<chrono::NaiveDate>, address: Option<&str>, latitude: Option<f64>, longitude: Option<f64>, notes: Option<&str>) -> Result<LeadModel, AppError> {
        let now = chrono::Utc::now();
        let active = lead_entity::ActiveModel {
            branch_id: Set(branch_id),
            name: Set(name.to_owned()),
            phone: Set(phone.to_owned()),
            email: Set(email.map(|s| s.to_owned())),
            source: Set(source.to_owned()),
            status: Set("new".to_owned()),
            interested_plan_id: Set(interested_plan_id),
            estimated_install_date: Set(estimated_install_date),
            address: Set(address.map(|s| s.to_owned())),
            latitude: Set(latitude),
            longitude: Set(longitude),
            notes: Set(notes.map(|s| s.to_owned())),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn update(&self, id: i64, name: Option<&str>, phone: Option<&str>, email: Option<&str>, source: Option<&str>, interested_plan_id: Option<i64>, estimated_install_date: Option<chrono::NaiveDate>, address: Option<&str>, latitude: Option<f64>, longitude: Option<f64>, notes: Option<&str>) -> Result<LeadModel, AppError> {
        let existing = lead_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Lead not found".into()))?;
        let mut active = existing.into_active_model();
        if let Some(v) = name { active.name = Set(v.to_owned()); }
        if let Some(v) = phone { active.phone = Set(v.to_owned()); }
        if let Some(v) = email { active.email = Set(Some(v.to_owned())); }
        if let Some(v) = source { active.source = Set(v.to_owned()); }
        if let Some(v) = interested_plan_id { active.interested_plan_id = Set(Some(v)); }
        if let Some(v) = estimated_install_date { active.estimated_install_date = Set(Some(v)); }
        if let Some(v) = address { active.address = Set(Some(v.to_owned())); }
        if let Some(v) = latitude { active.latitude = Set(Some(v)); }
        if let Some(v) = longitude { active.longitude = Set(Some(v)); }
        if let Some(v) = notes { active.notes = Set(Some(v.to_owned())); }
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn update_status(&self, id: i64, status: &str, lost_reason: Option<&str>) -> Result<LeadModel, AppError> {
        let existing = lead_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Lead not found".into()))?;
        let mut active = existing.into_active_model();
        active.status = Set(status.to_owned());
        if status == "lost" { active.lost_reason = Set(lost_reason.map(|s| s.to_owned())); }
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn assign(&self, id: i64, assigned_to: i64) -> Result<LeadModel, AppError> {
        let existing = lead_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Lead not found".into()))?;
        let mut active = existing.into_active_model();
        active.assigned_to = Set(Some(assigned_to));
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn convert(&self, id: i64, customer_id: i64) -> Result<LeadModel, AppError> {
        let existing = lead_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Lead not found".into()))?;
        let mut active = existing.into_active_model();
        active.status = Set("converted".to_owned());
        active.converted_customer_id = Set(Some(customer_id));
        active.converted_at = Set(Some(chrono::Utc::now().into()));
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn delete(&self, id: i64) -> Result<bool, AppError> {
        let result = lead_entity::Entity::delete_by_id(id).exec(self.db).await?;
        Ok(result.rows_affected > 0)
    }

    pub async fn list_activities(&self, lead_id: i64) -> Result<Vec<LeadActivityModel>, AppError> {
        let activities = lead_activity_entity::Entity::find()
            .filter(lead_activity_entity::Column::LeadId.eq(lead_id))
            .order_by_asc(lead_activity_entity::Column::CreatedAt)
            .all(self.db).await?;
        Ok(activities)
    }

    pub async fn add_activity(&self, lead_id: i64, activity_type: &str, description: &str, performed_by: i64, scheduled_at: Option<chrono::DateTime<chrono::Utc>>) -> Result<LeadActivityModel, AppError> {
        let now = chrono::Utc::now();
        let active = lead_activity_entity::ActiveModel {
            lead_id: Set(lead_id),
            activity_type: Set(activity_type.to_owned()),
            description: Set(description.to_owned()),
            performed_by: Set(performed_by),
            scheduled_at: Set(scheduled_at),
            created_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }
}
