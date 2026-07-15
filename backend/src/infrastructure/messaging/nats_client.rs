use async_nats::Client;
use tracing::info;

use crate::shared::errors::AppError;

/// Connect to NATS server and optionally set up JetStream.
pub async fn connect_nats(url: &str) -> Result<Client, AppError> {
    let client = async_nats::connect(url)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to connect to NATS: {}", e)))?;

    info!(url = %url, "Connected to NATS");
    Ok(client)
}

/// JetStream stream configuration for events.
pub struct JetStreamConfig {
    pub stream_name: String,
    pub subjects: Vec<String>,
    pub max_messages: i64,
    pub max_bytes: i64,
    pub max_age_secs: i64,
}

impl Default for JetStreamConfig {
    fn default() -> Self {
        Self {
            stream_name: "EVENTS".to_string(),
            subjects: vec!["aeroxe.>".to_string()],
            max_messages: 1_000_000,
            max_bytes: 1_073_741_824,     // 1GB
            max_age_secs: 30 * 24 * 3600, // 30 days
        }
    }
}

/// Ensure the JetStream stream exists, creating it if necessary.
pub async fn ensure_jetstream_stream(
    client: &Client,
    config: &JetStreamConfig,
) -> Result<(), AppError> {
    let jetstream = async_nats::jetstream::new(client.clone());

    // Try to get existing stream, create if not found
    match jetstream.get_stream(&config.stream_name).await {
        Ok(_) => {
            info!(stream = %config.stream_name, "JetStream stream already exists");
        }
        Err(_) => {
            let _subject_labels: Vec<&str> = config.subjects.iter().map(|s| s.as_str()).collect();
            let stream_config = async_nats::jetstream::stream::Config {
                name: config.stream_name.clone(),
                subjects: config.subjects.clone(),
                max_messages: config.max_messages,
                max_bytes: config.max_bytes,
                max_age: std::time::Duration::from_secs(config.max_age_secs as u64),
                storage: async_nats::jetstream::stream::StorageType::File,
                discard: async_nats::jetstream::stream::DiscardPolicy::Old,
                ..Default::default()
            };

            jetstream.create_stream(stream_config).await.map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to create JetStream stream: {}", e))
            })?;

            info!(stream = %config.stream_name, "Created JetStream stream");
        }
    }

    Ok(())
}
