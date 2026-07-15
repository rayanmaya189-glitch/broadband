use axum::extract::State;
use axum::response::Response;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use crate::shared::app_state::SharedState;
use crate::shared::middleware::auth::UserContext;

/// WebSocket channels based on user roles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WsChannel {
    CustomerUpdates(i64),
    BranchAlerts(i64),
    NocAlerts,
    NocDevices,
    NocSessions,
    NocDiscovery,
    AdminMetrics,
}

impl WsChannel {
    pub fn to_redis_channel(&self) -> String {
        match self {
            WsChannel::CustomerUpdates(id) => format!("ws:customer:{}", id),
            WsChannel::BranchAlerts(id) => format!("ws:branch:{}", id),
            WsChannel::NocAlerts => "ws:noc:alerts".to_string(),
            WsChannel::NocDevices => "ws:noc:devices".to_string(),
            WsChannel::NocSessions => "ws:noc:sessions".to_string(),
            WsChannel::NocDiscovery => "ws:noc:discovery".to_string(),
            WsChannel::AdminMetrics => "ws:admin:metrics".to_string(),
        }
    }
}

/// WebSocket message format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMessage {
    pub msg_type: String,
    pub channel: String,
    pub data: serde_json::Value,
    pub timestamp: String,
}

/// Connected client information
#[derive(Debug, Clone)]
pub struct ConnectedClient {
    pub user_id: i64,
    pub role: String,
    pub branch_id: Option<i64>,
    pub channels: Vec<WsChannel>,
}

/// Connection manager for tracking active WebSocket connections
pub struct ConnectionManager {
    connections: RwLock<HashMap<i64, Vec<ConnectedClient>>>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: RwLock::new(HashMap::new()),
        }
    }

    pub async fn add_client(&self, user_id: i64, client: ConnectedClient) {
        let mut connections = self.connections.write().await;
        connections.entry(user_id).or_default().push(client);
    }

    pub async fn remove_client(&self, user_id: i64) {
        let mut connections = self.connections.write().await;
        connections.remove(&user_id);
    }

    pub async fn get_connection_count(&self) -> usize {
        let connections = self.connections.read().await;
        connections.values().map(|v| v.len()).sum()
    }
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Resolve channels based on user role and branch
pub fn resolve_channels(user: &UserContext) -> Vec<WsChannel> {
    let mut channels = Vec::new();

    if let Some(branch_id) = user.branch_id {
        channels.push(WsChannel::BranchAlerts(branch_id));
    }

    match user.role.as_str() {
        "super_admin" | "isp_owner" => {
            channels.push(WsChannel::AdminMetrics);
            channels.push(WsChannel::NocAlerts);
            channels.push(WsChannel::NocDevices);
            channels.push(WsChannel::NocSessions);
            channels.push(WsChannel::NocDiscovery);
        }
        "noc_engineer" | "network_admin" => {
            channels.push(WsChannel::NocAlerts);
            channels.push(WsChannel::NocDevices);
            channels.push(WsChannel::NocSessions);
            channels.push(WsChannel::NocDiscovery);
        }
        "finance_manager" | "billing_admin" => {
            channels.push(WsChannel::AdminMetrics);
        }
        _ => {}
    }

    channels
}

/// WebSocket upgrade handler
pub async fn ws_handler(
    ws: axum::extract::ws::WebSocketUpgrade,
    State(state): State<SharedState>,
    user: UserContext,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state, user))
}

/// Handle individual WebSocket connection
async fn handle_socket(
    socket: axum::extract::ws::WebSocket,
    _state: SharedState,
    user: UserContext,
) {
    let (mut sender, mut receiver) = socket.split();
    let user_id = user.user_id;

    info!(user_id = user_id, role = %user.role, "WebSocket client connected");

    let channels = resolve_channels(&user);
    let channel_names: Vec<String> = channels.iter().map(|c| c.to_redis_channel()).collect();

    debug!(user_id = user_id, channels = ?channel_names, "Subscribing to channels");

    // Send initial connection confirmation
    let welcome = WsMessage {
        msg_type: "connected".to_string(),
        channel: "system".to_string(),
        data: serde_json::json!({
            "user_id": user_id,
            "channels": channel_names,
            "message": "Connected to AeroXe realtime"
        }),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    if let Ok(welcome_json) = serde_json::to_string(&welcome) {
        let _ = sender
            .send(axum::extract::ws::Message::Text(welcome_json.into()))
            .await;
    }

    // Handle incoming WebSocket messages (client -> server)
    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            axum::extract::ws::Message::Text(text) => {
                if let Err(e) = handle_client_message(&user, text.to_string()).await {
                    warn!(user_id = user_id, error = %e, "Error handling client message");
                }
            }
            axum::extract::ws::Message::Ping(data) => {
                let _ = sender.send(axum::extract::ws::Message::Pong(data)).await;
            }
            axum::extract::ws::Message::Close(_) => {
                info!(user_id = user_id, "WebSocket client disconnected");
                break;
            }
            _ => {}
        }
    }

    info!(user_id = user_id, "WebSocket connection closed");
}

