use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EventResponse { pub id: i64, pub event_type: String, pub aggregate_type: String, pub aggregate_id: i64, pub payload: Value, pub sequence_number: i64, pub published_at: DateTime<Utc> }
#[derive(Debug, Serialize, Deserialize)]
pub struct EventListResponse { pub events: Vec<EventResponse>, pub total: i64 }
