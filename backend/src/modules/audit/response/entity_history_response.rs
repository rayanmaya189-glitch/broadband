use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct EntityHistoryResponse {
    pub id: i64,
    pub entity_type: String,
    pub entity_id: i64,
    pub action: String,
    pub old_data: Option<serde_json::Value>,
    pub new_data: Option<serde_json::Value>,
    pub changed_fields: Option<Vec<String>>,
    pub user_id: Option<i64>,
    pub reason: Option<String>,
    pub rollback_reference: Option<i64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EntityHistoryListResponse {
    pub entries: Vec<EntityHistoryResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EntityHistoryStatsResponse {
    pub total_entries: i64,
    pub total_entities: i64,
    pub total_rollbacks: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}
