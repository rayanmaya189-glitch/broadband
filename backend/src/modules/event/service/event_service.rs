//! SeaORM-based service for the Event domain.

use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::modules::event::repository::event_repository::EventRepository;
use crate::modules::event::response::event_response::MessageResponse;

pub struct EventService<'a> {
    repo: EventRepository<'a>,
}

impl<'a> EventService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { repo: EventRepository::new(db) }
    }

    pub async fn list(&self, event_type: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<crate::modules::event::model::event_entity::Model>, i64), AppError> {
        self.repo.list(event_type, page, per_page).await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<crate::modules::event::model::event_entity::Model, AppError> {
        self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Event not found".into()))
    }

    pub async fn publish(&self, event_type: &str, aggregate_type: &str, aggregate_id: i64, payload: serde_json::Value, metadata: Option<serde_json::Value>, user_id: Option<i64>, branch_id: Option<i64>) -> Result<crate::modules::event::model::event_entity::Model, AppError> {
        self.repo.publish(event_type, aggregate_type, aggregate_id, payload, metadata, user_id, branch_id).await
    }

    pub async fn mark_processed(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.mark_processed(id).await? { return Err(AppError::NotFound("Event not found".into())); }
        Ok(MessageResponse { message: "Event marked as processed".into() })
    }

    pub async fn list_subscriptions(&self) -> Result<Vec<crate::modules::event::model::event_subscription_entity::Model>, AppError> {
        self.repo.list_subscriptions().await
    }

    pub async fn create_subscription(&self, subscriber_name: &str, event_type: &str) -> Result<crate::modules::event::model::event_subscription_entity::Model, AppError> {
        self.repo.create_subscription(subscriber_name, event_type).await
    }

    pub async fn delete_subscription(&self, id: i64) -> Result<(), AppError> {
        self.repo.delete_subscription(id).await?;
        Ok(())
    }

    pub async fn get_aggregate_events(&self, aggregate_type: &str, aggregate_id: i64) -> Result<Vec<crate::modules::event::model::event_entity::Model>, AppError> {
        self.repo.get_by_aggregate(aggregate_type, aggregate_id).await
    }
}
