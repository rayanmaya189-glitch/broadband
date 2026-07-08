use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct DiscoveryScan {
    pub id: i64,
    pub branch_id: i64,
    pub name: String,
    pub scan_type: String,
    pub is_active: bool,
    pub last_scan_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct DiscoveryResult {
    pub id: i64,
    pub scan_id: i64,
    pub discovered_ip: String,
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub firmware_version: Option<String>,
    pub status: String,
    pub discovered_at: DateTime<Utc>,
}
