use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Trait that all aggregate roots must implement
pub trait AggregateRoot: Send + Sync {
    /// The unique identifier type for this aggregate
    type Id: std::fmt::Display + Clone + Send + Sync + Eq + std::hash::Hash;

    /// Get the aggregate's unique identifier
    fn id(&self) -> Self::Id;

    /// Get the aggregate's version (for optimistic concurrency)
    fn version(&self) -> i64;

    /// Get when this aggregate was created
    fn created_at(&self) -> DateTime<Utc>;

    /// Get when this aggregate was last updated
    fn updated_at(&self) -> DateTime<Utc>;

    /// Check if the aggregate has been soft-deleted
    fn is_deleted(&self) -> bool;

    /// Get uncommitted domain events
    fn uncommitted_events(&self) -> &[DomainEvent];

    /// Mark events as committed (after persistence)
    fn mark_events_committed(&mut self);

    /// Add a domain event
    fn add_event(&mut self, event: DomainEvent);
}

/// Domain event that occurred on an aggregate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainEvent {
    pub event_id: String,
    pub event_type: String,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub version: i64,
    pub occurred_at: DateTime<Utc>,
    pub payload: serde_json::Value,
}

impl DomainEvent {
    pub fn new(
        event_type: &str,
        aggregate_type: &str,
        aggregate_id: &str,
        version: i64,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            event_id: uuid::Uuid::new_v4().to_string(),
            event_type: event_type.to_string(),
            aggregate_type: aggregate_type.to_string(),
            aggregate_id: aggregate_id.to_string(),
            version,
            occurred_at: Utc::now(),
            payload,
        }
    }
}

/// Base struct for aggregates with common fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregateBase {
    pub version: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    #[serde(skip)]
    pub(crate) uncommitted_events: Vec<DomainEvent>,
}

impl AggregateBase {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            version: 1,
            created_at: now,
            updated_at: now,
            deleted_at: None,
            uncommitted_events: Vec::new(),
        }
    }

    pub fn bump_version(&mut self) {
        self.version += 1;
        self.updated_at = Utc::now();
    }

    pub fn soft_delete(&mut self) {
        self.deleted_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Get uncommitted domain events
    pub fn uncommitted_events(&self) -> &[DomainEvent] {
        &self.uncommitted_events
    }

    /// Mark all events as committed (call after persistence)
    pub fn mark_events_committed(&mut self) {
        self.uncommitted_events.clear();
    }

    /// Add a domain event to the uncommitted list
    pub fn add_event(&mut self, event: DomainEvent) {
        self.uncommitted_events.push(event);
    }
}

impl Default for AggregateBase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregate_base_new() {
        let base = AggregateBase::new();
        assert_eq!(base.version, 1);
        assert!(base.deleted_at.is_none());
    }

    #[test]
    fn test_bump_version() {
        let mut base = AggregateBase::new();
        base.bump_version();
        assert_eq!(base.version, 2);
    }

    #[test]
    fn test_domain_event() {
        let event = DomainEvent::new(
            "CustomerCreated",
            "Customer",
            "123",
            1,
            serde_json::json!({"email": "test@example.com"}),
        );
        assert_eq!(event.event_type, "CustomerCreated");
        assert_eq!(event.aggregate_id, "123");
    }
}
