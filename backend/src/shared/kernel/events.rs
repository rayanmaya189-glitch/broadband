use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::shared::errors::AppError;

/// Event envelope wrapping all domain events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope<T: Serialize> {
    pub event_id: String,
    pub event_type: String,
    pub version: u32,
    pub occurred_at: DateTime<Utc>,
    pub producer: String,
    pub payload: T,
    pub metadata: HashMap<String, String>,
}

impl<T: Serialize> EventEnvelope<T> {
    pub fn new(event_type: &str, producer: &str, payload: T) -> Self {
        Self {
            event_id: uuid::Uuid::new_v4().to_string(),
            event_type: event_type.to_string(),
            version: 1,
            occurred_at: Utc::now(),
            producer: producer.to_string(),
            payload,
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_branch_id(mut self, branch_id: i64) -> Self {
        self.metadata
            .insert("branch_id".to_string(), branch_id.to_string());
        self
    }
}

/// Trait for domain events
pub trait DomainEventTrait: Send + Sync + Clone {
    /// The event type name
    fn event_type(&self) -> &str;

    /// The aggregate type this event belongs to
    fn aggregate_type(&self) -> &str;

    /// The aggregate ID
    fn aggregate_id(&self) -> &str;

    /// The event version
    fn version(&self) -> u32;
}

/// Event handler trait
#[async_trait]
pub trait EventHandler<E: DomainEventTrait>: Send + Sync {
    /// Handle a domain event
    async fn handle(&self, event: E) -> Result<(), AppError>;
}

/// Event bus for publishing and subscribing to events
#[async_trait]
pub trait EventBus: Send + Sync {
    /// Publish an event
    async fn publish<T: Serialize + Send + Sync>(
        &self,
        event_type: &str,
        payload: T,
    ) -> Result<(), AppError>;

    /// Subscribe to an event type
    async fn subscribe<F>(&self, event_type: &str, handler: F) -> Result<(), AppError>
    where
        F: Fn(
                serde_json::Value,
            )
                -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), AppError>> + Send>>
            + Send
            + Sync
            + 'static;

    /// Subscribe to an event type with a named subscriber
    async fn subscribe_named<F>(
        &self,
        event_type: &str,
        subscriber_name: &str,
        handler: F,
    ) -> Result<(), AppError>
    where
        F: Fn(
                serde_json::Value,
            )
                -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), AppError>> + Send>>
            + Send
            + Sync
            + 'static;
}

/// Event store for persisting events
#[async_trait]
pub trait EventStore: Send + Sync {
    /// Append events to the store
    async fn append(&self, events: Vec<EventEnvelope<serde_json::Value>>) -> Result<(), AppError>;

    /// Load events for an aggregate
    async fn load_events(
        &self,
        aggregate_type: &str,
        aggregate_id: &str,
    ) -> Result<Vec<EventEnvelope<serde_json::Value>>, AppError>;

    /// Load all events of a type
    async fn load_by_type(
        &self,
        event_type: &str,
    ) -> Result<Vec<EventEnvelope<serde_json::Value>>, AppError>;
}

/// NATS event bus implementation
pub struct NatsEventBus {
    client: async_nats::Client,
}

impl NatsEventBus {
    pub fn new(client: async_nats::Client) -> Self {
        Self { client }
    }

    /// Build NATS subject from event type
    fn build_subject(&self, event_type: &str) -> String {
        format!("aeroxe.{}", event_type)
    }
}

