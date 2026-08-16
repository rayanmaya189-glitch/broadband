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
    settings.validate_environment()?;
    let addr: SocketAddr = settings.server_addr.parse()?;
    tracing::info!("Server listening on {}", addr);

    // Boot-time integration health summary. External adapters read their
    // configuration directly from environment variables, so a missing secret
    // is only discoverable at first use unless we check now. Warn loudly at
    // startup instead of failing silently on the first invoice/OTP/device sync.
    log_integration_health();

    // Create database pool
    let db = create_database_pool(
        &settings.database_url,
        settings.db_max_connections,
        settings.db_min_connections,
        settings.db_connect_timeout_secs,
        settings.db_idle_timeout_secs,
    )
    .await?;
    tracing::info!("Database pool created");

    // Create Redis pool
    let redis = create_redis_pool(&settings.redis_url).await?;
    tracing::info!("Redis pool created");

    // Initialize JWT RS256 key pair
    let jwt_keys = init_jwt_keys(&settings.jwt_private_key_pem, &settings.jwt_public_key_pem)?;
    tracing::info!("JWT RS256 keys ready");
    // Shared rotating key store: signing/verification and the rotation manager
    // all reference the same store, so a rotation actually takes effect while
    // the previous key stays valid for the token grace window.
    let jwt_keys = std::sync::Arc::new(aeroxe_backend::shared::utils::jwt_keys::JwtKeys::new(
        jwt_keys,
    ));

    // Initialize global JWT keys for branch_scope middleware
    aeroxe_backend::shared::middleware::branch_scope::init_jwt_keys_global(jwt_keys.clone());

    // Build shared state. The NATS connection is left `None` here — the NATS
    // supervisor (started below) connects with retry/backoff and publishes the
    // live client into `state.nats` once available.
    let mut app_state = AppState::new(db, redis, settings.clone(), jwt_keys);
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
                    aeroxe_backend::shared::middleware::audit::audit_middleware(req, next).await
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
        // 8. Security alerting (§28) — outermost so it observes every response status
        .layer(
            aeroxe_backend::shared::middleware::security_alerts::SecurityAlertLayer::new(
                state.clone(),
            ),
        )
        .with_state(state.clone());

    // --- Graceful shutdown setup ---
    // Create a shutdown signal broadcast channel (capacity 32 for 8+ workers + subscribers)
    let (shutdown_tx, _) = tokio::sync::broadcast::channel::<()>(32);

    // Start the NATS supervisor. It owns the entire NATS lifecycle: connect
    // with retry/backoff (never giving up), publish the live client into
    // AppState, start the outbox worker + event subscribers, and reconnect
    // everything if the connection drops. This removes the previous behavior
    // where a NATS outage at boot permanently disabled outbox delivery and
    // cross-module subscribers.
    {
        let state_supervisor = state.clone();
        let shutdown_rx = shutdown_tx.subscribe();
        tokio::spawn(async move {
            nats_supervisor(state_supervisor, shutdown_rx).await;
        });
        tracing::info!("NATS supervisor started");
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
            let worker =
                aeroxe_backend::workers::radius_accounting_worker::RadiusAccountingWorker::new(db);
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
        // into_make_service_with_connect_info exposes the real peer IP to
        // middleware (used by rate limiting instead of trusting X-Forwarded-For).
        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
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
            tracing::warn!(drain_secs = 30, "Server drain timed out — forcing shutdown");
        }
    }

    // Broadcast shutdown to all workers
    let _ = shutdown_tx.send(());

    tracing::info!("AeroXe Backend shutdown complete");
    Ok(())
}

