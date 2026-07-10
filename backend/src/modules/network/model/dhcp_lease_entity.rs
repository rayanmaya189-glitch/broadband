use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "dhcp_leases")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub branch_id: i64,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub mac_address: String,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub ip_address: String,
    pub hostname: Option<String>,
    pub vlan_id: Option<i64>,
    pub ip_pool_id: i64,
    pub lease_start: DateTimeWithTimeZone,
    pub lease_end: DateTimeWithTimeZone,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub lease_type: String,
    pub customer_id: Option<i64>,
    pub subscription_id: Option<i64>,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub status: String,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
