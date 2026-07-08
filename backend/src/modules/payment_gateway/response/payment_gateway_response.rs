use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[derive(ToSchema)]
pub struct GatewayConfigResponse {
    pub id: i64,
    pub gateway_id: String,
    pub name: String,
    pub is_primary: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(ToSchema)]
pub struct PaymentLinkResponse {
    pub payment_url: String,
    pub expires_in: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(ToSchema)]
pub struct MessageResponse {
    pub message: String,
}
