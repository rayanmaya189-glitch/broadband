use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandwidthProfileUpdatedV1 {
    pub profile_id: i64,
    pub changes: serde_json::Value,
    pub affected_subscriptions: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandwidthProfileAppliedV1 {
    pub profile_id: i64,
    pub subscription_id: i64,
    pub device_id: i64,
    pub applied_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandwidthProfileFailedV1 {
    pub profile_id: i64,
    pub subscription_id: i64,
    pub device_id: i64,
    pub error: String,
    pub retry_count: i32,
}
