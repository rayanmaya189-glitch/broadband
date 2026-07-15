use sea_orm::{ConnectionTrait, DatabaseConnection, Statement, DbBackend};
use serde_json::Value;
use tracing::debug;

use crate::shared::errors::AppError;

/// Outbox event for reliable event publishing.
/// Events are stored within the same DB transaction as business logic,
/// then a background worker polls and publishes to NATS.
#[derive(Debug, Clone)]
pub struct OutboxEvent {
    pub id: i64,
    pub event_id: String,
    pub event_type: String,
    pub aggregate_type: String,
    pub aggregate_id: i64,
    pub payload: Value,
    pub metadata: Option<Value>,
    pub caused_by_user_id: Option<i64>,
    pub caused_by_branch_id: Option<i64>,
    pub published: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

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
) -> Result<(), AppError> {
    let event_id = uuid::Uuid::new_v4().to_string();

    let metadata_json = match &metadata {
        Some(m) => format!("'{}'", m.to_string().replace('\'', "''")),
        None => "NULL".to_string(),
    };
    let user_id_str = caused_by_user_id.map_or("NULL".to_string(), |id| id.to_string());
    let branch_id_str = caused_by_branch_id.map_or("NULL".to_string(), |id| id.to_string());

    let sql = format!(
        "INSERT INTO outbox_events (event_id, event_type, aggregate_type, aggregate_id, payload, metadata, caused_by_user_id, caused_by_branch_id, published, created_at)
         VALUES ('{}', '{}', '{}', {}, '{}'::jsonb, {}::jsonb, {}, {}, false, NOW())",
        event_id,
        event_type.replace('\'', "''"),
        aggregate_type.replace('\'', "''"),
        aggregate_id,
        payload.to_string().replace('\'', "''"),
        metadata_json,
        user_id_str,
        branch_id_str,
    );

    db.execute(Statement::from_string(DbBackend::Postgres, sql))
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to insert outbox event: {}", e)))?;

    debug!(event_id = %event_id, event_type = %event_type, "Stored event in outbox");
    Ok(())
}

/// Fetch unpublished events from the outbox (for the background worker).
pub async fn fetch_unpublished_events(
    db: &DatabaseConnection,
    limit: i64,
) -> Result<Vec<OutboxEvent>, AppError> {
    let sql = format!(
        "SELECT id, event_id, event_type, aggregate_type, aggregate_id, payload, metadata,
                caused_by_user_id, caused_by_branch_id, published, created_at
         FROM outbox_events
         WHERE published = false
         ORDER BY created_at ASC
         LIMIT {}",
        limit
    );

    let results = db
        .query_all(Statement::from_string(DbBackend::Postgres, sql))
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to fetch outbox events: {}", e)))?;

    let mut events = Vec::new();
    for row in results {
        let event = OutboxEvent {
            id: row.try_get_by_index(0).unwrap_or_default(),
            event_id: row.try_get_by_index(1).unwrap_or_default(),
            event_type: row.try_get_by_index(2).unwrap_or_default(),
            aggregate_type: row.try_get_by_index(3).unwrap_or_default(),
            aggregate_id: row.try_get_by_index(4).unwrap_or_default(),
            payload: row.try_get_by_index(5).unwrap_or_default(),
            metadata: row.try_get_by_index(6).ok(),
            caused_by_user_id: row.try_get_by_index(7).ok().flatten(),
            caused_by_branch_id: row.try_get_by_index(8).ok().flatten(),
            published: row.try_get_by_index(9).unwrap_or_default(),
            created_at: row.try_get_by_index(10).unwrap_or_default(),
        };
        events.push(event);
    }

    debug!(count = events.len(), "Fetched unpublished events from outbox");
    Ok(events)
}

/// Mark an event as published in the outbox.
pub async fn mark_event_published(db: &DatabaseConnection, event_id: &str) -> Result<(), AppError> {
    let sql = format!(
        "UPDATE outbox_events SET published = true WHERE event_id = '{}'",
        event_id.replace('\'', "''")
    );

    db.execute(Statement::from_string(DbBackend::Postgres, sql))
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to mark event published: {}", e)))?;

    debug!(event_id = %event_id, "Marked event as published");
    Ok(())
}
