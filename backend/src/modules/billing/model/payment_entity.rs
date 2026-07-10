use sea_orm::entity::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "payments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub payment_number: String,
    pub invoice_id: i64,
    pub customer_id: i64,
    pub branch_id: i64,
    pub amount: Decimal,
    pub currency: String,
    pub payment_method: String,
    pub payment_gateway: Option<String>,
    pub gateway_transaction_id: Option<String>,
    pub status: String,
    pub processed_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::invoice_entity::Entity",
        from = "Column::InvoiceId",
        to = "super::invoice_entity::Column::Id"
    )]
    Invoice,

    #[sea_orm(has_many = "super::refund_entity::Entity")]
    Refund,
}

impl Related<super::invoice_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Invoice.def()
    }
}

impl Related<super::refund_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Refund.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
