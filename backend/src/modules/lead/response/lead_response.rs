use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[derive(ToSchema)]
pub struct LeadResponse {
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
    #[sqlx(default)]
    pub assigned_to_name: Option<String>,
    #[sqlx(default)]
    pub branch_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(ToSchema)]
pub struct LeadListResponse {
    pub leads: Vec<LeadResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[derive(ToSchema)]
pub struct LeadActivityResponse {
    pub id: i64,
    pub lead_id: i64,
    pub activity_type: String,
    pub description: String,
    pub performed_by: i64,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    #[sqlx(default)]
    pub performer_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(ToSchema)]
pub struct LeadPipelineResponse {
    pub new: i64,
    pub contacted: i64,
    pub interested: i64,
    pub surveyed: i64,
    pub quoted: i64,
    pub converted: i64,
    pub lost: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(ToSchema)]
pub struct LeadStatsResponse {
    pub total_leads: i64,
    pub converted_this_month: i64,
    pub conversion_rate: f64,
    pub by_source: Vec<SourceCount>,
    pub by_status: Vec<StatusCount>,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(ToSchema)]
pub struct SourceCount {
    pub source: String,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(ToSchema)]
pub struct StatusCount {
    pub status: String,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(ToSchema)]
pub struct MessageResponse {
    pub message: String,
}
