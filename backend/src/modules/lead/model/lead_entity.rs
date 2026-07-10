use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "leads")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub branch_id: i64,
    pub assigned_to: Option<i64>,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub name: String,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub phone: String,
    pub email: Option<String>,
    #[sea_orm(column_type = "String(StringLen::N(30))")]
    pub source: String,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub status: String,
    pub interested_plan_id: Option<i64>,
    pub estimated_install_date: Option<Date>,
    pub address: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub lost_reason: Option<String>,
    pub notes: Option<String>,
    pub converted_customer_id: Option<i64>,
    pub converted_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
