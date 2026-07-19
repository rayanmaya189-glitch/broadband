use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};

use crate::modules::lead::domain::entities::{lead, lead_activity};
use crate::shared::errors::AppError;

pub struct LeadRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> LeadRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<lead::Model>, AppError> {
        Ok(lead::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn list_leads(
        &self,
        branch_id: Option<i64>,
        status: Option<&str>,
        assigned_to: Option<i64>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<lead::Model>, AppError> {
        let mut query = lead::Entity::find();
        if let Some(bid) = branch_id {
            query = query.filter(lead::Column::BranchId.eq(bid));
        }
        if let Some(s) = status {
            query = query.filter(lead::Column::Status.eq(s));
        }
        if let Some(uid) = assigned_to {
            query = query.filter(lead::Column::AssignedTo.eq(uid));
        }
        Ok(query
            .order_by_desc(lead::Column::CreatedAt)
            .limit(limit as u64)
            .offset(offset as u64)
            .all(self.db)
            .await?)
    }

    pub async fn count_leads(&self, branch_id: Option<i64>) -> Result<i64, AppError> {
        let mut query = lead::Entity::find();
        if let Some(bid) = branch_id {
            query = query.filter(lead::Column::BranchId.eq(bid));
        }
        Ok(query.count(self.db).await? as i64)
    }

    pub async fn create_lead(
        &self,
        branch_id: i64,
        name: String,
        phone: String,
        email: Option<String>,
        source: String,
        interested_plan_id: Option<i64>,
        address: Option<String>,
        notes: Option<String>,
        assigned_to: Option<i64>,
    ) -> Result<lead::Model, AppError> {
        let now = chrono::Utc::now();
        let model = lead::ActiveModel {
            branch_id: Set(branch_id),
            name: Set(name),
            phone: Set(phone),
            email: Set(email),
            source: Set(source),
            status: Set("new".to_string()),
            interested_plan_id: Set(interested_plan_id),
            address: Set(address),
            notes: Set(notes),
            assigned_to: Set(assigned_to),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(model.insert(self.db).await?)
    }

    pub async fn update_lead(
        &self,
        model: lead::Model,
        status: Option<String>,
        notes: Option<String>,
        assigned_to: Option<i64>,
    ) -> Result<lead::Model, AppError> {
        let mut active: lead::ActiveModel = model.into();
        if let Some(v) = status {
            active.status = Set(v);
        }
        if let Some(v) = notes {
            active.notes = Set(Some(v));
        }
        if let Some(v) = assigned_to {
            active.assigned_to = Set(Some(v));
        }
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(self.db).await?)
    }

    pub async fn convert_lead(
        &self,
        model: lead::Model,
        customer_id: i64,
    ) -> Result<lead::Model, AppError> {
        let mut active: lead::ActiveModel = model.into();
        active.status = Set("converted".to_string());
        active.converted_customer_id = Set(Some(customer_id));
        active.converted_at = Set(Some(chrono::Utc::now()));
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(self.db).await?)
    }

    // ── Activities ────────────────────────────────────────────────────

    pub async fn add_activity(
        &self,
        lead_id: i64,
        activity_type: String,
        description: String,
        performed_by: i64,
    ) -> Result<lead_activity::Model, AppError> {
        let model = lead_activity::ActiveModel {
            lead_id: Set(lead_id),
            activity_type: Set(activity_type),
            description: Set(description),
            performed_by: Set(performed_by),
            created_at: Set(chrono::Utc::now()),
            ..Default::default()
        };
        Ok(model.insert(self.db).await?)
    }

    pub async fn list_activities(
        &self,
        lead_id: i64,
    ) -> Result<Vec<lead_activity::Model>, AppError> {
        Ok(lead_activity::Entity::find()
            .filter(lead_activity::Column::LeadId.eq(lead_id))
            .order_by_asc(lead_activity::Column::CreatedAt)
            .all(self.db)
            .await?)
    }
}
