use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[derive(ToSchema)]
pub struct TemplateResponse { pub id: i64, pub name: String, pub channel: String, pub is_active: bool, pub created_at: DateTime<Utc> }
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[derive(ToSchema)]
pub struct NotificationResponse { pub id: i64, pub channel: String, pub recipient_address: String, pub status: String, pub created_at: DateTime<Utc> }
#[derive(Debug, Serialize, Deserialize)]
#[derive(ToSchema)]
pub struct MessageResponse { pub message: String }
