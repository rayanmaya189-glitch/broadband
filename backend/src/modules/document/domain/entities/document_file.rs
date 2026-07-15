use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "document_files")]
pub struct Model {
    #[sea_orm(primary_key)]
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
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
