use std::net::SocketAddr;

use axum::Router;
use aeroxe_broadband::api::openapi::ApiDoc;
use aeroxe_broadband::app::AppState;
use aeroxe_broadband::common::config::config::Config;
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

    // ── Seed permissions on startup ─────────────────────────
    if let Err(e) = aeroxe_broadband::common::seed::permission_seeder::seed_permissions(&state.db).await {
        tracing::error!(error = %e, "Failed to seed permissions");
    }

    // ── Seed roles with permissions ──────────────────────────
    if let Err(e) = aeroxe_broadband::common::seed::role_seeder::seed_roles(&state.db).await {
        tracing::error!(error = %e, "Failed to seed roles");
    }

    // ── Seed superadmin user ────────────────────────────────
    if let Err(e) = aeroxe_broadband::common::seed::admin_user_seeder::seed_admin_user(&state.db).await {
        tracing::error!(error = %e, "Failed to seed superadmin user");
    }

    let state = std::sync::Arc::new(state);

    // ── Admin API (scoped with JWT + admin role guard via rls_setup::admin_scoped) ──
    let admin_api = Router::new()
        .nest("/api/v1/admin", aeroxe_broadband::api::admin::router::admin_api_router());

    // ── Customer Self-Service API (scoped with JWT + customer role guard via rls_setup::customer_scoped) ──
    let customer_api = Router::new()
        .nest("/api/v1/customer", aeroxe_broadband::api::customer::router::customer_api_router());

    // ── Admin-only module routes (no customer access) ──────
    // All admin routes moved to /api/v1/admin/<resource> via admin API router.
    // Keep this empty — legacy routes will be removed after migration is verified.
    let admin_only_modules = Router::new();
        // This section intentionally left empty.

    // ── Public routes (no JWT required) ──────────────────────
    let public_routes = Router::new()
        .nest("/api/v1/auth", aeroxe_broadband::modules::user::router::user_router::auth_routes())
        .nest("/api/v1/coverage", aeroxe_broadband::modules::coverage::router::coverage_router::coverage_routes())
        .nest("/api/v1/payments/webhook", aeroxe_broadband::modules::payment_gateway::router::payment_gateway_router::payment_webhook_routes())
        .nest("/api/v1/plans", aeroxe_broadband::modules::plan::router::customer_router::customer_routes())
        .nest("/ws", aeroxe_broadband::modules::realtime::router::realtime_router::ws_routes());

    // ── Merge everything ────────────────────────────────────
    let router = Router::new()
        .merge(admin_api)
        .merge(customer_api)
        .merge(admin_only_modules)
        .merge(public_routes)
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .layer(aeroxe_broadband::common::middleware::cors_middleware::build_cors())
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            aeroxe_broadband::common::middleware::rls_layer::inject_pool_middleware,
        ))
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
    #[cfg(unix)]
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
    #[cfg(not(unix))]
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
        tracing::info!("Received SIGINT, initiating graceful shutdown");
        signal_token.cancel();
    });

    // Spawn background jobs with cancellation token
    let bg1 = state.clone();
    let t1 = shutdown_token.clone();
    tokio::spawn(async move {
        aeroxe_broadband::common::jobs::sla_checker::run_sla_checker(bg1, t1).await;
    });
    let bg2 = state.clone();
    let t2 = shutdown_token.clone();
    tokio::spawn(async move {
        aeroxe_broadband::common::jobs::subscription_renewal_reminder::run_subscription_renewal_reminder(bg2, t2).await;
    });
    let bg3 = state.clone();
    let t3 = shutdown_token.clone();
    tokio::spawn(async move {
        aeroxe_broadband::common::jobs::invoice_dunning::run_invoice_dunning(bg3, t3).await;
    });
    let bg4 = state.clone();
    let t4 = shutdown_token.clone();
    tokio::spawn(async move {
        aeroxe_broadband::common::jobs::wallet_expiry_cleanup::run_wallet_expiry_cleanup(bg4, t4).await;
    });
    let bg5 = state.clone();
    let t5 = shutdown_token.clone();
    tokio::spawn(async move {
        aeroxe_broadband::common::jobs::partition_manager::run_partition_manager(bg5, t5).await;
    });
    let bg6 = state.clone();
    let t6 = shutdown_token.clone();
    tokio::spawn(async move {
        aeroxe_broadband::common::jobs::data_cleanup::run_data_cleanup(bg6, t6).await;
    });
    // Spawn notification delivery orchestrator
    let t7 = shutdown_token.clone();
    let db_seaorm = std::sync::Arc::new(state.db.clone());
    tokio::spawn(async move {
        let orchestrator = aeroxe_broadband::modules::notification::delivery::NotificationOrchestrator::new(db_seaorm);
        orchestrator.run(t7).await;
    });
    tracing::info!("Background jobs spawned: SLA checker, renewal reminders, dunning, wallet expiry, partition manager, data cleanup, notification orchestrator");

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
