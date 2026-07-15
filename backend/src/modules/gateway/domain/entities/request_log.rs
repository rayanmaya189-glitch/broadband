use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Request logs for gateway analytics and audit trail.
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "gateway_request_logs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: Option<i64>,
    pub branch_id: Option<i64>,
    pub method: String,
    pub path: String,
    pub status_code: i32,
    pub response_time_ms: i32,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub rate_limited: bool,
    pub api_key_id: Option<i64>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
