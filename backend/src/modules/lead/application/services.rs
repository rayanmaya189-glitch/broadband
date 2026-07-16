use crate::modules::lead::domain::entities::{
    Lead, LeadActiveModel, LeadActivity, LeadActivityActiveModel, LeadColumn,
};
use crate::shared::errors::AppError;
use sea_orm::{PaginatorTrait, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub struct LeadService;

impl LeadService {
    pub async fn list_leads(
        db: &DatabaseConnection,
        branch_id: Option<i64>,
        _page: u64,
        _limit: u64,
    ) -> Result<(Vec<crate::modules::lead::domain::entities::lead::Model>, u64), AppError> {
        let mut query = Lead::find();
        if let Some(bid) = branch_id {
            query = query.filter(LeadColumn::BranchId.eq(bid));
        }
        let t = query.clone().count(db).await?;
        Ok((query.all(db).await?, t))
    }

    pub async fn create_lead(
        db: &DatabaseConnection,
        branch_id: i64,
        name: String,
        phone: String,
        email: Option<String>,
        source: String,
    ) -> Result<crate::modules::lead::domain::entities::lead::Model, AppError> {
        let now = chrono::Utc::now();
        let lead = LeadActiveModel {
            branch_id: Set(branch_id),
            name: Set(name),
            phone: Set(phone),
            email: Set(email),
            source: Set(source),
            status: Set("new".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(lead.insert(db).await?)
    }

    pub async fn update_lead_status(
        db: &DatabaseConnection,
        id: i64,
        new_status: &str,
    ) -> Result<crate::modules::lead::domain::entities::lead::Model, AppError> {
        let lead = Lead::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Lead {} not found", id)))?;
        let mut active = <crate::modules::lead::domain::entities::lead::Entity as sea_orm::EntityTrait>::ActiveModel::from(lead);
        active.status = Set(new_status.to_string());
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(db).await?)
    }

    pub async fn log_activity(
        db: &DatabaseConnection,
        lead_id: i64,
        activity_type: String,
        description: String,
        performed_by: i64,
    ) -> Result<crate::modules::lead::domain::entities::lead_activity::Model, AppError> {
        let act = LeadActivityActiveModel {
            lead_id: Set(lead_id),
            activity_type: Set(activity_type),
            description: Set(description),
            performed_by: Set(performed_by),
            created_at: Set(chrono::Utc::now()),
            ..Default::default()
        };
        Ok(act.insert(db).await?)
    }

    pub async fn get_activities(
        db: &DatabaseConnection,
        lead_id: i64,
    ) -> Result<Vec<crate::modules::lead::domain::entities::lead_activity::Model>, AppError> {
        Ok(LeadActivity::find()
            .filter(
                crate::modules::lead::domain::entities::lead_activity::Column::LeadId.eq(lead_id),
            )
            .all(db)
            .await?)
    }
}

