use utoipa::ToSchema;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UploadRequest {
    pub filename: String,
    pub mime_type: String,
    pub file_size: i64,
    pub bucket: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ConfirmUploadRequest {
    pub file_hash: Option<String>,
    pub storage_url: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AssociateEntityRequest {
    pub entity_type: String,
    pub entity_id: i64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DocumentQuery {
    pub entity_type: Option<String>,
    pub entity_id: Option<i64>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
