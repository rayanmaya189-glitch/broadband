use std::net::SocketAddr;

use axum::Router;
use aeraxe_backend::api::openapi::ApiDoc;
use aeraxe_backend::app::AppState;
use aeraxe_backend::common::config::config::Config;
use tokio_util::sync::CancellationToken;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

fn print_routes() {
    let doc = ApiDoc::openapi();

    println!("\n================ API ROUTES ================\n");

    for (path, item) in doc.paths.paths {
        if item.get.is_some() {
            println!("{:<7} {}", "GET", path);
        }
        if item.post.is_some() {
            println!("{:<7} {}", "POST", path);
        }
        if item.put.is_some() {
            println!("{:<7} {}", "PUT", path);
        }
        if item.delete.is_some() {
            println!("{:<7} {}", "DELETE", path);
        }
        if item.patch.is_some() {
            println!("{:<7} {}", "PATCH", path);
        }
        if item.options.is_some() {
            println!("{:<7} {}", "OPTIONS", path);
        }
        if item.head.is_some() {
            println!("{:<7} {}", "HEAD", path);
        }
    }

    println!("\n============================================\n");
}

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
        .nest("/api/v1/billing", aeraxe_backend::modules::billing::router::billing_router::billing_routes())
        .nest("/api/v1/devices", aeraxe_backend::modules::device::router::device_router::device_routes())
        .nest("/api/v1/bandwidth", aeraxe_backend::modules::bandwidth::router::bandwidth_router::bandwidth_routes())
        .nest("/api/v1/network", aeraxe_backend::modules::network::router::network_router::network_routes())
        .nest("/api/v1/coverage", aeraxe_backend::modules::coverage::router::coverage_router::coverage_routes())
        .nest("/api/v1/installations", aeraxe_backend::modules::installation::router::installation_router::installation_routes())
        .nest("/api/v1/inventory", aeraxe_backend::modules::inventory::router::inventory_router::inventory_routes())
        .nest("/api/v1/referrals", aeraxe_backend::modules::referral::router::referral_router::referral_routes())
        .nest("/api/v1/notifications", aeraxe_backend::modules::notification::router::notification_router::notification_routes())
        .nest("/api/v1/events", aeraxe_backend::modules::event::router::event_router::event_routes())
        .nest("/api/v1/documents", aeraxe_backend::modules::document::router::document_router::document_routes())
        .nest("/api/v1/accounting", aeraxe_backend::modules::accounting::router::accounting_router::accounting_routes())
        .nest("/api/v1/payments", aeraxe_backend::modules::payment_gateway::router::payment_gateway_router::payment_gateway_routes())
        .nest("/api/v1/payments/webhook", aeraxe_backend::modules::payment_gateway::router::payment_gateway_router::payment_webhook_routes())
        .nest("/api/v1/discovery", aeraxe_backend::modules::discovery::router::discovery_router::discovery_routes())
        .nest("/api/v1/realtime", aeraxe_backend::modules::realtime::router::realtime_router::realtime_routes())
        .nest("/ws", aeraxe_backend::modules::realtime::router::realtime_router::ws_routes())
        .nest("/api/v1/audit", aeraxe_backend::modules::audit::router::audit_router::audit_routes())
        .nest("/api/v1/audit/entity-history", aeraxe_backend::modules::audit::router::entity_history_router::entity_history_routes())
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .layer(aeraxe_backend::common::middleware::cors_middleware::build_cors())
        .with_state(state.clone());

    let addr: SocketAddr = format!("{}:{}", config.server_host, config.server_port)
        .parse()
        .expect("Invalid server address");

    print_routes();

    tracing::info!(addr = %addr, "Server listening");

    // ── Graceful shutdown via CancellationToken ──────────────
    let shutdown_token = CancellationToken::new();

    // Spawn signal handler for SIGINT (Ctrl+C) and SIGTERM
    let signal_token = shutdown_token.clone();
    tokio::spawn(async move {
        let ctrl_c = tokio::signal::ctrl_c();
        let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler");

        tokio::select! {
            _ = ctrl_c => {
                tracing::info!("Received SIGINT, initiating graceful shutdown");
            }
            _ = sigterm.recv() => {
                tracing::info!("Received SIGTERM, initiating graceful shutdown");
            }
        }
        signal_token.cancel();
    });

    // Spawn background jobs with cancellation token
    let bg1 = state.clone();
    let t1 = shutdown_token.clone();
    tokio::spawn(async move {
        aeraxe_backend::common::jobs::sla_checker::run_sla_checker(bg1, t1).await;
    });
    let bg2 = state.clone();
    let t2 = shutdown_token.clone();
    tokio::spawn(async move {
        aeraxe_backend::common::jobs::subscription_renewal_reminder::run_subscription_renewal_reminder(bg2, t2).await;
    });
    let bg3 = state.clone();
    let t3 = shutdown_token.clone();
    tokio::spawn(async move {
        aeraxe_backend::common::jobs::invoice_dunning::run_invoice_dunning(bg3, t3).await;
    });
    let bg4 = state.clone();
    let t4 = shutdown_token.clone();
    tokio::spawn(async move {
        aeraxe_backend::common::jobs::wallet_expiry_cleanup::run_wallet_expiry_cleanup(bg4, t4).await;
    });
    tracing::info!("Background jobs spawned: SLA checker, renewal reminders, dunning, wallet expiry");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    // Graceful HTTP server shutdown
    let server_token = shutdown_token.clone();
    axum::serve(listener, router)
        .with_graceful_shutdown(async move {
            server_token.cancelled().await;
            tracing::info!("HTTP server shutting down gracefully");
        })
        .await
        .expect("Server failed");
}
