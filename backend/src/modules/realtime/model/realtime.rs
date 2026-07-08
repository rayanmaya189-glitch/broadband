use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

/// WebSocket message envelope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMessage {
    pub message_type: String,
    pub channel: String,
    pub data: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

impl WsMessage {
    pub fn new(message_type: &str, channel: &str, data: serde_json::Value) -> Self {
        Self {
            message_type: message_type.to_string(),
            channel: channel.to_string(),
            data,
            timestamp: Utc::now(),
        }
    }
}

/// WebSocket channels that clients can subscribe to
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WsChannel {
    CustomerUpdates(i64),   // ws:customer:{id}
    BranchUpdates(i64),     // ws:branch:{id}
    NocAlerts,              // ws:noc:alerts
    NocDevices,             // ws:noc:devices
    NocSessions,            // ws:noc:sessions
    NocDiscovery,           // ws:noc:discovery
    AdminMetrics,           // ws:admin:metrics
}

impl WsChannel {
    pub fn to_redis_channel(&self) -> String {
        match self {
            WsChannel::CustomerUpdates(id) => format!("ws:customer:{}", id),
            WsChannel::BranchUpdates(id) => format!("ws:branch:{}", id),
            WsChannel::NocAlerts => "ws:noc:alerts".to_string(),
            WsChannel::NocDevices => "ws:noc:devices".to_string(),
            WsChannel::NocSessions => "ws:noc:sessions".to_string(),
            WsChannel::NocDiscovery => "ws:noc:discovery".to_string(),
            WsChannel::AdminMetrics => "ws:admin:metrics".to_string(),
        }
    }
}

/// Connected client state
#[derive(Debug, Clone)]
pub struct ClientInfo {
    pub user_id: i64,
    pub role: String,
    pub branch_id: Option<i64>,
    pub connected_at: DateTime<Utc>,
}

/// Broadcast message for internal channel distribution
#[derive(Debug, Clone)]
pub struct BroadcastMessage {
    pub channel: String,
    pub payload: String,
}

/// Shared broadcast channel for WebSocket connections
pub type BroadcastSender = broadcast::Sender<BroadcastMessage>;
