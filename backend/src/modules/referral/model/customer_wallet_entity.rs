use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "customer_wallets")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub customer_id: i64,
    pub balance: sea_orm::prelude::Decimal,
    pub total_earned: sea_orm::prelude::Decimal,
    pub total_spent: sea_orm::prelude::Decimal,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub status: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
