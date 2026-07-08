use chrono::NaiveDate;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateInstallationRequest {
    pub customer_id: i64,
    pub branch_id: i64,
    pub subscription_id: Option<i64>,
    pub installation_type: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ScheduleInstallationRequest {
    pub scheduled_date: NaiveDate,
    pub scheduled_time_slot: String,
    pub technician_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CompleteInstallationRequest {
    pub fiber_drop_length_meters: Option<i32>,
    pub onu_power_dbm: Option<f64>,
    pub equipment_issued: Option<serde_json::Value>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InstallationQuery {
    pub status: Option<String>,
    pub branch_id: Option<i64>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
