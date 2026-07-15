use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::{info, error, debug};

use crate::infrastructure::messaging::outbox;
use crate::infrastructure::messaging::EventPublisher;

/// Background worker that polls the outbox table and publishes events to NATS.
pub struct OutboxWorker {
    db: Arc<DatabaseConnection>,
    publisher: EventPublisher,
    poll_interval_secs: u64,
    batch_size: u64,
}

impl OutboxWorker {
    pub fn new(db: Arc<DatabaseConnection>, publisher: EventPublisher) -> Self {
        Self {
            db,
            publisher,
            poll_interval_secs: 5,
            batch_size: 100,
        }
    }

    pub fn with_poll_interval(mut self, secs: u64) -> Self {
        self.poll_interval_secs = secs;
        self
    }

    pub fn with_batch_size(mut self, size: u64) -> Self {
        self.batch_size = size;
        self
    }

    /// Start the outbox worker loop.
    pub async fn run(&self) {
        info!(
            poll_interval = self.poll_interval_secs,
            batch_size = self.batch_size,
            "Starting outbox worker"
        );

        let mut ticker = interval(Duration::from_secs(self.poll_interval_secs));

        loop {
            ticker.tick().await;

            match self.process_batch().await {
                Ok(count) if count > 0 => {
                    info!(count = count, "Published batch of outbox events");
                }
                Ok(_) => {
                    debug!("No unpublished events in outbox");
                }
                Err(e) => {
                    error!(error = %e, "Failed to process outbox batch");
                    // Continue loop despite error - next tick will retry
                }
            }
        }
    }

    /// Process a single batch of unpublished events.
    async fn process_batch(&self) -> Result<u64, anyhow::Error> {
        let events = outbox::fetch_unpublished_events(&self.db, self.batch_size).await?;

        let mut published_count: u64 = 0;

        for event in &events {
            // Build NATS subject from event type
            let subject = format!("aeroxe.{}", event.event_type);

            // Publish to NATS
            match self
                .publisher
                .publish_raw(&subject, &event.event_type, &event.payload)
                .await
            {
                Ok(_) => {
                    // Mark as published in outbox
                    outbox::mark_event_published(&self.db, &event.event_id).await?;
                    published_count += 1;
                    debug!(
                        event_id = %event.event_id,
                        event_type = %event.event_type,
                        "Published event from outbox"
                    );
                }
                Err(e) => {
                    error!(
                        event_id = %event.event_id,
                        error = %e,
                        "Failed to publish event from outbox"
                    );
                    // Don't mark as published - will retry on next tick
                }
            }
        }

        Ok(published_count)
    }
}
