use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallationScheduledV1 {
    pub installation_id: i64,
    pub customer_id: i64,
    pub technician_id: Option<i64>,
    pub scheduled_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallationCompletedV1 {
    pub installation_id: i64,
    pub customer_id: i64,
    pub equipment_issued: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallationCancelledV1 {
    pub installation_id: i64,
    pub customer_id: i64,
    pub reason: String,
}
