use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TemplateResponse { pub id: i64, pub name: String, pub channel: String, pub is_active: bool, pub created_at: DateTime<Utc> }
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NotificationResponse { pub id: i64, pub channel: String, pub recipient_address: String, pub status: String, pub created_at: DateTime<Utc> }
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse { pub message: String }
