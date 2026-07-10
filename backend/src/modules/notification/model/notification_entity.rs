use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "notifications")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub customer_id: Option<i64>,
    pub branch_id: Option<i64>,
    #[sea_orm(column_type = "String(StringLen::N(30))")]
    pub r#type: String,
    #[sea_orm(column_type = "String(StringLen::N(30))")]
    pub channel: String,
    pub title: Option<String>,
    pub body: Option<String>,
    pub metadata: Option<sea_orm::prelude::Json>,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub status: String,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
