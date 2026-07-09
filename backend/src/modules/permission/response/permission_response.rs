use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct PermissionResponse {
    pub id: i64,
    pub name: String,
    pub method: String,
    pub api_url: String,
    pub guard: String,
    pub module: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}
