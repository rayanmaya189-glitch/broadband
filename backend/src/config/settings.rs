use anyhow::{Context, Result};
use std::env;

/// Application configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct Settings {
    // Server
    pub server_addr: String,

    // Database
    pub database_url: String,
    pub db_max_connections: u32,
    pub db_min_connections: u32,
    pub db_connect_timeout_secs: u64,
    pub db_idle_timeout_secs: u64,

    // Redis
    pub redis_url: String,

    // NATS
    pub nats_url: String,

    // JWT (RS256 asymmetric keys)
    pub jwt_private_key_pem: Option<String>,
    pub jwt_public_key_pem: Option<String>,
    pub jwt_access_token_ttl_secs: i64,
    pub jwt_refresh_token_ttl_secs: i64,

    // MinIO / Storage
    pub minio_endpoint: String,
    pub minio_access_key: String,
    pub minio_secret_key: String,
    pub minio_bucket_kyc: String,
    pub minio_bucket_invoices: String,
    pub minio_bucket_documents: String,

    // SMTP
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_from_email: String,

    // Application
    pub app_name: String,
    pub app_env: String,
    pub app_public_url: String,
    pub cors_origins: Vec<String>,

    // Security
    pub jwt_key_rotation_days: i64,

    // Worker poll intervals
    pub worker_outbox_poll_interval_secs: u64,
    pub worker_billing_poll_interval_secs: u64,
    pub worker_notification_poll_interval_secs: u64,
    pub worker_device_sync_poll_interval_secs: u64,
    pub worker_bandwidth_poll_interval_secs: u64,
    pub worker_radius_poll_interval_secs: u64,
    pub worker_scheduler_poll_interval_secs: u64,
    pub worker_monitoring_poll_interval_secs: u64,
    pub worker_partition_poll_interval_secs: u64,
    pub worker_outbox_cleanup_poll_interval_secs: u64,

    // Tax / Compliance
    pub supplier_gstin: String,
    pub supplier_state: String,
    pub tds_rate_professional: f64,
    pub tds_rate_contractor: f64,
    pub tds_pan: String,
}

impl Settings {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            server_addr: env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:8000".to_string()),

            database_url: env::var("DATABASE_URL").context("DATABASE_URL must be set")?,
            db_max_connections: env::var("DB_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "20".to_string())
                .parse()
                .unwrap_or(20),
            db_min_connections: env::var("DB_MIN_CONNECTIONS")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .unwrap_or(5),
            db_connect_timeout_secs: env::var("DB_CONNECT_TIMEOUT_SECS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
            db_idle_timeout_secs: env::var("DB_IDLE_TIMEOUT_SECS")
                .unwrap_or_else(|_| "600".to_string())
                .parse()
                .unwrap_or(600),

            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),

            nats_url: env::var("NATS_URL").unwrap_or_else(|_| "nats://127.0.0.1:4222".to_string()),

            jwt_private_key_pem: env::var("JWT_PRIVATE_KEY").ok().filter(|s| !s.is_empty()),
            jwt_public_key_pem: env::var("JWT_PUBLIC_KEY").ok().filter(|s| !s.is_empty()),
            jwt_access_token_ttl_secs: env::var("JWT_ACCESS_TOKEN_TTL_SECS")
                .unwrap_or_else(|_| "1800".to_string()) // 30 minutes
                .parse()
                .unwrap_or(1800),
            jwt_refresh_token_ttl_secs: env::var("JWT_REFRESH_TOKEN_TTL_SECS")
                .unwrap_or_else(|_| "604800".to_string()) // 7 days
                .parse()
                .unwrap_or(604800),

            minio_endpoint: env::var("MINIO_ENDPOINT")
                .unwrap_or_else(|_| "minio.aeroxe.internal:9000".to_string()),
            minio_access_key: env::var("MINIO_ACCESS_KEY").unwrap_or_default(),
            minio_secret_key: env::var("MINIO_SECRET_KEY").unwrap_or_default(),
            minio_bucket_kyc: env::var("MINIO_BUCKET_KYC")
                .unwrap_or_else(|_| "aeroxe-kyc".to_string()),
            minio_bucket_invoices: env::var("MINIO_BUCKET_INVOICES")
                .unwrap_or_else(|_| "aeroxe-invoices".to_string()),
            minio_bucket_documents: env::var("MINIO_BUCKET_DOCUMENTS")
                .unwrap_or_else(|_| "aeroxe-documents".to_string()),

            smtp_host: env::var("SMTP_HOST").unwrap_or_else(|_| "smtp.gmail.com".to_string()),
            smtp_port: env::var("SMTP_PORT")
                .unwrap_or_else(|_| "587".to_string())
                .parse()
                .unwrap_or(587),
            smtp_username: env::var("SMTP_USERNAME").unwrap_or_default(),
            smtp_password: env::var("SMTP_PASSWORD").unwrap_or_default(),
            smtp_from_email: env::var("SMTP_FROM_EMAIL")
                .unwrap_or_else(|_| "noreply@aeroxebroadband.com".to_string()),

            app_name: env::var("APP_NAME").unwrap_or_else(|_| "AeroXe Broadband".to_string()),
            app_env: env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()),
            app_public_url: env::var("APP_PUBLIC_URL")
                .unwrap_or_else(|_| "http://localhost:8000".to_string()),
            cors_origins: env::var("CORS_ORIGINS")
                .unwrap_or_else(|_| "http://localhost:3000,http://localhost:5173".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),

