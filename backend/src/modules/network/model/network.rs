use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Vlan {
    pub id: i64,
    pub branch_id: i64,
    pub vlan_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub vlan_type: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct IpPool {
    pub id: i64,
    pub branch_id: i64,
    pub name: String,
    pub cidr: String,
    pub gateway: String,
    pub dns_primary: Option<String>,
    pub dns_secondary: Option<String>,
    pub vlan_id: Option<i64>,
    pub pool_type: String,
    pub allocated_count: i32,
    pub total_count: i32,
    pub status: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct PppoeSession {
    pub id: i64,
    pub branch_id: i64,
    pub customer_id: i64,
    pub subscription_id: i64,
    pub username: String,
    pub assigned_ip: Option<String>,
    pub status: String,
    pub session_start: Option<DateTime<Utc>>,
    pub bytes_in: i64,
    pub bytes_out: i64,
    pub created_at: DateTime<Utc>,
}
