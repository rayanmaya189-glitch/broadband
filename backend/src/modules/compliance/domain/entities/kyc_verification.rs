use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// KYC verification records for customer identity verification.
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "compliance", table_name = "")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub customer_id: i64,
    /// Document type (aadhaar, pan, passport, voter_id, driving_license)
    pub document_type: String,
    /// Document number (hashed for storage)
    pub document_number_hash: String,
    /// Verification status: pending, submitted, under_review, verified, rejected
    pub status: String,
    /// KYC provider used (digio, nsdl, manual)
    pub provider: Option<String>,
    /// Provider reference ID for tracking
    pub provider_reference: Option<String>,
    /// Rejection reason if rejected
    pub rejection_reason: Option<String>,
    /// Verified at datetime
    pub verified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Expiry of the verification (re-verification needed after)
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