#[async_trait]
impl EventBus for NatsEventBus {
    async fn publish<T: Serialize + Send + Sync>(
        &self,
        event_type: &str,
        payload: T,
    ) -> Result<(), AppError> {
        let envelope = EventEnvelope::new(event_type, "aeroxe-backend", payload);
        let subject = self.build_subject(event_type);
        let data = serde_json::to_vec(&envelope)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Serialization error: {}", e)))?;

        self.client
            .publish(subject.clone(), data.into())
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("NATS publish error: {}", e)))?;

        tracing::debug!(subject = %subject, event_type = %event_type, "Published domain event");
        Ok(())
    }

    async fn subscribe<F>(&self, event_type: &str, handler: F) -> Result<(), AppError>
    where
        F: Fn(
                serde_json::Value,
            )
                -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), AppError>> + Send>>
            + Send
            + Sync
            + 'static,
    {
        let subject = self.build_subject(event_type);
        let mut subscriber = self
            .client
            .subscribe(subject.clone())
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("NATS subscribe error: {}", e)))?;

        let handler = std::sync::Arc::new(handler);
        tokio::spawn(async move {
            while let Some(msg) = subscriber.next().await {
                if let Ok(payload) = serde_json::from_slice::<serde_json::Value>(&msg.payload) {
                    if let Err(e) = handler(payload).await {
                        tracing::error!(error = %e, "Event handler error");
                    }
                }
            }
        });

        tracing::info!(subject = %subject, "Subscribed to events");
        Ok(())
    }

    async fn subscribe_named<F>(
        &self,
        event_type: &str,
        subscriber_name: &str,
        handler: F,
    ) -> Result<(), AppError>
    where
        F: Fn(
                serde_json::Value,
            )
                -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), AppError>> + Send>>
            + Send
            + Sync
            + 'static,
    {
        let subject = self.build_subject(event_type);
        let queue_group = format!("aeroxe.{}.{}", event_type, subscriber_name);
        let subscriber_name_owned = subscriber_name.to_string();
        let mut subscriber = self
            .client
            .queue_subscribe(subject.clone(), queue_group.clone())
            .await
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("NATS queue subscribe error: {}", e))
            })?;

        let handler = std::sync::Arc::new(handler);
        tokio::spawn(async move {
            while let Some(msg) = subscriber.next().await {
                if let Ok(payload) = serde_json::from_slice::<serde_json::Value>(&msg.payload) {
                    if let Err(e) = handler(payload).await {
                        tracing::error!(error = %e, subscriber = %subscriber_name_owned, "Event handler error");
                    }
                }
            }
        });

        tracing::info!(subject = %subject, queue_group = %queue_group, "Subscribed to events with queue group");
        Ok(())
    }
}

/// Common event type constants
pub mod event_types {
    pub const CUSTOMER_CREATED: &str = "customer.created.v1";
    pub const CUSTOMER_ACTIVATED: &str = "customer.activated.v1";
    pub const CUSTOMER_SUSPENDED: &str = "customer.suspended.v1";
    pub const CUSTOMER_DELETED: &str = "customer.deleted.v1";

    pub const SUBSCRIPTION_CREATED: &str = "subscription.created.v1";
    pub const SUBSCRIPTION_ACTIVATED: &str = "subscription.activated.v1";
    pub const SUBSCRIPTION_SUSPENDED: &str = "subscription.suspended.v1";
    pub const SUBSCRIPTION_CANCELLED: &str = "subscription.cancelled.v1";

    pub const INVOICE_CREATED: &str = "invoice.created.v1";
    pub const INVOICE_PAID: &str = "invoice.paid.v1";
    pub const INVOICE_OVERDUE: &str = "invoice.overdue.v1";

    pub const PAYMENT_COMPLETED: &str = "payment.completed.v1";
    pub const PAYMENT_FAILED: &str = "payment.failed.v1";

    pub const DEVICE_PROVISIONED: &str = "device.provisioned.v1";
    pub const DEVICE_ONLINE: &str = "device.online.v1";
    pub const DEVICE_OFFLINE: &str = "device.offline.v1";

    pub const TICKET_CREATED: &str = "ticket.created.v1";
    pub const TICKET_RESOLVED: &str = "ticket.resolved.v1";
    pub const TICKET_ESCALATED: &str = "ticket.escalated.v1";

    pub const AUDIT_ACTION: &str = "audit.action.v1";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestEvent {
        customer_id: i64,
        email: String,
    }

    impl DomainEventTrait for TestEvent {
        fn event_type(&self) -> &str {
            "customer.created.v1"
        }

        fn aggregate_type(&self) -> &str {
            "Customer"
        }

        fn aggregate_id(&self) -> &str {
            "123"
        }

        fn version(&self) -> u32 {
            1
        }
    }

    #[test]
    fn test_event_envelope() {
        let event = TestEvent {
            customer_id: 123,
            email: "test@example.com".to_string(),
        };

        let envelope = EventEnvelope::new("customer.created.v1", "test-service", event)
            .with_branch_id(1)
            .with_metadata("source", "api");

        assert_eq!(envelope.event_type, "customer.created.v1");
        assert_eq!(envelope.producer, "test-service");
        assert_eq!(envelope.metadata.get("branch_id").unwrap(), "1");
    }
}
