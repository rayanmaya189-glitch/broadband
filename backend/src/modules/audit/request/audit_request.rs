use utoipa::ToSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, ToSchema)]
pub struct AuditQuery {
    pub user_id: Option<i64>,
    pub action: Option<String>,
    pub resource_type: Option<String>,
    pub result: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ExportAuditRequest {
    pub user_id: Option<i64>,
    pub action: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
}
