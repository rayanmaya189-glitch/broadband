use utoipa::ToSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, ToSchema)]
pub struct EntityHistoryQuery {
    pub entity_type: Option<String>,
    pub entity_id: Option<i64>,
    pub action: Option<String>,
    pub user_id: Option<i64>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RollbackRequest {
    pub history_id: i64,
    pub user_id: i64,
    pub reason: String,
}
