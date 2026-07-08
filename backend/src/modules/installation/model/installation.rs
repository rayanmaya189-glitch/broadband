use chrono::{DateTime, NaiveDate, Utc};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct InstallationOrder {
    pub id: i64,
    pub customer_id: i64,
    pub branch_id: i64,
    pub subscription_id: Option<i64>,
    pub assigned_technician_id: Option<i64>,
    pub status: String,
    pub scheduled_date: Option<NaiveDate>,
    pub scheduled_time_slot: Option<String>,
    pub completed_at: Option<DateTime<Utc>>,
    pub installation_type: String,
    pub equipment_issued: Option<Value>,
    pub fiber_drop_length_meters: Option<i32>,
    pub onu_power_dbm: Option<f64>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
