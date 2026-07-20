use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "ticket", table_name = "")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub ticket_number: String,
    pub branch_id: i64,
    pub customer_id: Option<i64>,
    pub subscription_id: Option<i64>,
    pub created_by: i64,
    pub assigned_to: Option<i64>,
    pub escalated_to: Option<i64>,
    pub category: String,
    pub subcategory: Option<String>,
    pub priority: String,
    pub status: String,
    pub subject: String,
    pub description: String,
    pub source: String,
    pub resolution_notes: Option<String>,
    pub first_response_at: Option<chrono::DateTime<chrono::Utc>>,
    pub resolved_at: Option<chrono::DateTime<chrono::Utc>>,
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub satisfaction_rating: Option<i32>,
    pub satisfaction_feedback: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
