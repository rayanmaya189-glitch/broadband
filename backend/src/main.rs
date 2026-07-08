use std::net::SocketAddr;

use axum::Router;
use aeraxe_backend::app::AppState;
use aeraxe_backend::common::config::config::Config;

#[tokio::main]
async fn main() {
    let config = Config::get().clone();

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

    let router = Router::new()
        .nest("/api/v1/auth", aeraxe_backend::modules::user::router::user_router::auth_routes())
        .nest("/api/v1/users", aeraxe_backend::modules::user::router::user_router::users_routes())
        .nest("/api/v1/roles", aeraxe_backend::modules::role::router::role_router::roles_routes())
        .nest("/api/v1/permissions", aeraxe_backend::modules::permission::router::permission_router::permissions_routes())
        .nest("/api/v1/branches", aeraxe_backend::modules::branch::router::branch_router::branches_routes())
        .nest("/api/v1/customers", aeraxe_backend::modules::customer::router::customer_router::customers_routes())
        .nest("/api/v1/plans", aeraxe_backend::modules::plan::router::plan_router::plans_routes())
        .nest("/api/v1/subscriptions", aeraxe_backend::modules::subscription::router::subscription_router::subscriptions_routes())
        .nest("/api/v1/tickets", aeraxe_backend::modules::ticket::router::ticket_router::ticket_routes())
        .nest("/api/v1/leads", aeraxe_backend::modules::lead::router::lead_router::lead_routes())
        .layer(aeraxe_backend::common::middleware::cors_middleware::build_cors())
        .with_state(state);

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
