use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
    pub role_id: i64,
    pub branch_id: Option<i64>,
    pub is_company_wide: bool,
    pub is_active: bool,
    pub is_locked: bool,
    pub locked_until: Option<DateTimeWithTimeZone>,
    pub failed_attempts: i32,
    pub last_login_at: Option<DateTimeWithTimeZone>,
    pub two_factor_enabled: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::refresh_token_entity::Entity")]
    RefreshToken,
}

impl Related<super::refresh_token_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RefreshToken.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
