use sqlx::PgPool;
use crate::modules::notification::model::notification::{NotificationTemplate, NotificationChannel, Notification, NotificationHistory};

pub struct NotificationRepository<'a> { pool: &'a PgPool }
impl<'a> NotificationRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }
    pub fn pool(&self) -> &'a PgPool { self.pool }

    // ── Templates ──────────────────────────────────────────

    pub async fn list_templates(&self) -> Result<Vec<NotificationTemplate>, sqlx::Error> {
        sqlx::query_as::<_, NotificationTemplate>("SELECT * FROM notification_templates ORDER BY name")
            .fetch_all(self.pool).await
    }

    pub async fn get_template(&self, id: i64) -> Result<Option<NotificationTemplate>, sqlx::Error> {
        sqlx::query_as::<_, NotificationTemplate>("SELECT * FROM notification_templates WHERE id = $1")
            .bind(id).fetch_optional(self.pool).await
    }

    pub async fn create_template(&self, name: &str, channel: &str, subject: Option<&str>, body: &str) -> Result<NotificationTemplate, sqlx::Error> {
        sqlx::query_as::<_, NotificationTemplate>("INSERT INTO notification_templates (name, channel, subject_template, body_template) VALUES ($1,$2,$3,$4) RETURNING *")
            .bind(name).bind(channel).bind(subject).bind(body).fetch_one(self.pool).await
    }

    pub async fn update_template(&self, id: i64, name: Option<&str>, channel: Option<&str>, subject: Option<&str>, body: Option<&str>) -> Result<NotificationTemplate, sqlx::Error> {
        sqlx::query_as::<_, NotificationTemplate>("UPDATE notification_templates SET name = COALESCE($2, name), channel = COALESCE($3, channel), subject_template = $4, body_template = COALESCE($5, body_template), updated_at = NOW() WHERE id = $1 RETURNING *")
            .bind(id).bind(name).bind(channel).bind(subject).bind(body).fetch_one(self.pool).await
    }

    pub async fn delete_template(&self, id: i64) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("DELETE FROM notification_templates WHERE id = $1").bind(id).execute(self.pool).await?;
        Ok(r.rows_affected() > 0)
    }

    // ── Channels ───────────────────────────────────────────

    pub async fn list_channels(&self) -> Result<Vec<NotificationChannel>, sqlx::Error> {
        sqlx::query_as::<_, NotificationChannel>("SELECT * FROM notification_channels ORDER BY channel")
            .fetch_all(self.pool).await
    }

    pub async fn get_channel(&self, id: i64) -> Result<Option<NotificationChannel>, sqlx::Error> {
        sqlx::query_as::<_, NotificationChannel>("SELECT * FROM notification_channels WHERE id = $1")
            .bind(id).fetch_optional(self.pool).await
    }

    pub async fn upsert_channel(&self, channel: &str, provider: &str, config: serde_json::Value) -> Result<NotificationChannel, sqlx::Error> {
        sqlx::query_as::<_, NotificationChannel>("INSERT INTO notification_channels (channel, provider, config) VALUES ($1,$2,$3) ON CONFLICT (channel) DO UPDATE SET provider = $2, config = $3, updated_at = NOW() RETURNING *")
            .bind(channel).bind(provider).bind(config).fetch_one(self.pool).await
    }

    // ── Notifications ──────────────────────────────────────

    pub async fn send(&self, channel: &str, recipient_id: i64, address: &str, subject: Option<&str>, body: &str) -> Result<Notification, sqlx::Error> {
        sqlx::query_as::<_, Notification>("INSERT INTO notifications (channel, recipient_type, recipient_id, recipient_address, subject, body, status) VALUES ($1,'user',$2,$3,$4,$5,'queued') RETURNING *")
            .bind(channel).bind(recipient_id).bind(address).bind(subject).bind(body).fetch_one(self.pool).await
    }

    pub async fn list_notifications(&self, channel: Option<&str>, status: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<Notification>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM notifications WHERE ($1::text IS NULL OR channel = $1) AND ($2::text IS NULL OR status = $2)")
            .bind(channel).bind(status).fetch_one(self.pool).await?;
        let notifications: Vec<Notification> = sqlx::query_as("SELECT * FROM notifications WHERE ($1::text IS NULL OR channel = $1) AND ($2::text IS NULL OR status = $2) ORDER BY created_at DESC LIMIT $3 OFFSET $4")
            .bind(channel).bind(status).bind(per_page).bind(offset).fetch_all(self.pool).await?;
        Ok((notifications, count_row.0))
    }

    pub async fn retry_notification(&self, id: i64) -> Result<Notification, sqlx::Error> {
        sqlx::query_as::<_, Notification>("UPDATE notifications SET status = 'queued', retry_count = retry_count + 1, updated_at = NOW() WHERE id = $1 AND status = 'failed' AND retry_count < max_retries RETURNING *")
            .bind(id).fetch_one(self.pool).await
    }

    // ── History ────────────────────────────────────────────

    pub async fn list_history(&self, notification_id: Option<i64>, page: i64, per_page: i64) -> Result<(Vec<NotificationHistory>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM notification_history WHERE ($1::bigint IS NULL OR notification_id = $1)")
            .bind(notification_id).fetch_one(self.pool).await?;
        let history: Vec<NotificationHistory> = sqlx::query_as("SELECT * FROM notification_history WHERE ($1::bigint IS NULL OR notification_id = $1) ORDER BY recorded_at DESC LIMIT $2 OFFSET $3")
            .bind(notification_id).bind(per_page).bind(offset).fetch_all(self.pool).await?;
        Ok((history, count_row.0))
    }
}
