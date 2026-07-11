use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct IpAddress {
    pub id: i64,
    pub ip_pool_id: i64,
    pub ip_address: String,
    pub status: String,
    pub allocated_to_type: Option<String>,
    pub allocated_to_id: Option<i64>,
    pub allocated_at: Option<DateTime<Utc>>,
    pub released_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct MacBinding {
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

#[derive(Debug, Clone)]
pub struct DhcpLease {
    pub id: i64,
    pub branch_id: i64,
    pub mac_address: String,
    pub ip_address: String,
    pub hostname: Option<String>,
    pub vlan_id: Option<i64>,
    pub ip_pool_id: i64,
    pub lease_start: DateTime<Utc>,
    pub lease_end: DateTime<Utc>,
    pub lease_type: String,
    pub customer_id: Option<i64>,
    pub subscription_id: Option<i64>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CustomerSession {
    pub id: i64,
    pub branch_id: i64,
    pub customer_id: i64,
    pub subscription_id: i64,
    pub mac_address: String,
    pub ip_address: String,
    pub connected_at: Option<DateTime<Utc>>,
    pub disconnected_at: Option<DateTime<Utc>>,
    pub last_activity_at: Option<DateTime<Utc>>,
    pub bytes_in: i64,
    pub bytes_out: i64,
    pub is_online: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct NetworkTopology {
    pub total_vlans: i64,
    pub total_ip_pools: i64,
    pub total_active_sessions: i64,
    pub total_mac_bindings: i64,
    pub active_pppoe_sessions: i64,
    pub active_dhcp_leases: i64,
    pub online_customers: i64,
}
