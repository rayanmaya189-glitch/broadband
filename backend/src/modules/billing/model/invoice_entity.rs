use chrono::NaiveDate;
use sea_orm::entity::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "invoices")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub invoice_number: String,
    pub customer_id: i64,
    pub branch_id: i64,
    pub subscription_id: i64,
    pub billing_period_start: NaiveDate,
    pub billing_period_end: NaiveDate,
    pub subtotal: Decimal,
    pub discount_amount: Decimal,
    pub tax_amount: Decimal,
    pub cgst_amount: Decimal,
    pub sgst_amount: Decimal,
    pub igst_amount: Decimal,
    pub total_amount: Decimal,
    pub currency: String,
    pub status: String,
    pub due_date: NaiveDate,
    pub paid_at: Option<DateTimeWithTimeZone>,
    pub payment_method: Option<String>,
    pub payment_reference: Option<String>,
    pub created_by: Option<i64>,
    pub review_status: Option<String>,
    pub review_notes: Option<String>,
    pub reviewed_by: Option<i64>,
    pub reviewed_at: Option<DateTimeWithTimeZone>,
    pub approved_by: Option<i64>,
    pub approved_at: Option<DateTimeWithTimeZone>,
    pub notes: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::invoice_line_item_entity::Entity")]
    LineItem,

    #[sea_orm(has_many = "super::payment_entity::Entity")]
    Payment,

    #[sea_orm(
        belongs_to = "crate::modules::customer::model::customer_entity::Entity",
        from = "Column::CustomerId",
        to = "crate::modules::customer::model::customer_entity::Column::Id"
    )]
    Customer,
}

impl Related<super::invoice_line_item_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LineItem.def()
    }
}

impl Related<super::payment_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Payment.def()
    }
}

impl Related<crate::modules::customer::model::customer_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
