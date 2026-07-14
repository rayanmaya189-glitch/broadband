//! Billing event publisher.

use crate::common::events::nats::NatsService;
use crate::common::shared::events::EventEnvelope;
use crate::modules::billing::domain::aggregates::invoice::invoice::InvoiceEvent;

/// Publisher for billing domain events.
pub struct BillingEventPublisher {
    nats: NatsService,
}

impl BillingEventPublisher {
    pub fn new(nats: NatsService) -> Self {
        Self { nats }
    }

    pub async fn publish(&self, event: EventEnvelope<InvoiceEvent>) -> Result<(), String> {
        let subject = format!("aeroxe.{}", event.event_type);
        self.nats
            .publish_event(&subject, &event.payload)
            .await
            .map_err(|e| format!("Failed to publish event: {}", e))
    }
}
