use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "installation", table_name = "")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub customer_id: i64,
    pub branch_id: i64,
    pub subscription_id: Option<i64>,
    pub assigned_technician_id: Option<i64>,
    pub status: String,
    pub scheduled_date: Option<chrono::NaiveDate>,
    pub scheduled_time_slot: Option<String>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub installation_type: Option<String>,
    pub equipment_issued: Option<serde_json::Value>,
    pub fiber_drop_length_meters: Option<i32>,
    pub onu_power_dbm: Option<sea_orm::prelude::Decimal>,
    pub notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
