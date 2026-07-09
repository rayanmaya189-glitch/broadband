use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct DocumentFile {
    pub id: i64,
    pub filename: String,
    pub original_filename: String,
    pub mime_type: String,
    pub file_size: i64,
    pub file_hash: Option<String>,
    pub storage_bucket: String,
    pub storage_key: String,
    pub storage_url: Option<String>,
    pub uploaded_by: i64,
    pub entity_type: Option<String>,
    pub entity_id: Option<i64>,
    pub status: String,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct DocumentAccessLog {
    pub id: i64,
    pub document_id: i64,
    pub accessed_by: Option<i64>,
    pub access_type: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub accessed_at: DateTime<Utc>,
}
