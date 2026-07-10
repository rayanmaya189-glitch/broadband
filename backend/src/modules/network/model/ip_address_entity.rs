use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "ip_addresses")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub ip_pool_id: i64,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub ip_address: String,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub status: String,
    pub allocated_to_type: Option<String>,
    pub allocated_to_id: Option<i64>,
    pub allocated_at: Option<DateTimeWithTimeZone>,
    pub released_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
