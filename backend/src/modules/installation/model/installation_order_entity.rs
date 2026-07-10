use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "installation_orders")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub customer_id: i64,
    pub branch_id: i64,
    pub subscription_id: Option<i64>,
    pub assigned_technician_id: Option<i64>,
    #[sea_orm(column_type = "String(StringLen::N(30))")]
    pub status: String,
    pub scheduled_date: Option<Date>,
    pub scheduled_time_slot: Option<String>,
    pub completed_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(column_type = "String(StringLen::N(30))")]
    pub installation_type: String,
    pub equipment_issued: Option<sea_orm::prelude::Json>,
    pub fiber_drop_length_meters: Option<i32>,
    pub onu_power_dbm: Option<f64>,
    pub notes: Option<String>,
    pub photos: Option<sea_orm::prelude::Json>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
