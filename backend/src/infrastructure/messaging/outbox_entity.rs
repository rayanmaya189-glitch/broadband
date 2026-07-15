use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "outbox_events")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub event_id: String,
    pub event_type: String,
    pub aggregate_type: String,
    pub aggregate_id: i64,
    pub payload: JsonValue,
    pub metadata: Option<JsonValue>,
    pub caused_by_user_id: Option<i64>,
    pub caused_by_branch_id: Option<i64>,
    pub published: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
