use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeadCreatedV1 {
    pub lead_id: i64,
    pub name: String,
    pub phone: String,
    pub source: String,
    pub branch_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeadStatusChangedV1 {
    pub lead_id: i64,
    pub old_status: String,
    pub new_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeadConvertedV1 {
    pub lead_id: i64,
    pub customer_id: i64,
    pub plan_id: Option<i64>,
}
