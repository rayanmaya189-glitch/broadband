use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub email: String,
    #[sea_orm(unique)]
    pub phone: String,
    pub password_hash: Option<String>,
    pub name: String,
    pub avatar_url: Option<String>,
    pub branch_id: Option<i64>,
    pub status: String,
    pub last_login_at: Option<chrono::DateTime<chrono::Utc>>,
    pub failed_login_attempts: i32,
    pub locked_until: Option<chrono::DateTime<chrono::Utc>>,
    pub two_factor_enabled: bool,
    pub two_factor_secret: Option<String>,
    pub phone_verified: bool,
    pub email_verified: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
