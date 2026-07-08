use sqlx::PgPool;
use crate::modules::event::model::event::Event;
pub struct EventRepository<'a> { pool: &'a PgPool }
impl<'a> EventRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }
    pub async fn list(&self, event_type: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<Event>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM events WHERE ($1::text IS NULL OR event_type = $1)").bind(event_type).fetch_one(self.pool).await?;
        let events: Vec<Event> = sqlx::query_as("SELECT * FROM events WHERE ($1::text IS NULL OR event_type = $1) ORDER BY published_at DESC LIMIT $2 OFFSET $3").bind(event_type).bind(per_page).bind(offset).fetch_all(self.pool).await?;
        Ok((events, count_row.0))
    }
}
