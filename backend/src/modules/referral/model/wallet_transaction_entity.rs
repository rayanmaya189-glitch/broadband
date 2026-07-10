use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "wallet_transactions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub wallet_id: i64,
    #[sea_orm(column_type = "String(StringLen::N(30))")]
    pub transaction_type: String,
    pub amount: sea_orm::prelude::Decimal,
    pub balance_after: sea_orm::prelude::Decimal,
    pub reference_type: Option<String>,
    pub reference_id: Option<i64>,
    pub description: Option<String>,
    pub performed_by: Option<i64>,
    pub expires_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
