use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "billing", table_name = "invoices")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub invoice_number: String,
    #[sea_orm(index)]
    pub customer_id: i64,
    #[sea_orm(index)]
    pub branch_id: i64,
    #[sea_orm(index)]
    pub subscription_id: i64,
    pub billing_period_start: chrono::NaiveDate,
    pub billing_period_end: chrono::NaiveDate,
    pub subtotal: sea_orm::prelude::Decimal,
    pub discount_amount: sea_orm::prelude::Decimal,
    pub tax_amount: sea_orm::prelude::Decimal,
    pub total_amount: sea_orm::prelude::Decimal,
    pub currency: String,
    pub status: String,
    pub due_date: chrono::NaiveDate,
    pub paid_at: Option<chrono::DateTime<chrono::Utc>>,
    pub payment_method: Option<String>,
    pub payment_reference: Option<String>,
    pub created_by: Option<i64>,
    pub reviewed_by: Option<i64>,
    pub reviewed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub review_notes: Option<String>,
    pub approved_by: Option<i64>,
    pub approved_at: Option<chrono::DateTime<chrono::Utc>>,
    pub notes: Option<String>,
    pub review_status: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub cgst_amount: sea_orm::prelude::Decimal,
    pub sgst_amount: sea_orm::prelude::Decimal,
    pub igst_amount: sea_orm::prelude::Decimal,
    pub place_of_supply_state: String,
    pub supplier_gstin: Option<String>,
    pub reverse_charge: bool,
    pub late_fee_subtotal: sea_orm::prelude::Decimal,
    pub late_fee_gst: sea_orm::prelude::Decimal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
