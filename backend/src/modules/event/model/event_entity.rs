use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "events")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub event_type: String,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub aggregate_type: String,
    pub aggregate_id: i64,
    pub payload: sea_orm::prelude::Json,
    pub metadata: Option<sea_orm::prelude::Json>,
    pub caused_by_user_id: Option<i64>,
    pub caused_by_branch_id: Option<i64>,
    pub sequence_number: i64,
    pub published_at: DateTimeWithTimeZone,
    pub processed: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
