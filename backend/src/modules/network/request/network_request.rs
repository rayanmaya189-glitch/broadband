use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateVlanRequest {
    pub branch_id: i64,
    pub vlan_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub vlan_type: String,
}

#[derive(Debug, Deserialize, Validate)]
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

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePppoeSessionRequest {
    pub customer_id: i64,
    pub subscription_id: i64,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct NetworkQuery {
    pub branch_id: Option<i64>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
