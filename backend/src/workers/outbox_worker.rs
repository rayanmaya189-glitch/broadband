use sea_orm::{DatabaseConnection, TransactionTrait};
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::{debug, error, info};

use crate::infrastructure::messaging::outbox;
use crate::infrastructure::messaging::EventPublisher;
use crate::infrastructure::metrics::Metrics;

/// Background worker that polls the outbox table and publishes events to NATS.
pub struct OutboxWorker {
    db: Arc<DatabaseConnection>,
    publisher: EventPublisher,
    poll_interval_secs: u64,
    batch_size: u64,
    metrics: Option<Arc<tokio::sync::RwLock<Metrics>>>,
}

impl OutboxWorker {
    pub fn new(db: Arc<DatabaseConnection>, publisher: EventPublisher) -> Self {
        Self {
            db,
            publisher,
            poll_interval_secs: 5,
            batch_size: 100,
            metrics: None,
        }
    }

    pub fn with_metrics(mut self, metrics: Arc<tokio::sync::RwLock<Metrics>>) -> Self {
        self.metrics = Some(metrics);
        self
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
        let txn = self.db.begin().await?;
        let events = outbox::fetch_unpublished_events(&txn, self.batch_size).await?;

        let mut published_count: u64 = 0;

        for event in &events {
            let subject = format!("events.{}", event.event_type);

            match self
                .publisher
                .publish_raw(&subject, &event.event_type, &event.payload)
                .await
            {
                Ok(_) => {
                    outbox::mark_event_published(&txn, &event.event_id).await?;
                    published_count += 1;
                    if let Some(ref metrics) = self.metrics {
                        metrics.read().await.nats_messages_published.inc();
                    }
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
                    if let Err(dlq_err) = outbox::record_publish_failure(&txn, &event.event_id, &e.to_string()).await {
                        error!(
                            event_id = %event.event_id,
                            error = %dlq_err,
                            "Failed to record publish failure for outbox event"
                        );
                    }
                }
            }
        }

        txn.commit().await?;
        Ok(published_count)
    }
}
