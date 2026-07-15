use sea_orm::{
    prelude::Expr, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use serde_json::Value;
use tracing::debug;

use crate::shared::errors::AppError;

use crate::infrastructure::messaging::outbox_entity::{self, Entity as OutboxEventEntity, ActiveModel, Model as OutboxEventModel};

/// Outbox event for reliable event publishing.
/// Events are stored within the same DB transaction as business logic,
/// then a background worker polls and publishes to NATS.
pub use outbox_entity::Model as OutboxEvent;

/// Insert an event into the outbox table within the current transaction.
pub async fn insert_outbox_event(
    db: &DatabaseConnection,
    event_type: &str,
    aggregate_type: &str,
    aggregate_id: i64,
    payload: Value,
    metadata: Option<Value>,
    caused_by_user_id: Option<i64>,
    caused_by_branch_id: Option<i64>,
) -> Result<i64, AppError> {
    let event_id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now();

    let model = ActiveModel {
        event_id: Set(event_id.clone()),
        event_type: Set(event_type.to_string()),
        aggregate_type: Set(aggregate_type.to_string()),
        aggregate_id: Set(aggregate_id),
        payload: Set(payload),
        metadata: Set(metadata),
        caused_by_user_id: Set(caused_by_user_id),
        caused_by_branch_id: Set(caused_by_branch_id),
        published: Set(false),
        created_at: Set(now),
        ..Default::default()
    };

    let inserted: outbox_entity::Model = model.insert(db).await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to insert outbox event: {}", e)))?;

    debug!(event_id = %event_id, event_type = %event_type, "Stored event in outbox");
    Ok(inserted.id)
}

/// Fetch unpublished events from the outbox (for the background worker).
pub async fn fetch_unpublished_events(
    db: &DatabaseConnection,
    limit: u64,
) -> Result<Vec<OutboxEventModel>, AppError> {
    let events = OutboxEventEntity::find()
        .filter(outbox_entity::Column::Published.eq(false))
        .order_by_asc(outbox_entity::Column::CreatedAt)
        .limit(limit)
        .all(db)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to fetch outbox events: {}", e)))?;

    debug!(count = events.len(), "Fetched unpublished events from outbox");
    Ok(events)
}

/// Mark an event as published in the outbox.
pub async fn mark_event_published(db: &DatabaseConnection, event_id: &str) -> Result<(), AppError> {
    let result = OutboxEventEntity::update_many()
        .col_expr(outbox_entity::Column::Published, Expr::value(true))
        .filter(outbox_entity::Column::EventId.eq(event_id))
        .exec(db)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to mark event published: {}", e)))?;

    debug!(event_id = %event_id, rows_affected = result.rows_affected, "Marked event as published");
    Ok(())
}

/// Delete old published events (cleanup worker).
pub async fn cleanup_published_events(
    db: &DatabaseConnection,
    older_than_hours: i64,
) -> Result<u64, AppError> {
    let cutoff = chrono::Utc::now() - chrono::Duration::hours(older_than_hours);

    let result = OutboxEventEntity::delete_many()
        .filter(outbox_entity::Column::Published.eq(true))
        .filter(outbox_entity::Column::CreatedAt.lt(cutoff))
        .exec(db)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to cleanup outbox events: {}", e)))?;

    debug!(rows_deleted = result.rows_affected, "Cleaned up published outbox events");
    Ok(result.rows_affected)
}
