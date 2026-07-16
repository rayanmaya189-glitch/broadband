use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "customer", table_name = "")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub customer_id: i64,
    pub aadhaar_hash: Option<String>,
    pub pan_hash: Option<String>,
    pub gender: Option<String>,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub occupation: Option<String>,
    pub kyc_status: String,
    pub kyc_verified_at: Option<chrono::DateTime<chrono::Utc>>,
    pub kyc_verified_by: Option<i64>,
    pub kyc_rejection_reason: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
