use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "payment_gateways")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    #[sea_orm(column_type = "String(StringLen::N(50))", unique)]
    pub gateway_id: String,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub name: String,
    pub is_primary: bool,
    pub is_active: bool,
    pub supported_methods: sea_orm::prelude::Json,
    #[sea_orm(column_type = "String(StringLen::N(10))")]
    pub currency: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