/// Handle messages from WebSocket clients
async fn handle_client_message(
    user: &UserContext,
    text: String,
) -> Result<(), String> {
    let msg: WsMessage = serde_json::from_str(&text)
        .map_err(|e| format!("Invalid message format: {}", e))?;

    match msg.msg_type.as_str() {
        "ping" => {
            debug!(user_id = user.user_id, "Received ping from client");
        }
        "subscribe" => {
            debug!(user_id = user.user_id, channel = %msg.channel, "Client requested subscription");
        }
        "unsubscribe" => {
            debug!(user_id = user.user_id, channel = %msg.channel, "Client requested unsubscription");
        }
        _ => {
            debug!(user_id = user.user_id, msg_type = %msg.msg_type, "Unknown message type");
        }
    }

    Ok(())
}

/// Broadcast a message to all clients on a specific channel using Redis PUBLISH
pub async fn broadcast_to_channel(
    redis: &redis::aio::ConnectionManager,
    channel: &str,
    message: &WsMessage,
) -> Result<(), String> {
    let payload = serde_json::to_string(message)
        .map_err(|e| format!("Serialization error: {}", e))?;

    let mut conn = redis.clone();
    let _: () = redis::cmd("PUBLISH")
        .arg(channel)
        .arg(&payload)
        .query_async(&mut conn)
        .await
        .map_err(|e| format!("Redis publish error: {}", e))?;

    Ok(())
}

/// Broadcast device status change to NOC
pub async fn broadcast_device_status(
    redis: &redis::aio::ConnectionManager,
    device_id: i64,
    device_name: &str,
    old_status: &str,
    new_status: &str,
) -> Result<(), String> {
    let msg = WsMessage {
        msg_type: "device.status.changed".to_string(),
        channel: "ws:noc:devices".to_string(),
        data: serde_json::json!({
            "device_id": device_id,
            "device_name": device_name,
            "old_status": old_status,
            "new_status": new_status,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    broadcast_to_channel(redis, "ws:noc:devices", &msg).await
}

/// Broadcast alert to NOC
pub async fn broadcast_noc_alert(
    redis: &redis::aio::ConnectionManager,
    alert_type: &str,
    message: &str,
    severity: &str,
) -> Result<(), String> {
    let msg = WsMessage {
        msg_type: "noc.alert".to_string(),
        channel: "ws:noc:alerts".to_string(),
        data: serde_json::json!({
            "alert_type": alert_type,
            "message": message,
            "severity": severity,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    broadcast_to_channel(redis, "ws:noc:alerts", &msg).await
}

/// Broadcast customer update
pub async fn broadcast_customer_update(
    redis: &redis::aio::ConnectionManager,
    customer_id: i64,
    update_type: &str,
    data: serde_json::Value,
) -> Result<(), String> {
    let channel = format!("ws:customer:{}", customer_id);
    let msg = WsMessage {
        msg_type: update_type.to_string(),
        channel: channel.clone(),
        data,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    broadcast_to_channel(redis, &channel, &msg).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_channels_super_admin() {
        let user = UserContext {
            user_id: 1,
            email: "admin@aeroxe.com".to_string(),
            role: "super_admin".to_string(),
            branch_id: Some(1),
            is_company_wide: true,
            permissions: Vec::new(),
        };

        let channels = resolve_channels(&user);
        assert!(channels.contains(&WsChannel::AdminMetrics));
        assert!(channels.contains(&WsChannel::NocAlerts));
        assert!(channels.contains(&WsChannel::NocDevices));
        assert!(channels.contains(&WsChannel::NocSessions));
        assert!(channels.contains(&WsChannel::NocDiscovery));
        assert!(channels.contains(&WsChannel::BranchAlerts(1)));
    }

    #[test]
    fn test_resolve_channels_noc_engineer() {
        let user = UserContext {
            user_id: 2,
            email: "noc@aeroxe.com".to_string(),
            role: "noc_engineer".to_string(),
            branch_id: Some(1),
            is_company_wide: false,
            permissions: Vec::new(),
        };

        let channels = resolve_channels(&user);
        assert!(!channels.contains(&WsChannel::AdminMetrics));
        assert!(channels.contains(&WsChannel::NocAlerts));
        assert!(channels.contains(&WsChannel::NocDevices));
    }

    #[test]
    fn test_ws_channel_to_redis_channel() {
        assert_eq!(WsChannel::CustomerUpdates(42).to_redis_channel(), "ws:customer:42");
        assert_eq!(WsChannel::BranchAlerts(5).to_redis_channel(), "ws:branch:5");
        assert_eq!(WsChannel::NocAlerts.to_redis_channel(), "ws:noc:alerts");
        assert_eq!(WsChannel::AdminMetrics.to_redis_channel(), "ws:admin:metrics");
    }
}
