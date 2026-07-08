use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct GatewayConfig {
    pub id: i64,
    pub gateway_id: String,
    pub name: String,
    pub is_primary: bool,
    pub is_active: bool,
    pub supported_methods: Vec<String>,
    pub currency: String,
    pub created_at: DateTime<Utc>,
}
