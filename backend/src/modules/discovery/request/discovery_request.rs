use utoipa::ToSchema;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateScanRequest {
    pub branch_id: i64,
    pub name: String,
    pub scan_type: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DiscoveryQuery {
    pub branch_id: Option<i64>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RejectRequest {
    #[validate(length(min = 1))]
    pub reason: String,
}

// Type aliases for backward compatibility
pub type CreateDiscoveryScanRequest = CreateScanRequest;
pub type RejectDiscoveryRequest = RejectRequest;

#[derive(Debug, Deserialize, ToSchema)]
pub struct DiscoveryResultQuery {
    pub status: Option<String>,
    pub branch_id: Option<i64>,
}
