use sqlx::PgPool;
use crate::modules::event::model::event::*;

pub struct EventRepository<'a> { pool: &'a PgPool }
impl<'a> EventRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }

    pub async fn list(&self, event_type: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<Event>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM events WHERE ($1::text IS NULL OR event_type = $1)").bind(event_type).fetch_one(self.pool).await?;
        let events: Vec<Event> = sqlx::query_as("SELECT id, event_type, aggregate_type, aggregate_id, payload, metadata, caused_by_user_id, caused_by_branch_id, sequence_number, published_at, processed FROM events WHERE ($1::text IS NULL OR event_type = $1) ORDER BY published_at DESC LIMIT $2 OFFSET $3").bind(event_type).bind(per_page).bind(offset).fetch_all(self.pool).await?;
        Ok((events, count_row.0))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Event>, sqlx::Error> {
        sqlx::query_as::<_, Event>("SELECT id, event_type, aggregate_type, aggregate_id, payload, metadata, caused_by_user_id, caused_by_branch_id, sequence_number, published_at, processed FROM events WHERE id = $1").bind(id).fetch_optional(self.pool).await
    }

    pub async fn get_by_aggregate(&self, aggregate_type: &str, aggregate_id: i64) -> Result<Vec<Event>, sqlx::Error> {
        sqlx::query_as::<_, Event>("SELECT id, event_type, aggregate_type, aggregate_id, payload, metadata, caused_by_user_id, caused_by_branch_id, sequence_number, published_at, processed FROM events WHERE aggregate_type = $1 AND aggregate_id = $2 ORDER BY sequence_number").bind(aggregate_type).bind(aggregate_id).fetch_all(self.pool).await
    }

    pub async fn publish(&self, event_type: &str, aggregate_type: &str, aggregate_id: i64, payload: serde_json::Value, metadata: Option<serde_json::Value>, user_id: Option<i64>, branch_id: Option<i64>) -> Result<Event, sqlx::Error> {
        sqlx::query_as::<_, Event>(
            "INSERT INTO events (event_type, aggregate_type, aggregate_id, payload, metadata, caused_by_user_id, caused_by_branch_id) VALUES ($1,$2,$3,$4,$5,$6,$7) RETURNING id, event_type, aggregate_type, aggregate_id, payload, metadata, caused_by_user_id, caused_by_branch_id, sequence_number, published_at, processed"
        ).bind(event_type).bind(aggregate_type).bind(aggregate_id).bind(payload).bind(metadata).bind(user_id).bind(branch_id).fetch_one(self.pool).await
    }

    pub async fn mark_processed(&self, id: i64) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("UPDATE events SET processed = true WHERE id = $1").bind(id).execute(self.pool).await?;
        Ok(r.rows_affected() > 0)
    }

    // ── Subscriptions ───────────────────────────────────────

    pub async fn list_subscriptions(&self) -> Result<Vec<EventSubscription>, sqlx::Error> {
        sqlx::query_as::<_, EventSubscription>("SELECT id, subscriber_name, event_type, last_processed_id, last_processed_at, is_active, created_at FROM event_subscriptions ORDER BY subscriber_name").fetch_all(self.pool).await
    }

    pub async fn create_subscription(&self, subscriber_name: &str, event_type: &str) -> Result<EventSubscription, sqlx::Error> {
        sqlx::query_as::<_, EventSubscription>(
            "INSERT INTO event_subscriptions (subscriber_name, event_type) VALUES ($1,$2) ON CONFLICT (subscriber_name, event_type) DO UPDATE SET is_active = true RETURNING id, subscriber_name, event_type, last_processed_id, last_processed_at, is_active, created_at"
        ).bind(subscriber_name).bind(event_type).fetch_one(self.pool).await
    }

    pub async fn delete_subscription(&self, id: i64) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("UPDATE event_subscriptions SET is_active = false WHERE id = $1").bind(id).execute(self.pool).await?;
        Ok(r.rows_affected() > 0)
    }

    // ── Stats ───────────────────────────────────────────────

    pub async fn get_stats(&self) -> Result<EventStats, sqlx::Error> {
        let row = sqlx::query_as::<_, (i64, i64, i64, i64)>(
            "SELECT
                (SELECT COUNT(*) FROM events) as total,
                (SELECT COUNT(*) FROM events WHERE processed = true) as processed,
                (SELECT COUNT(*) FROM events WHERE processed = false) as unprocessed,
                (SELECT COUNT(DISTINCT event_type) FROM events) as unique_types"
        ).fetch_one(self.pool).await?;
        Ok(EventStats { total_events: row.0, processed_events: row.1, unprocessed_events: row.2, unique_event_types: row.3 })
    }
}
