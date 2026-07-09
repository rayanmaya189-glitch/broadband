use sqlx::PgPool;
use crate::common::errors::app_error::AppError;
use crate::modules::event::repository::event_repository::EventRepository;
use crate::modules::event::request::event_request::*;
use crate::modules::event::response::event_response::*;

pub struct EventService<'a> { repo: EventRepository<'a> }
impl<'a> EventService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: EventRepository::new(pool) } }

    pub async fn list_events(&self, q: EventQuery) -> Result<EventListResponse, AppError> {
        let page = q.page.unwrap_or(1);
        let per_page = q.per_page.unwrap_or(50);
        let (events, total) = self.repo.list(q.event_type.as_deref(), page, per_page).await?;
        Ok(EventListResponse {
            events: events.into_iter().map(|e| EventResponse { id: e.id, event_type: e.event_type, aggregate_type: e.aggregate_type, aggregate_id: e.aggregate_id, payload: e.payload, metadata: e.metadata, caused_by_user_id: e.caused_by_user_id, caused_by_branch_id: e.caused_by_branch_id, sequence_number: e.sequence_number, published_at: e.published_at, processed: e.processed }).collect(),
            total, page, per_page,
        })
    }

    pub async fn get_event(&self, id: i64) -> Result<EventResponse, AppError> {
        let e = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Event not found".into()))?;
        Ok(EventResponse { id: e.id, event_type: e.event_type, aggregate_type: e.aggregate_type, aggregate_id: e.aggregate_id, payload: e.payload, metadata: e.metadata, caused_by_user_id: e.caused_by_user_id, caused_by_branch_id: e.caused_by_branch_id, sequence_number: e.sequence_number, published_at: e.published_at, processed: e.processed })
    }

    pub async fn get_aggregate_events(&self, aggregate_type: &str, aggregate_id: i64) -> Result<Vec<EventResponse>, AppError> {
        let events = self.repo.get_by_aggregate(aggregate_type, aggregate_id).await?;
        Ok(events.into_iter().map(|e| EventResponse { id: e.id, event_type: e.event_type, aggregate_type: e.aggregate_type, aggregate_id: e.aggregate_id, payload: e.payload, metadata: e.metadata, caused_by_user_id: e.caused_by_user_id, caused_by_branch_id: e.caused_by_branch_id, sequence_number: e.sequence_number, published_at: e.published_at, processed: e.processed }).collect())
    }

    pub async fn publish_event(&self, req: PublishEventRequest) -> Result<EventResponse, AppError> {
        let e = self.repo.publish(&req.event_type, &req.aggregate_type, req.aggregate_id, req.payload, req.metadata, None, None).await?;
        Ok(EventResponse { id: e.id, event_type: e.event_type, aggregate_type: e.aggregate_type, aggregate_id: e.aggregate_id, payload: e.payload, metadata: e.metadata, caused_by_user_id: e.caused_by_user_id, caused_by_branch_id: e.caused_by_branch_id, sequence_number: e.sequence_number, published_at: e.published_at, processed: e.processed })
    }

    pub async fn mark_processed(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.mark_processed(id).await? { return Err(AppError::NotFound("Event not found".into())); }
        Ok(MessageResponse { message: "Event marked as processed".into() })
    }

    // ── Subscriptions ───────────────────────────────────────

    pub async fn list_subscriptions(&self) -> Result<Vec<EventSubscriptionResponse>, AppError> {
        let subs = self.repo.list_subscriptions().await?;
        Ok(subs.into_iter().map(|s| EventSubscriptionResponse { id: s.id, subscriber_name: s.subscriber_name, event_type: s.event_type, last_processed_at: s.last_processed_at, is_active: s.is_active, created_at: s.created_at }).collect())
    }

    pub async fn create_subscription(&self, req: CreateSubscriptionRequest) -> Result<EventSubscriptionResponse, AppError> {
        let s = self.repo.create_subscription(&req.subscriber_name, &req.event_type).await?;
        Ok(EventSubscriptionResponse { id: s.id, subscriber_name: s.subscriber_name, event_type: s.event_type, last_processed_at: s.last_processed_at, is_active: s.is_active, created_at: s.created_at })
    }

    pub async fn delete_subscription(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.delete_subscription(id).await? { return Err(AppError::NotFound("Subscription not found".into())); }
        Ok(MessageResponse { message: "Subscription deactivated".into() })
    }

    // ── Stats ───────────────────────────────────────────────

    pub async fn get_stats(&self) -> Result<EventStatsResponse, AppError> {
        let s = self.repo.get_stats().await?;
        Ok(EventStatsResponse { total_events: s.total_events, processed_events: s.processed_events, unprocessed_events: s.unprocessed_events, unique_event_types: s.unique_event_types })
    }
}
