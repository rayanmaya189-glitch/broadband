use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
#[derive(ToSchema)]
pub struct PermissionResponse {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub module: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
#[derive(ToSchema)]
pub struct MessageResponse {
    pub message: String,
}
