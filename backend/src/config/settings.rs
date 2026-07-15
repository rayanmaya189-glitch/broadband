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

    // Redis
    pub redis_url: String,

    // NATS
    pub nats_url: String,

    // JWT
    pub jwt_secret: String,
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

    // SMS
    pub sms_api_key: String,
    pub sms_sender_id: String,

    // Application
    pub app_name: String,
    pub app_env: String,
    pub cors_origins: Vec<String>,
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

            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),

            nats_url: env::var("NATS_URL").unwrap_or_else(|_| "nats://127.0.0.1:4222".to_string()),

            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "aeroxe-jwt-secret-change-in-production".to_string()),
            jwt_access_token_ttl_secs: env::var("JWT_ACCESS_TOKEN_TTL_SECS")
                .unwrap_or_else(|_| "86400".to_string()) // 24 hours
                .parse()
                .unwrap_or(86400),
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

            sms_api_key: env::var("SMS_API_KEY").unwrap_or_default(),
            sms_sender_id: env::var("SMS_SENDER_ID").unwrap_or_else(|_| "AEROXE".to_string()),

            app_name: env::var("APP_NAME").unwrap_or_else(|_| "AeroXe Broadband".to_string()),
            app_env: env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()),
            cors_origins: env::var("CORS_ORIGINS")
                .unwrap_or_else(|_| "http://localhost:3000,http://localhost:5173".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        })
    }
}
