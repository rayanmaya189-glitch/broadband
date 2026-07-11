use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Debug, Serialize, Deserialize, ToSchema)]
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct IpAddressResponse {
    pub id: i64,
    pub ip_pool_id: i64,
    pub ip_address: String,
    pub status: String,
    pub allocated_to_type: Option<String>,
    pub allocated_to_id: Option<i64>,
    pub allocated_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PppoeSessionResponse {
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MacBindingResponse {
    pub id: i64,
    pub branch_id: i64,
    pub customer_id: i64,
    pub subscription_id: i64,
    pub mac_address: String,
    pub assigned_ip: String,
    pub vlan_id: Option<i64>,
    pub is_active: bool,
    pub bound_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DhcpLeaseResponse {
    pub id: i64,
    pub mac_address: String,
    pub ip_address: String,
    pub hostname: Option<String>,
    pub lease_type: String,
    pub status: String,
    pub lease_start: DateTime<Utc>,
    pub lease_end: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CustomerSessionResponse {
    pub id: i64,
    pub customer_id: i64,
    pub mac_address: String,
    pub ip_address: String,
    pub connected_at: Option<DateTime<Utc>>,
    pub last_activity_at: Option<DateTime<Utc>>,
    pub bytes_in: i64,
    pub bytes_out: i64,
    pub is_online: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NetworkTopologyResponse {
    pub total_vlans: i64,
    pub total_ip_pools: i64,
    pub total_active_sessions: i64,
    pub total_mac_bindings: i64,
    pub active_pppoe_sessions: i64,
    pub active_dhcp_leases: i64,
    pub online_customers: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PaginatedResponse<T: Serialize> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}
