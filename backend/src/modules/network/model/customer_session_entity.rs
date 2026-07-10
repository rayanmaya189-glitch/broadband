use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "customer_sessions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub branch_id: i64,
    pub customer_id: i64,
    pub subscription_id: i64,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub mac_address: String,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub ip_address: String,
    pub connected_at: Option<DateTimeWithTimeZone>,
    pub disconnected_at: Option<DateTimeWithTimeZone>,
    pub last_activity_at: Option<DateTimeWithTimeZone>,
    pub bytes_in: i64,
    pub bytes_out: i64,
    pub is_online: bool,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
