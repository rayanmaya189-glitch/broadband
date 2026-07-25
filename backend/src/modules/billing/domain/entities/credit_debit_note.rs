use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "billing", table_name = "credit_debit_notes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub note_number: String,
    pub note_type: String,
    #[sea_orm(index)]
    pub original_invoice_id: i64,
    #[sea_orm(index)]
    pub customer_id: i64,
    #[sea_orm(index)]
    pub branch_id: i64,
    pub reason: String,
    pub subtotal: sea_orm::prelude::Decimal,
    pub cgst_amount: sea_orm::prelude::Decimal,
    pub sgst_amount: sea_orm::prelude::Decimal,
    pub igst_amount: sea_orm::prelude::Decimal,
    pub total_amount: sea_orm::prelude::Decimal,
    pub status: String,
    pub approved_by: Option<i64>,
    pub approved_at: Option<chrono::DateTime<chrono::Utc>>,
    pub applied_to_invoice_id: Option<i64>,
    pub created_by: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
