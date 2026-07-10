use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "entity_history")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub entity_type: String,
    pub entity_id: i64,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub action: String,
    pub old_data: Option<sea_orm::prelude::Json>,
    pub new_data: Option<sea_orm::prelude::Json>,
    pub changed_fields: Option<sea_orm::prelude::Json>,
    pub user_id: Option<i64>,
    pub branch_id: Option<i64>,
    pub ip_address: Option<String>,
    pub reason: Option<String>,
    pub rollback_reference: Option<i64>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
