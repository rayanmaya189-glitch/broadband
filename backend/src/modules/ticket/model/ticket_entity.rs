use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "tickets")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    #[sea_orm(column_type = "String(StringLen::N(30))", unique)]
    pub ticket_number: String,
    pub branch_id: i64,
    pub customer_id: Option<i64>,
    pub subscription_id: Option<i64>,
    pub created_by: i64,
    pub assigned_to: Option<i64>,
    pub escalated_to: Option<i64>,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub category: String,
    pub subcategory: Option<String>,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub priority: String,
    #[sea_orm(column_type = "String(StringLen::N(30))")]
    pub status: String,
    pub subject: String,
    pub description: String,
    #[sea_orm(column_type = "String(StringLen::N(30))")]
    pub source: String,
    pub resolution_notes: Option<String>,
    pub sla_response_at: Option<DateTimeWithTimeZone>,
    pub sla_resolution_at: Option<DateTimeWithTimeZone>,
    pub first_response_at: Option<DateTimeWithTimeZone>,
    pub resolved_at: Option<DateTimeWithTimeZone>,
    pub closed_at: Option<DateTimeWithTimeZone>,
    pub reopen_count: i32,
    pub satisfaction_rating: Option<i32>,
    pub satisfaction_feedback: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