/// Owns the full NATS lifecycle: connect with retry/backoff (never giving up),
/// publish the live client into AppState, start the outbox worker and event
/// subscribers, and reconnect everything if the connection drops.
///
/// Previously, a NATS outage at boot permanently disabled outbox delivery and
/// cross-module subscribers because the outbox worker and subscribers were only
/// started when the initial boot connection succeeded. With this supervisor the
/// outbox drains automatically as soon as NATS is reachable, and a dropped
/// connection is detected (the subscriber tasks exit when their NATS streams
/// end) and healed.
async fn nats_supervisor(
    state: Arc<AppState>,
    mut shutdown_rx: tokio::sync::broadcast::Receiver<()>,
) {
    use aeroxe_backend::infrastructure::messaging::nats_client::{
        connect_nats, ensure_jetstream_stream, JetStreamConfig,
    };
    use aeroxe_backend::infrastructure::messaging::EventPublisher;
    use aeroxe_backend::workers::outbox_worker::OutboxWorker;

    const MAX_DELAY_SECS: u64 = 60;
    let mut delay_secs: u64 = 2;
    let nats_url = state.settings.nats_url.clone();
    let db = std::sync::Arc::new(state.db.clone());
    let metrics = state.metrics.clone();
    let outbox_poll_interval_secs = state.settings.worker_outbox_poll_interval_secs;

    loop {
        let client = match connect_nats(&nats_url).await {
            Ok(client) => client,
            Err(e) => {
                tracing::warn!(
                    error = %e,
                    nats_url = %nats_url,
                    retry_in_secs = delay_secs,
                    "NATS unavailable; outbox events will queue until connectivity is restored"
                );
                let sleep = tokio::time::sleep(std::time::Duration::from_secs(delay_secs));
                tokio::pin!(sleep);
                tokio::select! {
                    _ = &mut sleep => {}
                    _ = shutdown_rx.recv() => {
                        tracing::info!("NATS supervisor shutting down (NATS never connected)");
                        return;
                    }
                }
                delay_secs = (delay_secs * 2).min(MAX_DELAY_SECS);
                continue;
            }
        };

        // JetStream is best-effort: durable streams need it, but core NATS
        // messaging and outbox delivery work without it.
        if let Err(e) = ensure_jetstream_stream(&client, &JetStreamConfig::default()).await {
            tracing::warn!(
                error = %e,
                "JetStream setup failed; proceeding with core NATS only"
            );
        }

        // Publish the live client so health checks and consumers see it.
        state.set_nats(client.clone()).await;
        delay_secs = 2;
        tracing::info!("NATS connected; starting outbox worker and event subscribers");

        // Outbox worker: drains the outbox table to NATS. It tolerates NATS
        // blips internally (events stay queued and retry), so we only recreate
        // it with a fresh client on full reconnects.
        let publisher = EventPublisher::new(client.clone());
        let mut outbox_worker = OutboxWorker::new(db.clone(), publisher);
        outbox_worker = outbox_worker.with_poll_interval(outbox_poll_interval_secs);
        if let Some(ref m) = metrics {
            outbox_worker = outbox_worker.with_metrics(m.clone());
        }
        let outbox_handle = tokio::spawn(async move { outbox_worker.run().await });

        // Subscribers: return when their NATS streams end (connection dropped),
        // signalling the supervisor to reconnect.
        let sub_client = client.clone();
        let sub_db = db.clone();
        let subscribers_handle = tokio::spawn(async move {
            if let Err(e) =
                aeroxe_backend::infrastructure::messaging::subscribers::start_subscribers(
                    sub_client, sub_db,
                )
                .await
            {
                tracing::error!(error = %e, "NATS event subscribers failed");
            }
        });

        tokio::pin!(subscribers_handle);
        tokio::select! {
            _ = shutdown_rx.recv() => {
                outbox_handle.abort();
                subscribers_handle.abort();
                tracing::info!("NATS supervisor shutting down");
                return;
            }
            _ = &mut subscribers_handle => {
                tracing::warn!(
                    "NATS event subscribers exited (connection likely dropped); reconnecting"
                );
                state.set_nats_offline().await;
                outbox_handle.abort();
            }
        }
    }
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

/// Log a boot-time summary of external integration adapters. Each adapter
/// falls back to placeholder values when its environment variables are
/// missing, so this is the only place where a partially-configured channel
/// is surfaced before it is used. Severity is a warning (not a fatal error)
/// because some integrations (e.g. WhatsApp, FCM) are optional and the server
/// can still run without them.
fn log_integration_health() {
    use aeroxe_backend::modules::integrations::huawei::adapter::HuaweiOltConfig;
    use aeroxe_backend::modules::integrations::mikrotik::adapter::MikrotikConfig;
    use aeroxe_backend::modules::integrations::push::fcm::FcmConfig;
    use aeroxe_backend::modules::integrations::radius::adapter::RadiusConfig;
    use aeroxe_backend::modules::integrations::sms::{msg91::Msg91Config, twilio::TwilioConfig};
    use aeroxe_backend::modules::integrations::smtp::SmtpConfig;
    use aeroxe_backend::modules::integrations::whatsapp::WhatsAppConfig;

    // SMTP email delivery
    let smtp = SmtpConfig::from_env();
    if smtp.username.is_empty() || smtp.password.is_empty() {
        tracing::warn!(
            integration = "smtp",
            missing = "SMTP_USERNAME,SMTP_PASSWORD",
            "SMTP email delivery is not configured — invoice and alert emails will be unavailable"
        );
    } else {
        tracing::info!(integration = "smtp", host = %smtp.host, "SMTP email delivery configured");
    }

    // SMS: MSG91 (default provider) and Twilio (optional)
    let msg91 = Msg91Config::default();
    if msg91.auth_key.is_empty() {
        tracing::warn!(
            integration = "sms_msg91",
            missing = "MSG91_AUTH_KEY",
            "MSG91 SMS delivery is not configured — OTP and SMS notifications will be unavailable"
        );
    } else {
        tracing::info!(
            integration = "sms_msg91",
            sender = %msg91.sender_id,
            "MSG91 SMS delivery configured"
        );
    }

    let twilio = TwilioConfig::default();
    if twilio.account_sid.is_empty() || twilio.auth_token.is_empty() {
        tracing::warn!(
            integration = "sms_twilio",
            missing = "TWILIO_ACCOUNT_SID,TWILIO_AUTH_TOKEN",
            "Twilio SMS delivery is not configured"
        );
    } else {
        tracing::info!(integration = "sms_twilio", "Twilio SMS delivery configured");
    }

    // WhatsApp Business API
    let whatsapp = WhatsAppConfig::default();
    if whatsapp.access_token.is_empty() || whatsapp.phone_number_id.is_empty() {
        tracing::warn!(
            integration = "whatsapp",
            missing = "WHATSAPP_ACCESS_TOKEN,WHATSAPP_PHONE_NUMBER_ID",
            "WhatsApp Business API is not configured — WhatsApp OTP and notifications will be unavailable"
        );
    } else {
        tracing::info!(integration = "whatsapp", "WhatsApp Business API configured");
    }

    // FCM push notifications
    let fcm = FcmConfig::default();
    if fcm.service_account_key.is_empty() || fcm.project_id.is_empty() {
        tracing::warn!(
            integration = "fcm",
            missing = "FCM_SERVICE_ACCOUNT_KEY,FCM_PROJECT_ID",
            "FCM push notifications are not configured — mobile app alerts will be unavailable"
        );
    } else {
        tracing::info!(integration = "fcm", "FCM push notifications configured");
    }

    // RADIUS PPPoE authentication/accounting
    let radius = RadiusConfig::default();
    if radius.secret.is_empty() {
        tracing::warn!(
            integration = "radius",
            missing = "RADIUS_SECRET",
            "RADIUS is not configured — PPPoE authentication and accounting will fail"
        );
    } else {
        tracing::info!(
            integration = "radius",
            server = %radius.server,
            "RADIUS authentication configured"
        );
    }

    // MikroTik device management
    let mikrotik = MikrotikConfig::default();
    if mikrotik.password.is_empty() {
        tracing::warn!(
            integration = "mikrotik",
            missing = "MIKROTIK_PASSWORD",
            "MikroTik device management is not configured — device sync and bandwidth provisioning will be unavailable"
        );
    } else {
        tracing::info!(
            integration = "mikrotik",
            host = %mikrotik.host,
            "MikroTik device management configured"
        );
    }

    // Huawei OLT provisioning
    let huawei = HuaweiOltConfig::default();
    if huawei.password.is_empty() {
        tracing::warn!(
            integration = "huawei_olt",
            missing = "HUAWEI_OLT_PASSWORD",
            "Huawei OLT management is not configured — GPON provisioning will be unavailable"
        );
    } else {
        tracing::info!(
            integration = "huawei_olt",
            host = %huawei.host,
            "Huawei OLT management configured"
        );
    }
}
