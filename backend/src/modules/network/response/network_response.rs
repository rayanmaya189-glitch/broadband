use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[derive(ToSchema)]
pub struct VlanResponse {
    pub id: i64,
    pub branch_id: i64,
    pub vlan_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub vlan_type: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[derive(ToSchema)]
pub struct IpPoolResponse {
    pub id: i64,
    pub branch_id: i64,
    pub name: String,
    pub cidr: String,
    pub gateway: String,
    pub vlan_id: Option<i64>,
    pub pool_type: String,
    pub allocated_count: i32,
    pub total_count: i32,
    pub status: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[derive(ToSchema)]
pub struct PppoeSessionResponse {
    pub id: i64,
    pub customer_id: i64,
    pub username: String,
    pub assigned_ip: Option<String>,
    pub status: String,
    pub session_start: Option<DateTime<Utc>>,
    pub bytes_in: i64,
    pub bytes_out: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(ToSchema)]
pub struct MessageResponse {
    pub message: String,
}
