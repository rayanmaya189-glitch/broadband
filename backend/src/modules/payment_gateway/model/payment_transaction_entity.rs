use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "payment_transactions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub gateway_id: String,
    pub invoice_id: Option<i64>,
    pub customer_id: Option<i64>,
    pub amount: sea_orm::prelude::Decimal,
    #[sea_orm(column_type = "String(StringLen::N(10))")]
    pub currency: String,
    #[sea_orm(column_type = "String(StringLen::N(30))")]
    pub payment_method: String,
    pub gateway_transaction_id: Option<String>,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub status: String,
    pub idempotency_key: Option<String>,
    pub failure_reason: Option<String>,
    pub webhook_received_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
