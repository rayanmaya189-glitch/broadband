use std::net::SocketAddr;

use aeraxe_backend::app::AppState;
use aeraxe_backend::config::Config;
use aeraxe_backend::interfaces::http::create_router;

#[tokio::main]
async fn main() {
    // ── Load configuration ──────────────────────────────────
    let config = Config::get().clone();

    // ── Initialize tracing / logging ────────────────────────
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(true)
        .with_thread_ids(true)
        .json()
        .init();

    tracing::info!(
        host = %config.server_host,
        port = config.server_port,
        db = %config.database_url_redacted(),
        "Starting AeroXe backend"
    );

    // ── Build application state (connects DB, Redis, NATS) ─
    let state = match AppState::new(config.clone()).await {
        Ok(state) => {
            tracing::info!("All services connected successfully");
            state
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to connect to backing services");
            std::process::exit(1);
        }
    };

    let state = std::sync::Arc::new(state);

    // ── Build and start Axum server ────────────────────────
    let router = create_router(state);

    let addr: SocketAddr = format!("{}:{}", config.server_host, config.server_port)
        .parse()
        .expect("Invalid server address");

    tracing::info!(addr = %addr, "Server listening");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, router)
        .await
        .expect("Server failed");
}
