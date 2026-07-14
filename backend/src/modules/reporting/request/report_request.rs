use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct GenerateReportRequest {
    pub report_type: String,
    pub name: String,
    pub parameters: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateScheduleRequest {
    pub report_type: String,
    pub name: String,
    pub parameters: Option<serde_json::Value>,
    pub frequency: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ReportQuery {
    pub branch_id: Option<i64>,
    pub report_type: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
