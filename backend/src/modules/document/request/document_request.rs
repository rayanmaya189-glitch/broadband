use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UploadRequest { pub filename: String, pub mime_type: String, pub file_size: i64, pub bucket: String, pub entity_type: Option<String>, pub entity_id: Option<i64> }
#[derive(Debug, Deserialize)]
pub struct DocumentQuery { pub entity_type: Option<String>, pub entity_id: Option<i64>, pub page: Option<i64>, pub per_page: Option<i64> }
