use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "traffic_aggregates")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub customer_id: Option<i64>,
    pub subscription_id: Option<i64>,
    pub branch_id: Option<i64>,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub period: String,
    pub total_bytes_in: i64,
    pub total_bytes_out: i64,
    pub peak_bytes_in: i64,
    pub peak_bytes_out: i64,
    pub sample_count: i64,
    pub period_start: DateTimeWithTimeZone,
    pub period_end: DateTimeWithTimeZone,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
