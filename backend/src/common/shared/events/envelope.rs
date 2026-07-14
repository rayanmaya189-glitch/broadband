//! Event envelope for versioned domain events.
//!
//! All cross-module communication happens over versioned domain events published to NATS.
//! Every event implements the `DomainEvent` trait and carries a standard envelope.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Standard envelope wrapping all domain events.
///
/// # Structure
/// ```json
/// {
///   "event_id": "uuid",
///   "event_type": "customer.created.v1",
///   "version": 1,
///   "occurred_at": "2026-07-14T12:00:00Z",
///   "producer": "customer-service",
///   "payload": { ... }
/// }
/// ```
///
/// # NATS Subject Naming Convention
/// Format: `aeroxe.<context>.<entity>.<action>.<version>`
/// Example: `aeroxe.customer.created.v1`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope<T> {
    /// Unique identifier for this event instance
    pub event_id: Uuid,

    /// Event type identifier (e.g., "customer.created.v1")
    pub event_type: String,

    /// Schema version of the event payload
    pub version: u32,

    /// Timestamp when the event occurred
    pub occurred_at: DateTime<Utc>,

    /// The service/module that produced this event
    pub producer: String,

    /// The actual event payload
    pub payload: T,
}

impl<T> EventEnvelope<T> {
    /// Create a new event envelope with a generated UUID and current timestamp.
    pub fn new(event_type: String, version: u32, producer: String, payload: T) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            event_type,
            version,
            occurred_at: Utc::now(),
            producer,
            payload,
        }
    }

    /// Create an envelope with a specific event ID (useful for testing).
    pub fn with_id(
        event_id: Uuid,
        event_type: String,
        version: u32,
        producer: String,
        payload: T,
    ) -> Self {
        Self {
            event_id,
            event_type,
            version,
            occurred_at: Utc::now(),
            producer,
            payload,
        }
    }

    /// Map the payload to a different type while preserving the envelope metadata.
    pub fn map_payload<U, F>(self, f: F) -> EventEnvelope<U>
    where
        F: FnOnce(T) -> U,
    {
        EventEnvelope {
            event_id: self.event_id,
            event_type: self.event_type,
            version: self.version,
            occurred_at: self.occurred_at,
            producer: self.producer,
            payload: f(self.payload),
        }
    }
}

/// Trait that all domain events must implement.
pub trait DomainEvent {
    /// Returns the event type string (e.g., "customer.created.v1")
    fn event_type(&self) -> &str;

    /// Returns the schema version of this event
    fn version(&self) -> u32;

    /// Returns the NATS subject for this event
    fn nats_subject(&self) -> String {
        format!("aeroxe.{}", self.event_type())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_event_envelope_creation() {
        let payload = json!({"customer_id": 123});
        let envelope = EventEnvelope::new(
            "customer.created.v1".to_string(),
            1,
            "customer-service".to_string(),
            payload.clone(),
        );

        assert_eq!(envelope.event_type, "customer.created.v1");
        assert_eq!(envelope.version, 1);
        assert_eq!(envelope.producer, "customer-service");
        assert_eq!(envelope.payload, payload);
        assert!(envelope.event_id.to_string().len() > 0);
    }

    #[test]
    fn test_event_envelope_serialization() {
        let payload = json!({"customer_id": 123});
        let envelope = EventEnvelope::new(
            "customer.created.v1".to_string(),
            1,
            "customer-service".to_string(),
            payload,
        );

        let serialized = serde_json::to_string(&envelope).unwrap();
        let deserialized: EventEnvelope<serde_json::Value> = serde_json::from_str(&serialized).unwrap();

        assert_eq!(envelope.event_id, deserialized.event_id);
        assert_eq!(envelope.event_type, deserialized.event_type);
        assert_eq!(envelope.version, deserialized.version);
    }

    #[test]
    fn test_domain_event_trait() {
        struct TestEvent;

        impl DomainEvent for TestEvent {
            fn event_type(&self) -> &str {
                "test.created.v1"
            }

            fn version(&self) -> u32 {
                1
            }
        }

        let event = TestEvent;
        assert_eq!(event.nats_subject(), "aeroxe.test.created.v1");
    }
}
