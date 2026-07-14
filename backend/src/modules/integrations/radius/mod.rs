//! RADIUS integration adapter.
//!
//! Adapter for RADIUS server integration (authentication and accounting).

use serde::{Deserialize, Serialize};

/// RADIUS server configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiusConfig {
    pub server: String,
    pub port: u16,
    pub shared_secret: String,
    pub timeout_ms: u64,
}

/// RADIUS adapter trait.
#[async_trait::async_trait]
pub trait RadiusAdapter: Send + Sync {
    /// Authenticate a user.
    async fn authenticate(
        &self,
        username: &str,
        password: &str,
    ) -> Result<RadiusResponse, RadiusError>;

    /// Start accounting session.
    async fn start_accounting(
        &self,
        session: &AccountingSession,
    ) -> Result<(), RadiusError>;

    /// Stop accounting session.
    async fn stop_accounting(
        &self,
        session: &AccountingSession,
    ) -> Result<(), RadiusError>;
}

/// RADIUS response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiusResponse {
    pub accepted: bool,
    pub session_timeout: Option<u32>,
    pub ip_address: Option<String>,
    pub framed_protocol: Option<String>,
    pub reply_message: Option<String>,
}

/// RADIUS accounting session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountingSession {
    pub session_id: String,
    pub username: String,
    pub nas_ip: String,
    pub framed_ip: Option<String>,
    pub input_octets: u64,
    pub output_octets: u64,
    pub session_time: u32,
}

/// RADIUS errors.
#[derive(Debug, Clone)]
pub enum RadiusError {
    ServerUnavailable(String),
    AuthenticationFailed,
    AccountingFailed(String),
    NetworkError(String),
}

impl std::fmt::Display for RadiusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RadiusError::ServerUnavailable(msg) => write!(f, "Server unavailable: {}", msg),
            RadiusError::AuthenticationFailed => write!(f, "Authentication failed"),
            RadiusError::AccountingFailed(msg) => write!(f, "Accounting failed: {}", msg),
            RadiusError::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

impl std::error::Error for RadiusError {}
