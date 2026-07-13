use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

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

/// Request body for rolling back an entity to a previous state.
/// The `history_id` comes from the URL path, not the body.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RollbackRequest {
    #[validate(range(min = 1, message = "User ID must be a positive integer"))]
    pub user_id: i64,
    #[validate(length(min = 1, message = "Reason is required for rollback"))]
    pub reason: String,
}
