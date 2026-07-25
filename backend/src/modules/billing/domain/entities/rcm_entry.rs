use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "billing", table_name = "rcm_entries")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(index)]
    pub invoice_id: Option<i64>,
    pub vendor_name: String,
    pub vendor_gstin: Option<String>,
    pub service_description: String,
    pub taxable_value: sea_orm::prelude::Decimal,
    pub cgst_amount: sea_orm::prelude::Decimal,
    pub sgst_amount: sea_orm::prelude::Decimal,
    pub igst_amount: sea_orm::prelude::Decimal,
    pub total_gst: sea_orm::prelude::Decimal,
    pub itc_claimed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
