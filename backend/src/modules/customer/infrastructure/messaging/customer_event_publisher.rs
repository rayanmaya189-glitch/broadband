//! Customer event publisher.
//!
//! Publishes customer domain events to NATS.

use crate::common::events::nats::NatsService;
use crate::common::shared::events::EventEnvelope;
use crate::modules::customer::domain::aggregates::customer::customer::CustomerEvent;

/// Publisher for customer domain events.
pub struct CustomerEventPublisher {
    nats: NatsService,
}

impl CustomerEventPublisher {
    /// Create a new customer event publisher.
    pub fn new(nats: NatsService) -> Self {
        Self { nats }
    }

    /// Publish a customer event to NATS.
    ///
    /// # NATS Subject Format
    /// `aeroxe.customer.<action>.v1`
    pub async fn publish(&self, event: EventEnvelope<CustomerEvent>) -> Result<(), String> {
        let subject = format!("aeroxe.{}", event.event_type);

        self.nats
            .publish_event(&subject, &event.payload)
            .await
            .map_err(|e| format!("Failed to publish event: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn test_event_subject_format() {
        let event = EventEnvelope::with_id(
            Uuid::new_v4(),
            "customer.created.v1".to_string(),
            1,
            "customer-service".to_string(),
            CustomerEvent::Created {
                customer_id: 1,
                customer_code: "AX-GEN-202607-0001".to_string(),
                first_name: "John".to_string(),
                last_name: Some("Doe".to_string()),
                email: Some("john@example.com".to_string()),
                phone: "+1234567890".to_string(),
                branch_id: 1,
            },
        );

        let subject = format!("aeroxe.{}", event.event_type);
        assert_eq!(subject, "aeroxe.customer.created.v1");
    }
}
