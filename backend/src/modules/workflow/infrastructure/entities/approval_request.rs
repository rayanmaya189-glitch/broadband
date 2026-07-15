use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// SeaORM entity for approval_requests table
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "approval_requests")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub workflow_type: String,
    pub resource_type: String,
    pub resource_id: i64,
    pub requested_by: i64,
    pub branch_id: Option<i64>,
    pub status: String,
    pub payload: Json,
    pub reason: Option<String>,
    pub reviewer_id: Option<i64>,
    pub reviewer_comment: Option<String>,
    pub requested_at: DateTimeWithTimeZone,
    pub reviewed_at: Option<DateTimeWithTimeZone>,
    pub expires_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
