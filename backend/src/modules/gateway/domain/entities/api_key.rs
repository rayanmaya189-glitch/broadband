use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// API keys for external integrations and partner access.
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "gateway", table_name = "api_keys")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// Human-readable key name (e.g. "MikroTik Integration")
    pub name: String,
    /// The hashed API key value
    pub key_hash: String,
    /// Prefix of the key for identification (e.g. "ax_live_")
    pub key_prefix: String,
    /// Associated branch (optional — platform-level keys have no branch)
    pub branch_id: Option<i64>,
    /// Comma-separated permissions (e.g. "device.read,network.write")
    pub permissions: String,
    /// Optional expiry datetime
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub is_active: bool,
    pub last_used_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
