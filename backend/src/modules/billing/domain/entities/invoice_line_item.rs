use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "billing", table_name = "invoice_line_items")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub invoice_id: i64,
    pub description: String,
    pub quantity: sea_orm::prelude::Decimal,
    pub unit_price: sea_orm::prelude::Decimal,
    pub amount: sea_orm::prelude::Decimal,
    pub tax_rate: sea_orm::prelude::Decimal,
    pub tax_amount: sea_orm::prelude::Decimal,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub hsn_sac_code: Option<String>,
    pub tax_type: String,
    pub cgst_rate: sea_orm::prelude::Decimal,
    pub sgst_rate: sea_orm::prelude::Decimal,
    pub igst_rate: sea_orm::prelude::Decimal,
    pub cgst_amount: sea_orm::prelude::Decimal,
    pub sgst_amount: sea_orm::prelude::Decimal,
    pub igst_amount: sea_orm::prelude::Decimal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
