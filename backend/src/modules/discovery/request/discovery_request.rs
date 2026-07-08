use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateScanRequest {
    pub branch_id: i64,
    pub name: String,
    pub scan_type: String,
}

#[derive(Debug, Deserialize)]
pub struct DiscoveryQuery {
    pub branch_id: Option<i64>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
