//! AeroXe Broadband ISP Platform - Backend API
//!
//! A modular monolith built with Rust, Axum, SeaORM, PostgreSQL, Redis, and NATS.

use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use aeroxe_backend::config::settings::Settings;
use aeroxe_backend::infrastructure::database::create_database_pool;
use aeroxe_backend::infrastructure::cache::create_redis_pool;
use aeroxe_backend::shared::app_state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "aeroxe_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    tracing::info!("Starting AeroXe Backend v{}", env!("CARGO_PKG_VERSION"));

    // Load configuration
    let settings = Settings::from_env()?;
    let addr: SocketAddr = settings.server_addr.parse()?;
    tracing::info!("Server listening on {}", addr);

    // Create database pool
    let db = create_database_pool(&settings.database_url, settings.db_max_connections).await?;
    tracing::info!("Database pool created");

    // Create Redis pool
    let redis = create_redis_pool(&settings.redis_url).await?;
    tracing::info!("Redis pool created");

    // Connect to NATS (optional - gracefully handle if unavailable)
    let nats_client = match aeroxe_backend::infrastructure::messaging::nats_client::connect_nats(&settings.nats_url).await {
        Ok(client) => {
            // Set up JetStream
            let js_config = aeroxe_backend::infrastructure::messaging::nats_client::JetStreamConfig::default();
            if let Err(e) = aeroxe_backend::infrastructure::messaging::nats_client::ensure_jetstream_stream(&client, &js_config).await {
                tracing::warn!(error = %e, "Failed to set up JetStream, continuing without NATS");
                None
            } else {
                tracing::info!("NATS JetStream ready");
                Some(client)
            }
        }
        Err(e) => {
            tracing::warn!(error = %e, "Failed to connect to NATS, continuing without event publishing");
            None
        }
    };

    // Build shared state
    let mut app_state = AppState::new(db, redis, settings.clone());
    if let Some(client) = nats_client {
        app_state = app_state.with_nats(client);
    }
    // Initialize MinIO/S3 storage (optional - gracefully handle if unavailable)
    match aeroxe_backend::infrastructure::storage::StorageService::from_env().await {
        Ok(storage) => {
            tracing::info!("MinIO/S3 storage service initialized");
            app_state = app_state.with_storage(storage);
        }
        Err(e) => {
            tracing::warn!(error = %e, "Failed to initialize storage service, file uploads will be unavailable");
        }
    }
    let state = Arc::new(app_state);

    // Build CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build application router
    let app = Router::new()
        .nest("/api/v1", aeroxe_backend::routes::v1_routes())
        .merge(aeroxe_backend::routes::health_routes())
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state.clone());

    // Start outbox worker in background (if NATS is available)
    if let Some(nats_client) = state.nats.clone() {
        let outbox_db = state.db.clone();
        let outbox_publisher = aeroxe_backend::infrastructure::messaging::EventPublisher::new(nats_client);
        let outbox_worker = aeroxe_backend::workers::outbox_worker::OutboxWorker::new(
            std::sync::Arc::new(outbox_db),
            outbox_publisher,
        );
        tokio::spawn(async move {
            outbox_worker.run().await;
        });
        tracing::info!("Outbox worker started");
    }

    // Start server
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Server ready to accept connections on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
