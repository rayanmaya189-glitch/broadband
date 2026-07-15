use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerCreatedV1 {
    pub customer_id: i64,
    pub customer_code: String,
    pub name: String,
    pub phone: String,
    pub email: Option<String>,
    pub branch_id: i64,
    pub referred_by: Option<i64>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerActivatedV1 {
    pub customer_id: i64,
    pub subscription_id: i64,
    pub plan_id: i64,
    pub branch_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerSuspendedV1 {
    pub customer_id: i64,
    pub subscription_id: Option<i64>,
    pub reason: String,
    pub suspended_by: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerReactivatedV1 {
    pub customer_id: i64,
    pub subscription_id: Option<i64>,
    pub reactivated_by: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerTerminatedV1 {
    pub customer_id: i64,
    pub reason: String,
    pub terminated_by: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerKycSubmittedV1 {
    pub customer_id: i64,
    pub document_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerKycVerifiedV1 {
    pub customer_id: i64,
    pub verified_by: i64,
}
