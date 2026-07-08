use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[derive(ToSchema)]
pub struct InventoryItemResponse {
    pub id: i64,
    pub branch_id: i64,
    pub item_type: String,
    pub serial_number: Option<String>,
    pub status: String,
    pub assigned_to: Option<i64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(ToSchema)]
pub struct InventoryListResponse {
    pub items: Vec<InventoryItemResponse>,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(ToSchema)]
pub struct MessageResponse { pub message: String }
