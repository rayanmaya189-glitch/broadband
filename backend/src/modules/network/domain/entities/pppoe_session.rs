use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "pppoe_sessions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub branch_id: i64,
    pub customer_id: i64,
    pub subscription_id: i64,
    pub username: String,
    pub password_encrypted: String,
    pub pppoe_server_ip: Option<String>,
    pub assigned_ip: Option<String>,
    pub nas_port_id: Option<String>,
    pub nas_ip_address: Option<String>,
    pub nas_session_id: Option<String>,
    pub session_start: Option<chrono::DateTime<chrono::Utc>>,
    pub session_duration_seconds: i64,
    pub bytes_in: i64,
    pub bytes_out: i64,
    pub status: String,
    pub device_id: Option<i64>,
    pub last_activity_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
