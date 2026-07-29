//! AeroXe Broadband ISP Platform - Backend API
//!
//! A modular monolith built with Rust, Axum, SeaORM, PostgreSQL, Redis, and NATS.

use std::net::SocketAddr;
use std::panic::AssertUnwindSafe;
use std::sync::Arc;

use axum::Router;
use chrono::Datelike;
use futures::FutureExt;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use aeroxe_backend::config::settings::Settings;
use aeroxe_backend::infrastructure::cache::create_redis_pool;
use aeroxe_backend::infrastructure::database::create_database_pool;
use aeroxe_backend::infrastructure::metrics::create_metrics;
use aeroxe_backend::shared::app_state::AppState;
use aeroxe_backend::shared::utils::jwt_keys::init_jwt_keys;

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
    let db = create_database_pool(
        &settings.database_url,
        settings.db_max_connections,
        settings.db_min_connections,
        settings.db_connect_timeout_secs,
        settings.db_idle_timeout_secs,
    ).await?;
    tracing::info!("Database pool created");

    // Create Redis pool
    let redis = create_redis_pool(&settings.redis_url).await?;
    tracing::info!("Redis pool created");

    // Connect to NATS (optional - gracefully handle if unavailable)
    let nats_client = match aeroxe_backend::infrastructure::messaging::nats_client::connect_nats(
        &settings.nats_url,
    )
    .await
    {
        Ok(client) => {
            // Set up JetStream
            let js_config =
                aeroxe_backend::infrastructure::messaging::nats_client::JetStreamConfig::default();
            if let Err(e) =
                aeroxe_backend::infrastructure::messaging::nats_client::ensure_jetstream_stream(
                    &client, &js_config,
                )
                .await
            {
                tracing::warn!(error = %e, "Failed to set up JetStream, continuing without event publishing");
                None
            } else {
                tracing::info!("NATS JetStream ready");
                Some(client)
            }
        }
        Err(e) => {
            tracing::error!(
                error = %e,
                nats_url = %settings.nats_url,
                "CRITICAL: Failed to connect to NATS — event publishing, outbox delivery, and cross-module communication DISABLED. Retrying in background."
            );
            // Spawn background NATS reconnection task
            let nats_url = settings.nats_url.clone();
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
                loop {
                    interval.tick().await;
                    match aeroxe_backend::infrastructure::messaging::nats_client::connect_nats(&nats_url).await {
                        Ok(_client) => {
                            tracing::info!("NATS reconnection successful");
                            // Note: can't easily hot-swap into AppState, but at least we know it's back
                            break;
                        }
                        Err(e) => {
                            tracing::warn!(error = %e, "NATS reconnection attempt failed, retrying in 30s");
                        }
                    }
                }
            });
            None
        }
    };

    // Initialize JWT RS256 key pair
    let jwt_keys = init_jwt_keys(&settings.jwt_private_key_pem, &settings.jwt_public_key_pem)?;
    tracing::info!("JWT RS256 keys ready");

    // Initialize global JWT keys for branch_scope middleware
    aeroxe_backend::shared::middleware::branch_scope::init_jwt_keys_global(jwt_keys.clone());

    // Build shared state
    let mut app_state = AppState::new(db, redis, settings.clone(), jwt_keys.clone());
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
    // Initialize Prometheus metrics
    let metrics = create_metrics();
    app_state = app_state.with_metrics(metrics.clone());
    let state = Arc::new(app_state);

    // Start JWT key rotation background worker (checks daily)
    state
        .jwt_rotation_manager
        .clone()
        .start_background_rotation();
    tracing::info!(
        rotation_days = state.settings.jwt_key_rotation_days,
        "JWT key rotation worker started"
    );

    // Build CORS layer - production lockdown
    // In production, restrict origins to configured list; in development, allow any
    let cors = if settings.app_env == "production" {
        let origins: Vec<_> = settings
            .cors_origins
            .iter()
            .filter_map(|o| o.parse().ok())
            .collect();
        CorsLayer::new()
            .allow_origin(origins)
            .allow_methods([
                axum::http::Method::GET,
                axum::http::Method::POST,
                axum::http::Method::PUT,
                axum::http::Method::PATCH,
                axum::http::Method::DELETE,
                axum::http::Method::OPTIONS,
            ])
            .allow_headers([
                axum::http::header::AUTHORIZATION,
                axum::http::header::CONTENT_TYPE,
                axum::http::header::ACCEPT,
                "X-Request-ID".parse().unwrap(),
                "X-Idempotency-Key".parse().unwrap(),
            ])
            .max_age(std::time::Duration::from_secs(3600))
    } else {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
    };

    // Clone rate_limit_store for the middleware closure
    let rate_limit_store = state.rate_limit_store.clone();

    // Build application router with middlewares
    // Layer order matters: outermost layer runs first on request, last on response
    // IMPORTANT: CORS must be outermost to handle preflight OPTIONS before other layers
    let app = Router::new()
        .nest("/api/v1", aeroxe_backend::routes::v1_routes())
        .merge(aeroxe_backend::routes::health_routes())
        .merge(aeroxe_backend::infrastructure::openapi::swagger_routes())
        // 0. Request ID (outermost — generates/propagates X-Request-ID before anything else)
        .layer(axum::middleware::from_fn(
            aeroxe_backend::shared::middleware::request_id::request_id_middleware,
        ))
        // 1. Request body size limit (10 MB default)
        .layer(tower_http::limit::RequestBodyLimitLayer::new(
            10 * 1024 * 1024,
        ))
        // 2. SSRF protection (blocks private IPs in request bodies)
        .layer(axum::middleware::from_fn(
            aeroxe_backend::shared::middleware::ssrf::ssrf_protection_middleware,
        ))
        // 3. Security headers (adds headers to every response)
        .layer(axum::middleware::from_fn(
            aeroxe_backend::shared::middleware::security_headers::security_headers_middleware,
        ))
        // 4. Audit middleware (captures timing, logs after response, persists to DB)
        .layer(axum::middleware::from_fn({
            let db = std::sync::Arc::new(state.db.clone());
            move |req: axum::http::Request<axum::body::Body>, next: axum::middleware::Next| {
                let db = db.clone();
                async move {
                    let mut req = req;
                    req.extensions_mut().insert(db);
                    aeroxe_backend::shared::middleware::audit::audit_middleware(req, next)
                        .await
                }
            }
        }))
        // 5. Rate limiting (with injected store)
        .layer(axum::middleware::from_fn({
            let store = rate_limit_store.clone();
            move |req: axum::http::Request<axum::body::Body>, next: axum::middleware::Next| {
                let store = store.clone();
                async move {
                    let mut req = req;
                    req.extensions_mut().insert(store);
                    aeroxe_backend::shared::middleware::rate_limit::rate_limit_middleware(req, next)
                        .await
                }
            }
        }))
        // 6. Branch scope (extracts JWT, sets BranchScope in extensions)
        .layer(axum::middleware::from_fn(
            aeroxe_backend::shared::middleware::branch_scope::branch_scope_middleware,
        ))
        .layer(TraceLayer::new_for_http())
        // 7. CORS (outermost for preflight handling)
        .layer(cors)
        .with_state(state.clone());

    // --- Graceful shutdown setup ---
    // Create a shutdown signal broadcast channel (capacity 32 for 8+ workers + subscribers)
    let (shutdown_tx, _) = tokio::sync::broadcast::channel::<()>(32);

    // Start outbox worker and NATS subscribers (if NATS is available)
    if let Some(nats_client) = state.nats.clone() {
        // Start outbox worker
        let outbox_db = state.db.clone();
        let outbox_client = nats_client.clone();
        let outbox_publisher =
            aeroxe_backend::infrastructure::messaging::EventPublisher::new(outbox_client);
        let outbox_worker = aeroxe_backend::workers::outbox_worker::OutboxWorker::new(
            std::sync::Arc::new(outbox_db),
            outbox_publisher,
        )
        .with_poll_interval(state.settings.worker_outbox_poll_interval_secs);
        let outbox_worker = if let Some(ref metrics) = state.metrics {
            outbox_worker.with_metrics(metrics.clone())
        } else {
            outbox_worker
        };
        let mut outbox_rx = shutdown_tx.subscribe();
        tokio::spawn(async move {
            tokio::select! {
                _ = outbox_worker.run() => {},
                _ = outbox_rx.recv() => {
                    tracing::info!("Outbox worker shutting down");
                }
            }
        });
        tracing::info!("Outbox worker started");

        // Start NATS event subscribers for cross-module communication
        let sub_db = Arc::new(state.db.clone());
        let mut sub_rx = shutdown_tx.subscribe();
        let nats_clone = nats_client.clone();
        tokio::spawn(async move {
            tokio::select! {
                result = aeroxe_backend::infrastructure::messaging::subscribers::start_subscribers(
                    nats_clone, sub_db,
                ) => {
                    if let Err(e) = result {
                        tracing::error!(error = %e, "NATS subscribers failed");
                    }
                }
                _ = sub_rx.recv() => {
                    tracing::info!("NATS subscribers shutting down");
                }
            }
        });
        tracing::info!("NATS event subscribers started");
    }

    // Start background workers with graceful shutdown
    {
        let worker_db = state.db.clone();
        let worker_metrics = state.metrics.clone();
        let worker_settings = state.settings.clone();

        // Billing worker - runs every 5 minutes
        {
            let db = worker_db.clone();
            let wm = worker_metrics.clone();
            let poll = worker_settings.worker_billing_poll_interval_secs;
            let worker = aeroxe_backend::workers::billing_worker::BillingWorker::new(db);
            let mut rx = shutdown_tx.subscribe();
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(poll));
                loop {
                    tokio::select! {
                        _ = interval.tick() => {
                            let result = AssertUnwindSafe(worker.run_cycle())
                                .catch_unwind()
                                .await;
                            match result {
                                Ok(Ok(())) => {
                                    if let Some(ref m) = wm { m.read().await.worker_cycles_total.with_label_values(&["billing"]).inc(); }
                                },
                                Ok(Err(e)) => {
                                    if let Some(ref m) = wm { m.read().await.worker_errors_total.with_label_values(&["billing"]).inc(); }
                                    tracing::error!(error = %e, "Billing worker cycle failed");
                                },
                                Err(_) => {
                                    if let Some(ref m) = wm { m.read().await.worker_errors_total.with_label_values(&["billing"]).inc(); }
                                    tracing::error!("Billing worker PANICKED — will restart next cycle");
                                },
                            }
                        }
                        _ = rx.recv() => {
                            tracing::info!("Billing worker shutting down");
                            break;
                        }
                    }
                }
            });
            tracing::info!("Billing worker started (every 5 minutes)");
        }

        // Notification worker - runs every 30 seconds
        {
            let db = worker_db.clone();
            let wm = worker_metrics.clone();
            let poll = worker_settings.worker_notification_poll_interval_secs;
            let worker = aeroxe_backend::workers::notification_worker::NotificationWorker::new(db);
            let mut rx = shutdown_tx.subscribe();
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(poll));
                loop {
                    tokio::select! {
                        _ = interval.tick() => {
                            let result = AssertUnwindSafe(worker.run_cycle())
                                .catch_unwind()
                                .await;
                            match result {
                                Ok(Ok(())) => {
                                    if let Some(ref m) = wm { m.read().await.worker_cycles_total.with_label_values(&["notification"]).inc(); }
                                },
                                Ok(Err(e)) => {
                                    if let Some(ref m) = wm { m.read().await.worker_errors_total.with_label_values(&["notification"]).inc(); }
                                    tracing::error!(error = %e, "Notification worker cycle failed");
                                },
                                Err(_) => {
                                    if let Some(ref m) = wm { m.read().await.worker_errors_total.with_label_values(&["notification"]).inc(); }
                                    tracing::error!("Notification worker PANICKED — will restart next cycle");
                                },
                            }
                        }
                        _ = rx.recv() => {
                            tracing::info!("Notification worker shutting down");
                            break;
                        }
                    }
                }
            });
            tracing::info!("Notification worker started (every 30 seconds)");
        }

        // Device sync worker - runs every 2 minutes
        {
            let db = worker_db.clone();
            let wm = worker_metrics.clone();
            let poll = worker_settings.worker_device_sync_poll_interval_secs;
            let worker = aeroxe_backend::workers::device_sync_worker::DeviceSyncWorker::new(db);
            let mut rx = shutdown_tx.subscribe();
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(poll));
                loop {
                    tokio::select! {
                        _ = interval.tick() => {
                            let result = AssertUnwindSafe(worker.run_cycle())
                                .catch_unwind()
                                .await;
                            match result {
                                Ok(Ok(())) => {
                                    if let Some(ref m) = wm { m.read().await.worker_cycles_total.with_label_values(&["device_sync"]).inc(); }
                                },
                                Ok(Err(e)) => {
                                    if let Some(ref m) = wm { m.read().await.worker_errors_total.with_label_values(&["device_sync"]).inc(); }
                                    tracing::error!(error = %e, "Device sync worker cycle failed");
                                },
                                Err(_) => {
                                    if let Some(ref m) = wm { m.read().await.worker_errors_total.with_label_values(&["device_sync"]).inc(); }
                                    tracing::error!("Device sync worker PANICKED — will restart next cycle");
                                },
                            }
                        }
                        _ = rx.recv() => {
                            tracing::info!("Device sync worker shutting down");
                            break;
                        }
                    }
                }
            });
            tracing::info!("Device sync worker started (every 2 minutes)");
        }

        // Bandwidth worker - runs every minute
        {
            let db = worker_db.clone();
            let wm = worker_metrics.clone();
            let poll = worker_settings.worker_bandwidth_poll_interval_secs;
            let worker = aeroxe_backend::workers::bandwidth_worker::BandwidthWorker::new(db);
            let mut rx = shutdown_tx.subscribe();
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(poll));
                loop {
                    tokio::select! {
                        _ = interval.tick() => {
                            let result = AssertUnwindSafe(worker.run_cycle())
                                .catch_unwind()
                                .await;
                            match result {
                                Ok(Ok(())) => {
                                    if let Some(ref m) = wm { m.read().await.worker_cycles_total.with_label_values(&["bandwidth"]).inc(); }
                                },
                                Ok(Err(e)) => {
                                    if let Some(ref m) = wm { m.read().await.worker_errors_total.with_label_values(&["bandwidth"]).inc(); }
                                    tracing::error!(error = %e, "Bandwidth worker cycle failed");
                                },
                                Err(_) => {
                                    if let Some(ref m) = wm { m.read().await.worker_errors_total.with_label_values(&["bandwidth"]).inc(); }
                                    tracing::error!("Bandwidth worker PANICKED — will restart next cycle");
                                },
                            }
                        }
                        _ = rx.recv() => {
                            tracing::info!("Bandwidth worker shutting down");
                            break;
                        }
                    }
                }
            });
            tracing::info!("Bandwidth worker started (every minute)");
        }

        // RADIUS accounting worker - runs every 5 minutes
        {
            let db = worker_db.clone();
            let wm = worker_metrics.clone();
            let poll = worker_settings.worker_radius_poll_interval_secs;
            let worker = aeroxe_backend::workers::radius_accounting_worker::RadiusAccountingWorker::new(db);
            let mut rx = shutdown_tx.subscribe();
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(poll));
                loop {
                    tokio::select! {
                        _ = interval.tick() => {
                            let result = AssertUnwindSafe(worker.run_cycle())
                                .catch_unwind()
                                .await;
                            match result {
                                Ok(Ok(())) => {
                                    if let Some(ref m) = wm { m.read().await.worker_cycles_total.with_label_values(&["radius"]).inc(); }
                                },
                                Ok(Err(e)) => {
                                    if let Some(ref m) = wm { m.read().await.worker_errors_total.with_label_values(&["radius"]).inc(); }
                                    tracing::error!(error = %e, "RADIUS accounting worker cycle failed");
                                },
                                Err(_) => {
                                    if let Some(ref m) = wm { m.read().await.worker_errors_total.with_label_values(&["radius"]).inc(); }
                                    tracing::error!("RADIUS accounting worker PANICKED — will restart next cycle");
                                },
                            }
                        }
                        _ = rx.recv() => {
                            tracing::info!("RADIUS accounting worker shutting down");
                            break;
                        }
                    }
                }
            });
            tracing::info!("RADIUS accounting worker started (every 5 minutes)");
        }

        // Scheduler worker - runs every 30 seconds
        {
            let db = worker_db.clone();
            let wm = worker_metrics.clone();
            let poll = worker_settings.worker_scheduler_poll_interval_secs;
            let worker = aeroxe_backend::workers::scheduler_worker::SchedulerWorker::new(db);
            let mut rx = shutdown_tx.subscribe();
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(poll));
                loop {
                    tokio::select! {
                        _ = interval.tick() => {
                            let result = AssertUnwindSafe(worker.run_cycle())
                                .catch_unwind()
                                .await;
                            match result {
                                Ok(Ok(())) => {
                                    if let Some(ref m) = wm { m.read().await.worker_cycles_total.with_label_values(&["scheduler"]).inc(); }
                                },
                                Ok(Err(e)) => {
                                    if let Some(ref m) = wm { m.read().await.worker_errors_total.with_label_values(&["scheduler"]).inc(); }
                                    tracing::error!(error = %e, "Scheduler worker cycle failed");
                                },
                                Err(_) => {
                                    if let Some(ref m) = wm { m.read().await.worker_errors_total.with_label_values(&["scheduler"]).inc(); }
                                    tracing::error!("Scheduler worker PANICKED — will restart next cycle");
                                },
                            }
                        }
                        _ = rx.recv() => {
                            tracing::info!("Scheduler worker shutting down");
                            break;
                        }
                    }
                }
            });
            tracing::info!("Scheduler worker started (every 30 seconds)");
        }

        // Outbox cleanup worker - runs every hour
        {
            let db = worker_db.clone();
            let poll = worker_settings.worker_outbox_cleanup_poll_interval_secs;
            let mut rx = shutdown_tx.subscribe();
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(poll));
                loop {
                    tokio::select! {
                        _ = interval.tick() => {
                            if let Err(e) =
                                aeroxe_backend::infrastructure::messaging::outbox::cleanup_published_events(
                                    &db, 24,
                                )
                                .await
                            {
                                tracing::error!(error = %e, "Outbox cleanup failed");
                            }
                        }
                        _ = rx.recv() => {
                            tracing::info!("Outbox cleanup worker shutting down");
                            break;
                        }
                    }
                }
            });
            tracing::info!("Outbox cleanup worker started (every hour)");
        }

        // Monitoring worker - runs every 2 minutes
        {
            let db = worker_db.clone();
            let wm = worker_metrics.clone();
            let poll = worker_settings.worker_monitoring_poll_interval_secs;
            let worker = aeroxe_backend::workers::monitoring_worker::MonitoringWorker::new(db);
            let mut rx = shutdown_tx.subscribe();
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(poll));
                loop {
                    tokio::select! {
                        _ = interval.tick() => {
                            let result = AssertUnwindSafe(worker.run_cycle())
                                .catch_unwind()
                                .await;
                            match result {
                                Ok(Ok(())) => {
                                    if let Some(ref m) = wm { m.read().await.worker_cycles_total.with_label_values(&["monitoring"]).inc(); }
                                },
                                Ok(Err(e)) => {
                                    if let Some(ref m) = wm { m.read().await.worker_errors_total.with_label_values(&["monitoring"]).inc(); }
                                    tracing::error!(error = %e, "Monitoring worker cycle failed");
                                },
                                Err(_) => {
                                    if let Some(ref m) = wm { m.read().await.worker_errors_total.with_label_values(&["monitoring"]).inc(); }
                                    tracing::error!("Monitoring worker PANICKED — will restart next cycle");
                                },
                            }
                        }
                        _ = rx.recv() => {
                            tracing::info!("Monitoring worker shutting down");
                            break;
                        }
                    }
                }
            });
            tracing::info!("Monitoring worker started (every 2 minutes)");
        }

        // Partition worker - runs on 1st of each month (via scheduler, but also as a standalone fallback)
        {
            let db = worker_db.clone();
            let wm = worker_metrics.clone();
            let poll = worker_settings.worker_partition_poll_interval_secs;
            let mut rx = shutdown_tx.subscribe();
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(poll));
                loop {
                    tokio::select! {
                        _ = interval.tick() => {
                            let now = chrono::Utc::now();
                            // Only run on 1st of the month
                            if now.date_naive().day() == 1 {
                                let result = AssertUnwindSafe(async {
                                    aeroxe_backend::workers::partition_worker::create_monthly_partitions(&db).await
                                }).catch_unwind().await;
                                match result {
                                    Ok(Ok(())) => {
                                        if let Some(ref m) = wm { m.read().await.worker_cycles_total.with_label_values(&["partition"]).inc(); }
                                    },
                                    Ok(Err(e)) => {
                                        if let Some(ref m) = wm { m.read().await.worker_errors_total.with_label_values(&["partition"]).inc(); }
                                        tracing::error!(error = %e, "Partition creation failed");
                                    },
                                    Err(_) => {
                                        if let Some(ref m) = wm { m.read().await.worker_errors_total.with_label_values(&["partition"]).inc(); }
                                        tracing::error!("Partition worker PANICKED");
                                    }
                                }
                                let result2 = AssertUnwindSafe(async {
                                    aeroxe_backend::workers::partition_worker::run_cleanup(&db).await
                                }).catch_unwind().await;
                                match result2 {
                                    Ok(Ok(_)) => {},
                                    Ok(Err(e)) => tracing::error!(error = %e, "Partition cleanup failed"),
                                    Err(_) => tracing::error!("Partition cleanup PANICKED"),
                                }
                            }
                        }
                        _ = rx.recv() => {
                            tracing::info!("Partition worker shutting down");
                            break;
                        }
                    }
                }
            });
            tracing::info!("Partition worker started (monthly on 1st)");
        }
    }

    // Start server with graceful shutdown
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Server ready to accept connections on {}", addr);

    // Spawn the server in a separate task so we can handle shutdown signals
    let server_handle = tokio::spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .expect("Server failed");
    });

    // Wait for the server task to complete, with 30-second drain timeout
    let drain_timeout = std::time::Duration::from_secs(30);
    match tokio::time::timeout(drain_timeout, server_handle).await {
        Ok(Ok(())) => {
            tracing::info!("Server drained cleanly");
        }
        Ok(Err(e)) => {
            tracing::error!(error = %e, "Server task panicked during shutdown");
        }
        Err(_) => {
            tracing::warn!(
                drain_secs = 30,
                "Server drain timed out — forcing shutdown"
            );
        }
    }

    // Broadcast shutdown to all workers
    let _ = shutdown_tx.send(());

    tracing::info!("AeroXe Backend shutdown complete");
    Ok(())
}

/// Wait for a shutdown signal (SIGINT on Unix, Ctrl+C on all platforms)
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Received SIGINT (Ctrl+C), initiating graceful shutdown");
        }
        _ = terminate => {
            tracing::info!("Received SIGTERM, initiating graceful shutdown");
        }
    }
}
