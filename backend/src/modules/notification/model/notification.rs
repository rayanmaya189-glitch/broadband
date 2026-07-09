use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct NotificationTemplate {
    pub id: i64,
    pub name: String,
    pub channel: String,
    pub subject_template: Option<String>,
    pub body_template: String,
    pub variables: Option<Value>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct NotificationChannel {
    pub id: i64,
    pub channel: String,
    pub provider: String,
    pub config: Value,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct Notification {
    pub id: i64,
    pub template_id: Option<i64>,
    pub channel: String,
    pub recipient_type: String,
    pub recipient_id: i64,
    pub recipient_address: String,
    pub subject: Option<String>,
    pub body: String,
    pub variables: Option<Value>,
    pub status: String,
    pub retry_count: i32,
    pub max_retries: i32,
    pub last_error: Option<String>,
    pub sent_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct NotificationHistory {
    pub id: i64,
    pub notification_id: i64,
    pub event: String,
    pub details: Option<Value>,
    pub recorded_at: DateTime<Utc>,
}
