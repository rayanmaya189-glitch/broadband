use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "notification", table_name = "notification_delivery_history")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub notification_id: i64,
    pub channel: String,
    pub status: String,
    pub attempts: i32,
    pub last_error: Option<String>,
    pub sent_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
