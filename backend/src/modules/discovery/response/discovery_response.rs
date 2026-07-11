use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DiscoveryScanResponse {
    pub id: i64,
    pub branch_id: i64,
    pub name: String,
    pub scan_type: String,
    pub is_active: bool,
    pub last_scan_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Alias for backward compatibility
pub type ScanResponse = DiscoveryScanResponse;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DiscoveryResultResponse {
    pub id: i64,
    pub scan_id: i64,
    pub discovered_ip: String,
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub firmware_version: Option<String>,
    pub status: String,
    pub discovered_at: DateTime<Utc>,
}

/// Alias for backward compatibility
pub type ResultResponse = DiscoveryResultResponse;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DashboardResponse {
    pub pending: i64,
    pub approved: i64,
    pub rejected: i64,
    pub recent_24h: i64,
    pub by_vendor: Vec<VendorCount>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct VendorCount {
    pub vendor: String,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}
