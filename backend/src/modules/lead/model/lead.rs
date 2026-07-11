use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug, Clone)]
pub struct Lead {
    pub id: i64,
    pub branch_id: i64,
    pub assigned_to: Option<i64>,
    pub name: String,
    pub phone: String,
    pub email: Option<String>,
    pub source: String,
    pub status: String,
    pub interested_plan_id: Option<i64>,
    pub estimated_install_date: Option<NaiveDate>,
    pub address: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub lost_reason: Option<String>,
    pub notes: Option<String>,
    pub converted_customer_id: Option<i64>,
    pub converted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct LeadActivity {
    pub id: i64,
    pub lead_id: i64,
    pub activity_type: String,
    pub description: String,
    pub performed_by: i64,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
