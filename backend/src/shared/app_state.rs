use std::sync::Arc;

use crate::config::settings::Settings;
use crate::infrastructure::messaging::EventPublisher;
use crate::infrastructure::storage::StorageService;
use crate::shared::middleware::rate_limit::RateLimitStore;
use crate::infrastructure::metrics::SharedMetrics;
use crate::shared::utils::jwt_keys::JwtKeyPair;

/// Shared application state available to all handlers.
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
    pub redis: redis::aio::ConnectionManager,
    pub nats: Option<async_nats::Client>,
    pub event_publisher: Option<EventPublisher>,
    pub settings: Settings,
    pub storage: Option<StorageService>,
    pub rate_limit_store: Arc<RateLimitStore>,
    pub jwt_keys: Arc<JwtKeyPair>,
    pub metrics: Option<SharedMetrics>,
}

impl AppState {
    pub fn new(
        db: sea_orm::DatabaseConnection,
        redis: redis::aio::ConnectionManager,
        settings: Settings,
        jwt_keys: JwtKeyPair,
    ) -> Self {
        Self {
            db,
            redis,
            nats: None,
            event_publisher: None,
            settings,
            storage: None,
            rate_limit_store: Arc::new(RateLimitStore::new()),
            jwt_keys: Arc::new(jwt_keys),
            metrics: None,
        }
    }

    pub fn with_nats(mut self, nats: async_nats::Client) -> Self {
        self.nats = Some(nats.clone());
        self.event_publisher = Some(EventPublisher::new(nats));
        self
    }

    pub fn with_storage(mut self, storage: StorageService) -> Self {
        self.storage = Some(storage);
        self
    }

    pub fn with_metrics(mut self, metrics: SharedMetrics) -> Self {
        self.metrics = Some(metrics);
        self
    }
}

/// Type alias for shared state reference
pub type SharedState = Arc<AppState>;
