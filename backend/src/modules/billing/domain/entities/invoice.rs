use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "billing", table_name = "")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub invoice_number: String,
    pub customer_id: i64,
    pub branch_id: i64,
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
    pub review_status: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
