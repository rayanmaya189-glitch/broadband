use sea_orm::entity::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "invoice_line_items")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub invoice_id: i64,
    pub description: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub amount: Decimal,
    pub tax_rate: Decimal,
    pub tax_amount: Decimal,
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
}

impl Related<super::invoice_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Invoice.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
