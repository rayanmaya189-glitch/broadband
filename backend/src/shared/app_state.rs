use std::sync::Arc;

use crate::config::settings::Settings;

/// Shared application state available to all handlers.
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
    pub redis: redis::aio::ConnectionManager,
    pub settings: Settings,
}

impl AppState {
    pub fn new(
        db: sea_orm::DatabaseConnection,
        redis: redis::aio::ConnectionManager,
        settings: Settings,
    ) -> Self {
        Self {
            db,
            redis,
            settings,
        }
    }
}

/// Type alias for shared state reference
pub type SharedState = Arc<AppState>;
