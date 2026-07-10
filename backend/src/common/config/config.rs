use std::sync::LazyLock;

/// Application configuration loaded from environment variables.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    // ── Server ──────────────────────────────────────────────
    pub server_host: String,
    pub server_port: u16,
    pub server_workers: usize,

    // ── Database (PostgreSQL) ───────────────────────────────
    pub database_url: String,
    pub db_max_connections: u32,
    pub db_min_connections: u32,
    pub db_connect_timeout_secs: u64,
    pub db_idle_timeout_secs: u64,

    // ── Redis ───────────────────────────────────────────────
    pub redis_url: String,

    // ── NATS ────────────────────────────────────────────────
    pub nats_url: String,

    // ── JWT ─────────────────────────────────────────────────
    pub jwt_secret: String,
    pub jwt_access_expiry_hours: i64,
    pub jwt_refresh_expiry_days: i64,

    // ── Rate Limiting ───────────────────────────────────────
    pub rate_limit_requests: u64,
    pub rate_limit_window_secs: u64,

    // ── CORS ────────────────────────────────────────────────
    pub cors_origins: Vec<String>,

    // ── SMTP ────────────────────────────────────────────────
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_pass: String,

    // ── MinIO / Object Storage ──────────────────────────────
    pub minio_endpoint: String,
    pub minio_access_key: String,
    pub minio_secret_key: String,
    pub minio_bucket: String,

    // ── Super Admin ──────────────────────────────────────────
    pub superadmin_email: Option<String>,
    pub superadmin_password: Option<String>,
}

static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    dotenvy::dotenv().ok();

    let cfg = config::Config::builder()
        .add_source(config::Environment::default().separator("__"))
        .build()
        .expect("Failed to build configuration");

    cfg.try_deserialize::<Config>()
        .expect("Failed to deserialize configuration — check your .env file")
});

impl Config {
    /// Returns the global config (loaded once, then cached).
    pub fn get() -> &'static Config {
        &CONFIG
    }

    /// Returns the database URL (redacted for logging).
    pub fn database_url_redacted(&self) -> String {
        self.database_url
            .split('@')
            .last()
            .unwrap_or("***")
            .to_string()
    }

    /// Returns the server address as "host:port".
    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}
