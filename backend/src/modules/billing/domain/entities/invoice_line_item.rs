use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "invoice_line_items")]
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
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
