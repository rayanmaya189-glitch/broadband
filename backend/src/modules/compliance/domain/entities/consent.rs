use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Customer consent records for GDPR/data privacy compliance.
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "compliance", table_name = "")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub customer_id: i64,
    /// Consent type: marketing, data_sharing, analytics, third_party, sms_broadcast
    pub consent_type: String,
    /// Whether consent was given or revoked
    pub granted: bool,
    /// Channel through which consent was collected: web, mobile, email, call_center
    pub collection_channel: String,
    /// IP address of the consenting party
    pub ip_address: Option<String>,
    /// User agent of the consenting party
    pub user_agent: Option<String>,
    /// When consent expires (None = valid indefinitely until revoked)
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// When consent was revoked (if applicable)
    pub revoked_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
