use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[derive(ToSchema)]
pub struct AuditLogResponse {
    pub id: i64,
    pub user_id: Option<i64>,
    pub user_email: Option<String>,
    pub action: String,
    pub resource_type: Option<String>,
    pub result: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(ToSchema)]
pub struct AuditListResponse {
    pub logs: Vec<AuditLogResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}
