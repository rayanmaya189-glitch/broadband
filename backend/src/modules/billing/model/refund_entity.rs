use sea_orm::entity::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "refunds")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub refund_number: String,
    pub payment_id: i64,
    pub invoice_id: i64,
    pub customer_id: i64,
    pub amount: Decimal,
    pub reason: String,
    pub requested_by: Option<i64>,
    pub approved_by: Option<i64>,
    pub status: String,
    pub processed_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::payment_entity::Entity",
        from = "Column::PaymentId",
        to = "super::payment_entity::Column::Id"
    )]
    Payment,
}

impl Related<super::payment_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Payment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
