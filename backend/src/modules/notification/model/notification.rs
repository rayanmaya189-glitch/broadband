use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct NotificationTemplate {
    pub id: i64,
    pub name: String,
    pub channel: String,
    pub subject_template: Option<String>,
    pub body_template: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct Notification {
    pub id: i64,
    pub channel: String,
    pub recipient_type: String,
    pub recipient_id: i64,
    pub recipient_address: String,
    pub subject: Option<String>,
    pub body: String,
    pub status: String,
    pub retry_count: i32,
    pub created_at: DateTime<Utc>,
}
