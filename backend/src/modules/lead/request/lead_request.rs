use chrono::NaiveDate;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateLeadRequest {
    pub branch_id: i64,
    pub name: String,
    pub phone: String,
    pub email: Option<String>,
    pub source: String,
    pub interested_plan_id: Option<i64>,
    pub estimated_install_date: Option<NaiveDate>,
    pub address: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateLeadRequest {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub source: Option<String>,
    pub interested_plan_id: Option<i64>,
    pub estimated_install_date: Option<NaiveDate>,
    pub address: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LeadStatusRequest {
    #[validate(length(min = 1, max = 30))]
    pub status: String,
    pub lost_reason: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AssignLeadRequest {
    pub assigned_to: i64,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddActivityRequest {
    #[validate(length(min = 1, max = 30))]
    pub activity_type: String,
    #[validate(length(min = 1))]
    pub description: String,
    pub scheduled_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ConvertLeadRequest {
    pub plan_id: Option<i64>,
    pub branch_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct LeadQuery {
    pub status: Option<String>,
    pub source: Option<String>,
    pub branch_id: Option<i64>,
    pub assigned_to: Option<i64>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
