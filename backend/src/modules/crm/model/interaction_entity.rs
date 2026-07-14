use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "crm_interactions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub customer_id: i64,
    pub branch_id: i64,
    pub user_id: i64,
    #[sea_orm(column_type = "String(StringLen::N(30))")]
    pub interaction_type: String,
    #[sea_orm(column_type = "Text")]
    pub subject: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub body: Option<String>,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub channel: String,
    pub duration_seconds: Option<i32>,
    pub sentiment: Option<String>,
    pub follow_up_date: Option<Date>,
    pub follow_up_done: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
