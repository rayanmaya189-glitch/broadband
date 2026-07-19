/// OpenAPI schemas and stub handlers for Network endpoints.
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

// ── Request / Response types ─────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct VlanResponse {
    pub id: i64,
    pub branch_id: i64,
    pub vlan_id: i32,
    pub name: String,
    pub vlan_type: String,
    pub is_active: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateVlanRequest {
    pub branch_id: i64,
    pub vlan_id: i32,
    pub name: String,
    pub vlan_type: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct IpPoolResponse {
    pub id: i64,
    pub name: String,
    pub cidr: String,
    pub gateway: String,
    pub allocated_count: i32,
    pub total_count: i32,
    pub status: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateIpPoolRequest {
    pub branch_id: i64,
    pub name: String,
    pub cidr: String,
    pub gateway: String,
    #[serde(default)]
    pub vlan_id: Option<i64>,
    pub pool_type: String,
    pub total_count: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PppoeSessionResponse {
    pub id: i64,
    pub username: String,
    pub assigned_ip: Option<String>,
    pub status: String,
    pub customer_id: i64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePppoeRequest {
    pub branch_id: i64,
    pub customer_id: i64,
    pub subscription_id: i64,
    pub username: String,
    pub password_encrypted: String,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct VlanListParams {
    pub branch_id: Option<i64>,
    pub vlan_type: Option<String>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct PppoeSessionListParams {
    pub branch_id: Option<i64>,
    pub customer_id: Option<i64>,
    pub status: Option<String>,
}

// ── Stub handler functions ───────────────────────────────────────────

/// List all VLANs
#[utoipa::path(
    get,
    path = "/api/v1/network/vlans",
    tag = "Network",
    params(VlanListParams),
    responses(
        (status = 200, description = "List of VLANs"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_vlans() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a new VLAN
#[utoipa::path(
    post,
    path = "/api/v1/network/vlans",
    tag = "Network",
    request_body = CreateVlanRequest,
    responses(
        (status = 201, description = "VLAN created", body = VlanResponse),
        (status = 409, description = "VLAN ID already exists")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_vlan() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Delete a VLAN
#[utoipa::path(
    delete,
    path = "/api/v1/network/vlans/{id}",
    tag = "Network",
    params(("id" = i64, Path, description = "VLAN ID")),
    responses(
        (status = 204, description = "VLAN deleted"),
        (status = 404, description = "VLAN not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_vlan() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List all IP pools
#[utoipa::path(
    get,
    path = "/api/v1/network/ip-pools",
    tag = "Network",
    responses(
        (status = 200, description = "List of IP pools"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_ip_pools() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a new IP pool
#[utoipa::path(
    post,
    path = "/api/v1/network/ip-pools",
    tag = "Network",
    request_body = CreateIpPoolRequest,
    responses(
        (status = 201, description = "IP pool created", body = IpPoolResponse),
        (status = 400, description = "Invalid CIDR range")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_ip_pool() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List PPPoE sessions
#[utoipa::path(
    get,
    path = "/api/v1/network/pppoe/sessions",
    tag = "Network",
    params(PppoeSessionListParams),
    responses(
        (status = 200, description = "List of PPPoE sessions"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_pppoe_sessions() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a new PPPoE session
#[utoipa::path(
    post,
    path = "/api/v1/network/pppoe/sessions",
    tag = "Network",
    request_body = CreatePppoeRequest,
    responses(
        (status = 201, description = "PPPoE session created", body = PppoeSessionResponse),
        (status = 400, description = "Invalid request")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_pppoe_session() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Terminate a PPPoE session
#[utoipa::path(
    post,
    path = "/api/v1/network/pppoe/sessions/{id}/terminate",
    tag = "Network",
    params(("id" = i64, Path, description = "Session ID")),
    responses(
        (status = 200, description = "Session terminated"),
        (status = 404, description = "Session not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn terminate_pppoe_session() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
