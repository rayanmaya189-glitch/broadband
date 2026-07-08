use sqlx::PgPool;
use crate::common::errors::app_error::AppError;
use crate::modules::event::repository::event_repository::EventRepository;
use crate::modules::event::request::event_request::*;
use crate::modules::event::response::event_response::*;

pub struct EventService<'a> { repo: EventRepository<'a> }
impl<'a> EventService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: EventRepository::new(pool) } }
    pub async fn list(&self, q: EventQuery) -> Result<EventListResponse, AppError> {
        let page = q.page.unwrap_or(1); let per_page = q.per_page.unwrap_or(50);
        let (events, total) = self.repo.list(q.event_type.as_deref(), page, per_page).await?;
        Ok(EventListResponse { events: events.iter().map(|e| EventResponse { id: e.id, event_type: e.event_type.clone(), aggregate_type: e.aggregate_type.clone(), aggregate_id: e.aggregate_id, payload: e.payload.clone(), sequence_number: e.sequence_number, published_at: e.published_at }).collect(), total })
    }
}
