use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "subscriptions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub customer_id: i64,
    pub branch_id: i64,
    pub plan_id: i64,
    pub status: String,
    pub billing_period_months: i32,
    pub start_date: chrono::NaiveDate,
    pub end_date: Option<chrono::NaiveDate>,
    pub next_billing_date: Option<chrono::NaiveDate>,
    pub auto_renew: bool,
    pub pppoe_session_id: Option<i64>,
    pub mac_address: Option<String>,
    pub ip_address: Option<String>,
    pub vlan_id: Option<i32>,
    pub review_status: Option<String>,
    pub approved_by: Option<i64>,
    pub approved_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
