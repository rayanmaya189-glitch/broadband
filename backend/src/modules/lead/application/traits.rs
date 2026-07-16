use async_trait::async_trait;
use sea_orm::{DatabaseConnection};
use crate::shared::errors::AppError;

pub type LeadModel = crate::modules::lead::domain::entities::lead::Model;
pub type LeadActivityModel = crate::modules::lead::domain::entities::lead_activity::Model;

#[async_trait]
pub trait LeadServiceTrait: Send + Sync {
    async fn list_leads(
        &self,
        db: &DatabaseConnection,
        branch_id: Option<i64>,
    ) -> Result<Vec<LeadModel>, AppError>;

    async fn create_lead(
        &self,
        db: &DatabaseConnection,
        branch_id: i64,
        name: String,
        phone: String,
        email: Option<String>,
        source: String,
    ) -> Result<LeadModel, AppError>;

    async fn update_lead_status(
        &self,
        db: &DatabaseConnection,
        id: i64,
        status: &str,
    ) -> Result<LeadModel, AppError>;

    async fn log_activity(
        &self,
        db: &DatabaseConnection,
        lead_id: i64,
        activity_type: String,
        description: Option<String>,
        performed_by: i64,
    ) -> Result<LeadActivityModel, AppError>;

    async fn get_activities(
        &self,
        db: &DatabaseConnection,
        lead_id: i64,
    ) -> Result<Vec<LeadActivityModel>, AppError>;
}
