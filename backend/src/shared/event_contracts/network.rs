use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VlanCreatedV1 {
    pub vlan_id: i64,
    pub branch_id: i64,
    pub vlan_tag: i32,
    pub vlan_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VlanDeletedV1 {
    pub vlan_id: i64,
    pub branch_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PppoeSessionStartedV1 {
    pub session_id: i64,
    pub customer_id: i64,
    pub assigned_ip: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PppoeSessionEndedV1 {
    pub session_id: i64,
    pub customer_id: i64,
    pub duration_seconds: i64,
    pub bytes_in: i64,
    pub bytes_out: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerSessionConnectedV1 {
    pub session_id: i64,
    pub customer_id: i64,
    pub ip_address: String,
    pub mac_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerSessionDisconnectedV1 {
    pub session_id: i64,
    pub customer_id: i64,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpPoolExhaustedV1 {
    pub pool_id: i64,
    pub branch_id: i64,
    pub utilization_percent: f64,
}
