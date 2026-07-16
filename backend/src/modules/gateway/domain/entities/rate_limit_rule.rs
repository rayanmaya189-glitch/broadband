use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Rate limit rules for API gateway request throttling.
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "gateway", table_name = "")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// Route pattern (e.g. "/api/v1/customers")
    pub route_pattern: String,
    /// HTTP methods to apply to (comma-separated: GET,POST,PUT,DELETE)
    pub methods: String,
    /// Maximum requests allowed within the window
    pub max_requests: i32,
    /// Window size in seconds
    pub window_seconds: i32,
    /// Optional: apply only to specific role
    pub role: Option<String>,
    /// Optional: apply only to specific branch
    pub branch_id: Option<i64>,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
