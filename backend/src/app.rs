use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::common::config::config::Config;
use crate::common::errors::app_error::AppError;
use crate::common::events::nats::NatsService;
use crate::common::cache::redis::RedisService;
use crate::modules::realtime::service::realtime_service::ConnectionManager;

/// Shared application state passed to every Axum handler via `State<SharedState>`.
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: RedisService,
    pub nats: NatsService,
    pub config: Arc<Config>,
    pub ws_manager: Arc<ConnectionManager>,
}

pub type SharedState = Arc<AppState>;

impl AppState {
    pub async fn new(config: Config) -> Result<Self, AppError> {
        let db = PgPoolOptions::new()
            .max_connections(config.db_max_connections)
            .min_connections(config.db_min_connections)
            .acquire_timeout(std::time::Duration::from_secs(config.db_connect_timeout_secs))
            .idle_timeout(std::time::Duration::from_secs(config.db_idle_timeout_secs))
            .connect(&config.database_url)
            .await?;

        tracing::info!(max = config.db_max_connections, "PostgreSQL pool connected");

        let redis_client = redis::Client::open(config.redis_url.as_str())
            .map_err(|e| AppError::External(format!("Failed to create Redis client: {e}")))?;

        let redis_conn = redis::aio::ConnectionManager::new(redis_client)
            .await
            .map_err(|e| AppError::External(format!("Failed to connect to Redis: {e}")))?;

        let redis = RedisService::new(redis_conn);
        tracing::info!("Redis connection manager ready");

        let nats_client = async_nats::connect(&config.nats_url)
            .await
            .map_err(|e| AppError::External(format!("Failed to connect to NATS: {e}")))?;

        let nats = NatsService::new(nats_client);
        tracing::info!("NATS connected");

        let ws_manager = Arc::new(ConnectionManager::new());
        tracing::info!("WebSocket connection manager ready");

        Ok(Self { db, redis, nats, config: Arc::new(config), ws_manager })
    }
}
