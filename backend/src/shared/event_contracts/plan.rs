use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanCreatedV1 {
    pub plan_id: i64,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanUpdatedV1 {
    pub plan_id: i64,
    pub changes: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanPublishedV1 {
    pub plan_id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanUnpublishedV1 {
    pub plan_id: i64,
    pub name: String,
}
