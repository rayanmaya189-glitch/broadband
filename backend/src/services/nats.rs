//! NATS service — event publishing and JetStream integration.

use async_nats::Client;
use serde::Serialize;

use crate::error::AppError;

/// NATS service for event-driven messaging.
#[derive(Clone)]
pub struct NatsService {
    client: Client,
}

impl NatsService {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Publish a message to a NATS subject.
    pub async fn publish(
        &self,
        subject: &str,
        payload: &str,
    ) -> Result<(), AppError> {
        let payload_bytes = bytes::Bytes::from(payload.to_string());
        self.client
            .publish(subject.to_string(), payload_bytes)
            .await
            .map_err(|e| {
                AppError::External(format!("NATS publish failed: {e}"))
            })?;
        Ok(())
    }

    /// Publish a serializable event to the events stream.
    pub async fn publish_event<T: Serialize>(
        &self,
        event_type: &str,
        payload: &T,
    ) -> Result<(), AppError> {
        let subject = format!("events.{event_type}");
        let json = serde_json::to_string(payload).map_err(|e| {
            AppError::Internal(anyhow::anyhow!("Event serialization failed: {e}"))
        })?;
        self.publish(&subject, &json).await
    }

    /// Publish a WebSocket broadcast message.
    pub async fn broadcast_ws(
        &self,
        channel: &str,
        payload: &str,
    ) -> Result<(), AppError> {
        self.publish(channel, payload).await
    }

    /// Get the raw NATS client for advanced usage.
    pub fn client(&self) -> &Client {
        &self.client
    }
}
