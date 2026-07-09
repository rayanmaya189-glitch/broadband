use utoipa::ToSchema;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateVlanRequest {
    pub branch_id: i64,
    pub vlan_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub vlan_type: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateVlanRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub vlan_type: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateIpPoolRequest {
    pub branch_id: i64,
    pub name: String,
    pub cidr: String,
    pub gateway: String,
    pub dns_primary: Option<String>,
    pub dns_secondary: Option<String>,
    pub vlan_id: Option<i64>,
    pub pool_type: Option<String>,
    pub total_count: i32,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreatePppoeSessionRequest {
    pub branch_id: i64,
    pub customer_id: i64,
    pub subscription_id: i64,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AllocateIpRequest {
    pub pool_id: i64,
    pub allocated_to_type: String,
    pub allocated_to_id: i64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateMacBindingRequest {
    pub branch_id: i64,
    pub customer_id: i64,
    pub subscription_id: i64,
    pub mac_address: String,
    pub assigned_ip: String,
    pub vlan_id: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct NetworkQuery {
    pub branch_id: Option<i64>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
    pub is_online: Option<bool>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct IpPoolQuery {
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ReleaseIpRequest {
    pub pool_id: i64,
    pub ip_id: i64,
}
