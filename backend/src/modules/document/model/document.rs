use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct DocumentFile {
    pub id: i64,
    pub filename: String,
    pub original_filename: String,
    pub mime_type: String,
    pub file_size: i64,
    pub storage_bucket: String,
    pub storage_key: String,
    pub uploaded_by: i64,
    pub entity_type: Option<String>,
    pub entity_id: Option<i64>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}
