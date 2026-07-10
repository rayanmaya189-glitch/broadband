use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "pppoe_sessions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub branch_id: i64,
    pub customer_id: i64,
    pub subscription_id: i64,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub username: String,
    pub password_encrypted: Option<String>,
    pub assigned_ip: Option<String>,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub status: String,
    pub session_start: Option<DateTimeWithTimeZone>,
    pub bytes_in: i64,
    pub bytes_out: i64,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
