use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "document_files")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub filename: String,
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub original_filename: String,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub mime_type: String,
    pub file_size: i64,
    pub file_hash: Option<String>,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub storage_bucket: String,
    #[sea_orm(column_type = "String(StringLen::N(500))")]
    pub storage_key: String,
    pub storage_url: Option<String>,
    pub uploaded_by: i64,
    pub entity_type: Option<String>,
    pub entity_id: Option<i64>,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub status: String,
    pub metadata: Option<sea_orm::prelude::Json>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
