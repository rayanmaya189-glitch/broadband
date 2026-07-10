use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "ticket_escalations")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub ticket_id: i64,
    pub from_user_id: i64,
    pub to_user_id: i64,
    pub from_priority: Option<String>,
    pub to_priority: Option<String>,
    pub reason: String,
    pub escalated_at: DateTimeWithTimeZone,
    pub acknowledged_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
