use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Data retention policies defining how long different data types are kept.
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "data_retention_policies")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// Entity type this policy applies to (e.g. "customers", "invoices", "audit_logs")
    pub entity_type: String,
    /// Retention period in days (-1 = retain forever)
    pub retention_days: i32,
    /// Action when retention expires: delete, archive, anonymize
    pub action: String,
    /// Whether this policy is currently active
    pub is_active: bool,
    /// Description of the policy
    pub description: Option<String>,
    /// Legal basis for retention (e.g. "RBI regulation", "IT Act 2000")
    pub legal_basis: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
