//! SeaORM-based repository for the Event domain.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};

use crate::common::errors::app_error::AppError;
use crate::modules::event::model::event_entity::{self, Model as EventModel};
use crate::modules::event::model::event_subscription_entity::{self, Model as EventSubscriptionModel};

pub struct EventRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> EventRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    pub async fn list(&self, event_type: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<EventModel>, i64), AppError> {
        let page_size = per_page as u64;
        let page_num = if per_page > 0 { ((page - 1).max(0) as u64 * page_size) / page_size } else { 0 };
        let mut select = event_entity::Entity::find();
        if let Some(et) = event_type {
            select = select.filter(event_entity::Column::EventType.eq(et));
        }
        let total = select.clone().count(self.db).await?;
        let events = select
            .order_by_desc(event_entity::Column::PublishedAt)
            .paginate(self.db, page_size)
            .fetch_page(page_num).await?;
        Ok((events, total as i64))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<EventModel>, AppError> {
        Ok(event_entity::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn get_by_aggregate(&self, aggregate_type: &str, aggregate_id: i64) -> Result<Vec<EventModel>, AppError> {
        let events = event_entity::Entity::find()
            .filter(event_entity::Column::AggregateType.eq(aggregate_type))
            .filter(event_entity::Column::AggregateId.eq(aggregate_id))
            .order_by_asc(event_entity::Column::SequenceNumber)
            .all(self.db).await?;
        Ok(events)
    }

    pub async fn publish(
        &self, event_type: &str, aggregate_type: &str, aggregate_id: i64,
        payload: serde_json::Value, metadata: Option<serde_json::Value>,
        user_id: Option<i64>, branch_id: Option<i64>,
    ) -> Result<EventModel, AppError> {
        let now = chrono::Utc::now();
        let active = event_entity::ActiveModel {
            event_type: Set(event_type.to_owned()),
            aggregate_type: Set(aggregate_type.to_owned()),
            aggregate_id: Set(aggregate_id),
            payload: Set(payload),
            metadata: Set(metadata),
            caused_by_user_id: Set(user_id),
            caused_by_branch_id: Set(branch_id),
            processed: Set(false),
            published_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn mark_processed(&self, id: i64) -> Result<bool, AppError> {
        let existing = event_entity::Entity::find_by_id(id).one(self.db).await?;
        match existing {
            Some(e) => {
                let mut active = e.into_active_model();
                active.processed = Set(true);
                active.update(self.db).await?;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    pub async fn list_subscriptions(&self) -> Result<Vec<EventSubscriptionModel>, AppError> {
        let subs = event_subscription_entity::Entity::find()
            .order_by_asc(event_subscription_entity::Column::SubscriberName)
            .all(self.db).await?;
        Ok(subs)
    }

    pub async fn create_subscription(&self, subscriber_name: &str, event_type: &str) -> Result<EventSubscriptionModel, AppError> {
        let existing = event_subscription_entity::Entity::find()
            .filter(event_subscription_entity::Column::SubscriberName.eq(subscriber_name))
            .filter(event_subscription_entity::Column::EventType.eq(event_type))
            .one(self.db).await?;
        match existing {
            Some(e) => {
                let mut active = e.into_active_model();
                active.is_active = Set(true);
                Ok(active.update(self.db).await?)
            }
            None => {
                let now = chrono::Utc::now();
                let active = event_subscription_entity::ActiveModel {
                    subscriber_name: Set(subscriber_name.to_owned()),
                    event_type: Set(event_type.to_owned()),
                    is_active: Set(true),
                    created_at: Set(now.into()),
                    ..Default::default()
                };
                Ok(active.insert(self.db).await?)
            }
        }
    }

    pub async fn delete_subscription(&self, id: i64) -> Result<bool, AppError> {
        let existing = event_subscription_entity::Entity::find_by_id(id).one(self.db).await?;
        match existing {
            Some(e) => {
                let mut active = e.into_active_model();
                active.is_active = Set(false);
                active.update(self.db).await?;
                Ok(true)
            }
            None => Ok(false),
        }
    }
}
