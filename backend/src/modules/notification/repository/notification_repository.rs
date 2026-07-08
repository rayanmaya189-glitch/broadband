use sqlx::PgPool;
use crate::modules::notification::model::notification::{NotificationTemplate, Notification};

pub struct NotificationRepository<'a> { pool: &'a PgPool }
impl<'a> NotificationRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }
    pub async fn list_templates(&self) -> Result<Vec<NotificationTemplate>, sqlx::Error> { sqlx::query_as::<_, NotificationTemplate>("SELECT * FROM notification_templates ORDER BY name").fetch_all(self.pool).await }
    pub async fn create_template(&self, name: &str, channel: &str, subject: Option<&str>, body: &str) -> Result<NotificationTemplate, sqlx::Error> {
        sqlx::query_as::<_, NotificationTemplate>("INSERT INTO notification_templates (name, channel, subject_template, body_template) VALUES ($1,$2,$3,$4) RETURNING *").bind(name).bind(channel).bind(subject).bind(body).fetch_one(self.pool).await
    }
    pub async fn send(&self, channel: &str, recipient_id: i64, address: &str, subject: Option<&str>, body: &str) -> Result<Notification, sqlx::Error> {
        sqlx::query_as::<_, Notification>("INSERT INTO notifications (channel, recipient_type, recipient_id, recipient_address, subject, body, status) VALUES ($1,'user',$2,$3,$4,$5,'queued') RETURNING *").bind(channel).bind(recipient_id).bind(address).bind(subject).bind(body).fetch_one(self.pool).await
    }
}
