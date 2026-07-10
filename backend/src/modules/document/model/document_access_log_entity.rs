use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "document_access_logs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub document_id: i64,
    pub accessed_by: Option<i64>,
    #[sea_orm(column_type = "String(StringLen::N(30))")]
    pub access_type: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub accessed_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
