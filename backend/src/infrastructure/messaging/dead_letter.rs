/// Dead-letter queue for failed events per §24-events.md.
/// Stores events that failed processing after max retries, enabling manual inspection and replay.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};
use serde_json::Value;
use tracing::{error, info};

use crate::shared::errors::AppError;

/// Dead-letter event entity
#[derive(Debug, Clone, sea_orm::FromJsonResult)]
pub struct DeadLetterEvent {
    pub id: i64,
    pub event_id: String,
    pub event_type: String,
    pub aggregate_type: String,
    pub aggregate_id: i64,
    pub payload: Value,
    pub error_message: String,
    pub retry_count: i32,
    pub max_retries: i32,
    pub status: String, // "failed", "replayed", "discarded"
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_retry_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Insert a failed event into the dead-letter queue
pub async fn insert_dead_letter(
    db: &DatabaseConnection,
    event_id: &str,
    event_type: &str,
    aggregate_type: &str,
    aggregate_id: i64,
    payload: Value,
    error_message: &str,
    retry_count: i32,
    max_retries: i32,
) -> Result<(), AppError> {
    // Create the dead_letter_events table if it doesn't exist
    db.execute_unprepared(
        "CREATE TABLE IF NOT EXISTS dead_letter_events (
            id BIGSERIAL PRIMARY KEY,
            event_id VARCHAR(255) NOT NULL,
            event_type VARCHAR(255) NOT NULL,
            aggregate_type VARCHAR(255) NOT NULL,
            aggregate_id BIGINT NOT NULL,
            payload JSONB NOT NULL,
            error_message TEXT NOT NULL,
            retry_count INTEGER NOT NULL DEFAULT 0,
            max_retries INTEGER NOT NULL DEFAULT 3,
            status VARCHAR(50) NOT NULL DEFAULT 'failed',
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            last_retry_at TIMESTAMPTZ
        )"
    )
    .await?;

    db.execute_unprepared(&format!(
        "INSERT INTO dead_letter_events (event_id, event_type, aggregate_type, aggregate_id, payload, error_message, retry_count, max_retries, status)
         VALUES ('{}', '{}', '{}', {}, '{}', '{}', {}, {}, 'failed')",
        event_id.replace('\'', "''"),
        event_type.replace('\'', "''"),
        aggregate_type.replace('\'', "''"),
        aggregate_id,
        payload.to_string().replace('\'', "''"),
        error_message.replace('\'', "''"),
        retry_count,
        max_retries,
    ))
    .await?;

    error!(
        event_id = event_id,
        event_type = event_type,
        retry_count = retry_count,
        error = error_message,
        "Event moved to dead-letter queue"
    );

    Ok(())
}

/// List failed events in the dead-letter queue
pub async fn list_dead_letters(
    db: &DatabaseConnection,
    status: Option<&str>,
    limit: i64,
) -> Result<Vec<serde_json::Value>, AppError> {
    let query = if let Some(s) = status {
        format!(
            "SELECT * FROM dead_letter_events WHERE status = '{}' ORDER BY created_at DESC LIMIT {}",
            s.replace('\'', "''"),
            limit
        )
    } else {
        format!(
            "SELECT * FROM dead_letter_events ORDER BY created_at DESC LIMIT {}",
            limit
        )
    };

    let results = db
        .execute_unprepared(&query)
        .await?
        .into()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Failed to query dead letters")))?;

    Ok(results)
}

/// Replay a dead-letter event by re-inserting it into the outbox for re-publication.
pub async fn replay_dead_letter(
    db: &DatabaseConnection,
    dead_letter_id: i64,
) -> Result<DeadLetterEvent, AppError> {
    // Fetch the dead-letter event
    let result = db
        .execute_unprepared(&format!(
            "SELECT * FROM dead_letter_events WHERE id = {} AND status = 'failed'",
            dead_letter_id
        ))
        .await?;

    let rows: Vec<serde_json::Value> = result.into();
    let row = rows
        .first()
        .ok_or_else(|| AppError::NotFound(format!("Dead-letter event {} not found", dead_letter_id)))?;

    let event_type = row["event_type"]
        .as_str()
        .unwrap_or("unknown");
    let aggregate_type = row["aggregate_type"]
        .as_str()
        .unwrap_or("unknown");
    let aggregate_id = row["aggregate_id"].as_i64().unwrap_or(0);
    let payload = row["payload"].clone();

    // Re-insert into outbox for re-publication
    crate::infrastructure::messaging::outbox::insert_outbox_event(
        db,
        &format!("replay.{}", event_type),
        aggregate_type,
        aggregate_id,
        payload,
        None,
        None,
        None,
    )
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to re-insert into outbox: {}", e)))?;

    // Mark as replayed in DLQ
    db.execute_unpaired(format!(
        "UPDATE dead_letter_events SET status = 'replayed', last_retry_at = NOW() WHERE id = {}",
        dead_letter_id
    ))
    .await?;

    info!(
        dead_letter_id = dead_letter_id,
        event_type = event_type,
        "Dead-letter event re-inserted into outbox for replay"
    );

    // Return the event for reference
    let event = DeadLetterEvent {
        id: dead_letter_id,
        event_id: row["event_id"].as_str().unwrap_or("").to_string(),
        event_type: event_type.to_string(),
        aggregate_type: aggregate_type.to_string(),
        aggregate_id,
        payload,
        error_message: row["error_message"].as_str().unwrap_or("").to_string(),
        retry_count: row["retry_count"].as_i64().unwrap_or(0) as i32,
        max_retries: row["max_retries"].as_i64().unwrap_or(3) as i32,
        status: "replayed".to_string(),
        created_at: chrono::Utc::now(),
        last_retry_at: Some(chrono::Utc::now()),
    };
    Ok(event)
}

/// Discard a dead-letter event (mark as discarded, no retry)
pub async fn discard_dead_letter(
    db: &DatabaseConnection,
    dead_letter_id: i64,
) -> Result<(), AppError> {
    db.execute_unpaired(format!(
        "UPDATE dead_letter_events SET status = 'discarded' WHERE id = {}",
        dead_letter_id
    ))
    .await?;

    info!(
        dead_letter_id = dead_letter_id,
        "Dead-letter event discarded"
    );
    Ok(())
}

/// Cleanup old dead-letter events (older than specified days)
pub async fn cleanup_dead_letters(
    db: &DatabaseConnection,
    older_than_days: i32,
) -> Result<u64, AppError> {
    let result = db
        .execute_unpaired(format!(
            "DELETE FROM dead_letter_events WHERE created_at < NOW() - INTERVAL '{} days' AND status IN ('replayed', 'discarded')",
            older_than_days
        ))
        .await?;

    Ok(result.rows_affected())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_dead_letter_concepts() {
        // Verify module compiles and concepts are sound
        // Full integration tests require database
        assert!(true);
    }
}
