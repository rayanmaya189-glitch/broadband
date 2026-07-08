use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::config::Config;
use crate::error::AppError;
use crate::services::nats::NatsService;
use crate::services::redis_client::RedisService;

/// Shared application state passed to every Axum handler via `State<SharedState>`.
#[derive(Clone)]
pub struct AppState {
    /// PostgreSQL connection pool.
    pub db: PgPool,
    /// Redis service (caching, rate limiting, pub/sub).
    pub redis: RedisService,
    /// NATS service (event-driven messaging).
    pub nats: NatsService,
    /// Application configuration.
    pub config: Arc<Config>,
}

/// Shared state type alias.
pub type SharedState = Arc<AppState>;

impl AppState {
    /// Build the application state by connecting to all backing services.
    pub async fn new(config: Config) -> Result<Self, AppError> {
        // ── PostgreSQL ───────────────────────────────────────
        let db = PgPoolOptions::new()
            .max_connections(config.db_max_connections)
            .min_connections(config.db_min_connections)
            .acquire_timeout(std::time::Duration::from_secs(
                config.db_connect_timeout_secs,
            ))
            .idle_timeout(std::time::Duration::from_secs(config.db_idle_timeout_secs))
            .connect(&config.database_url)
            .await?;

        tracing::info!(
            max = config.db_max_connections,
            "PostgreSQL pool connected"
        );

        // ── Redis ───────────────────────────────────────────
        let redis_client =
            redis::Client::open(config.redis_url.as_str()).map_err(|e| {
                AppError::External(format!("Failed to create Redis client: {e}"))
            })?;

        let redis_conn = redis::aio::ConnectionManager::new(redis_client)
            .await
            .map_err(|e| {
                AppError::External(format!("Failed to connect to Redis: {e}"))
            })?;

        let redis = RedisService::new(redis_conn);
        tracing::info!("Redis connection manager ready");

        // ── NATS ────────────────────────────────────────────
        let nats_client = async_nats::connect(&config.nats_url)
            .await
            .map_err(|e| {
                AppError::External(format!("Failed to connect to NATS: {e}"))
            })?;

        let nats = NatsService::new(nats_client);
        tracing::info!("NATS connected");

        Ok(Self {
            db,
            redis,
            nats,
            config: Arc::new(config),
        })
    }
}