            jwt_key_rotation_days: env::var("JWT_KEY_ROTATION_DAYS")
                .unwrap_or_else(|_| "90".to_string())
                .parse()
                .unwrap_or(90),

            worker_outbox_poll_interval_secs: env::var("WORKER_OUTBOX_POLL_INTERVAL_SECS")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .unwrap_or(5),
            worker_billing_poll_interval_secs: env::var("WORKER_BILLING_POLL_INTERVAL_SECS")
                .unwrap_or_else(|_| "300".to_string())
                .parse()
                .unwrap_or(300),
            worker_notification_poll_interval_secs: env::var(
                "WORKER_NOTIFICATION_POLL_INTERVAL_SECS",
            )
            .unwrap_or_else(|_| "30".to_string())
            .parse()
            .unwrap_or(30),
            worker_device_sync_poll_interval_secs: env::var(
                "WORKER_DEVICE_SYNC_POLL_INTERVAL_SECS",
            )
            .unwrap_or_else(|_| "120".to_string())
            .parse()
            .unwrap_or(120),
            worker_bandwidth_poll_interval_secs: env::var("WORKER_BANDWIDTH_POLL_INTERVAL_SECS")
                .unwrap_or_else(|_| "60".to_string())
                .parse()
                .unwrap_or(60),
            worker_radius_poll_interval_secs: env::var("WORKER_RADIUS_POLL_INTERVAL_SECS")
                .unwrap_or_else(|_| "300".to_string())
                .parse()
                .unwrap_or(300),
            worker_scheduler_poll_interval_secs: env::var("WORKER_SCHEDULER_POLL_INTERVAL_SECS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
            worker_monitoring_poll_interval_secs: env::var("WORKER_MONITORING_POLL_INTERVAL_SECS")
                .unwrap_or_else(|_| "120".to_string())
                .parse()
                .unwrap_or(120),
            worker_partition_poll_interval_secs: env::var("WORKER_PARTITION_POLL_INTERVAL_SECS")
                .unwrap_or_else(|_| "86400".to_string())
                .parse()
                .unwrap_or(86400),
            worker_outbox_cleanup_poll_interval_secs: env::var(
                "WORKER_OUTBOX_CLEANUP_POLL_INTERVAL_SECS",
            )
            .unwrap_or_else(|_| "3600".to_string())
            .parse()
            .unwrap_or(3600),

            supplier_gstin: env::var("SUPPLIER_GSTIN")
                .unwrap_or_else(|_| "27AABCA1234H1Z5".to_string()),
            supplier_state: env::var("SUPPLIER_STATE")
                .unwrap_or_else(|_| "Maharashtra".to_string()),
            tds_rate_professional: env::var("TDS_RATE_PROFESSIONAL")
                .unwrap_or_else(|_| "0.10".to_string())
                .parse()
                .unwrap_or(0.10),
            tds_rate_contractor: env::var("TDS_RATE_CONTRACTOR")
                .unwrap_or_else(|_| "0.02".to_string())
                .parse()
                .unwrap_or(0.02),
            tds_pan: env::var("TDS_PAN").unwrap_or_default(),
        })
    }
}
