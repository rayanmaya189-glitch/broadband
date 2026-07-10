use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "event_subscriptions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub subscriber_name: String,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub event_type: String,
    pub last_processed_id: Option<i64>,
    pub last_processed_at: Option<DateTimeWithTimeZone>,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
