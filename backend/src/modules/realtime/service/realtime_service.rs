use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::response::Response;
use futures::{SinkExt, StreamExt};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use tokio::sync::{broadcast, RwLock};
use tracing::{info, warn};

use crate::app::SharedState;
use crate::common::config::config::Config;
use crate::common::middleware::auth_middleware::Claims;
use crate::modules::realtime::model::realtime::*;

/// WebSocket connection manager
#[derive(Clone)]
pub struct ConnectionManager {
    /// Broadcast channel for distributing messages to all connections
    pub broadcast_tx: BroadcastSender,
    /// Track active connections count
    pub active_connections: Arc<RwLock<HashMap<i64, usize>>>, // user_id -> count
}

impl ConnectionManager {
    pub fn new() -> Self {
        let (broadcast_tx, _) = broadcast::channel(1024);
        Self {
            broadcast_tx,
            active_connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn increment_connections(&self, user_id: i64) {
        let mut conns = self.active_connections.write().await;
        *conns.entry(user_id).or_insert(0) += 1;
    }

    pub async fn decrement_connections(&self, user_id: i64) {
        let mut conns = self.active_connections.write().await;
        if let Some(count) = conns.get_mut(&user_id) {
            if *count <= 1 {
                conns.remove(&user_id);
            } else {
                *count -= 1;
            }
        }
    }

    pub async fn total_connections(&self) -> usize {
        let conns = self.active_connections.read().await;
        conns.values().sum()
    }

    pub async fn user_connections(&self, user_id: i64) -> usize {
        let conns = self.active_connections.read().await;
        conns.get(&user_id).copied().unwrap_or(0)
    }

    /// Publish a message to all connected clients on a specific channel
    pub fn broadcast(&self, channel: &str, payload: &str) {
        let msg = BroadcastMessage {
            channel: channel.to_string(),
            payload: payload.to_string(),
        };
        if let Err(e) = self.broadcast_tx.send(msg) {
            warn!("Failed to broadcast message: {}", e);
        }
    }
}

/// Resolve WebSocket channels based on user role and context
pub fn resolve_channels(user_id: i64, role: &str, branch_id: Option<i64>) -> Vec<WsChannel> {
    let mut channels = vec![];

    // All authenticated users get their personal channel
    channels.push(WsChannel::CustomerUpdates(user_id));

    // Branch staff get branch-wide updates
    if let Some(bid) = branch_id {
        channels.push(WsChannel::BranchUpdates(bid));
    }

    // NOC engineers and above get NOC channels
    match role {
        "super_admin" | "isp_owner" | "network_admin" | "noc_engineer" => {
            channels.push(WsChannel::NocAlerts);
            channels.push(WsChannel::NocDevices);
            channels.push(WsChannel::NocSessions);
            channels.push(WsChannel::NocDiscovery);
        }
        _ => {}
    }

    // Admins get metrics
    match role {
        "super_admin" | "isp_owner" | "admin" => {
            channels.push(WsChannel::AdminMetrics);
        }
        _ => {}
    }

    channels
}

/// Validate a JWT token and extract claims.
fn validate_jwt(token: &str) -> Option<Claims> {
    let config = Config::get();
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &validation,
    ).ok().map(|d| d.claims)
}

/// Handle WebSocket upgrade request
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
    axum::extract::Query(params): axum::extract::Query<HashMap<String, String>>,
) -> Response {
    // Extract and validate JWT token from query params
    let token = params.get("token").cloned().unwrap_or_default();

    let claims = match validate_jwt(&token) {
        Some(c) => c,
        None => {
            warn!("WebSocket connection rejected: invalid JWT token");
            return ws.on_upgrade(|socket| async move {
                let (mut sender, _) = socket.split();
                let _ = sender.send(Message::Text("{\"error\":\"unauthorized\"}".into())).await;
                drop(sender);
            });
        }
    };

    let user_id = claims.sub;
    let role = claims.role.clone();
    let branch_id = claims.branch_id;

    info!(
        user_id = user_id,
        role = %role,
        "WebSocket upgrade request"
    );

    ws.on_upgrade(move |socket| {
        handle_socket(socket, state, user_id, role, branch_id)
    })
}

/// Handle individual WebSocket connection
async fn handle_socket(
    socket: WebSocket,
    state: SharedState,
    user_id: i64,
    role: String,
    branch_id: Option<i64>,
) {
    let (mut sender, mut receiver) = socket.split();

    // Resolve channels based on user context
    let default_channels = resolve_channels(user_id, &role, branch_id);
    let initial_channel_names: Vec<String> = default_channels.iter().map(|c| c.to_redis_channel()).collect();

    // Shared channel list between send and recv tasks
    let subscribed_channels = Arc::new(RwLock::new(initial_channel_names.clone()));

    info!(
        user_id = user_id,
        channels = ?initial_channel_names,
        "WebSocket client connected"
    );

    state.ws_manager.increment_connections(user_id).await;

    // Subscribe to broadcast channel
    let mut rx = state.ws_manager.broadcast_tx.subscribe();

    // Send welcome message
    let welcome = WsMessage::new(
        "connected",
        "system",
        serde_json::json!({
            "message": "Connected to AeroXe realtime",
            "channels": initial_channel_names,
            "user_id": user_id,
        }),
    );
    if let Ok(welcome_json) = serde_json::to_string(&welcome) {
        let _ = sender.send(Message::Text(welcome_json.into())).await;
    }

    // Spawn task to forward broadcast messages to this WebSocket client
    let channels_for_send = subscribed_channels.clone();
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // Check current subscribed channels (shared via Arc<RwLock>)
            let channels = channels_for_send.read().await;
            if channels.contains(&msg.channel) {
                drop(channels); // release lock before async send
                if let Err(e) = sender.send(Message::Text(msg.payload.into())).await {
                    warn!("Failed to send WebSocket message: {}", e);
                    break;
                }
            }
        }
    });

    // Handle incoming messages from client (with dynamic channel management)
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                        match json.get("type").and_then(|v| v.as_str()) {
                            Some("ping") => {
                                info!(user_id = user_id, "Received ping");
                            }
                            Some("subscribe") => {
                                if let Some(channel) = json.get("channel").and_then(|v| v.as_str()) {
                                    let mut channels = subscribed_channels.write().await;
                                    if !channels.contains(&channel.to_string()) {
                                        channels.push(channel.to_string());
                                        info!(user_id = user_id, channel = channel, "Client subscribed to channel");
                                    }
                                }
                            }
                            Some("unsubscribe") => {
                                if let Some(channel) = json.get("channel").and_then(|v| v.as_str()) {
                                    let mut channels = subscribed_channels.write().await;
                                    channels.retain(|c| c != channel);
                                    info!(user_id = user_id, channel = channel, "Client unsubscribed from channel");
                                }
                            }
                            _ => {
                                warn!(user_id = user_id, "Unknown message type: {:?}", json.get("type"));
                            }
                        }
                    }
                }
                Message::Ping(data) => {
                    let _ = data;
                }
                Message::Close(_) => {
                    info!(user_id = user_id, "WebSocket client disconnected");
                    break;
                }
                _ => {}
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }

    // Cleanup
    state.ws_manager.decrement_connections(user_id).await;
    info!(user_id = user_id, "WebSocket connection closed");
}

/// Realtime broadcaster service for other modules to publish events
pub struct RealtimeBroadcaster {
    connection_manager: Arc<ConnectionManager>,
}

impl RealtimeBroadcaster {
    pub fn new(connection_manager: Arc<ConnectionManager>) -> Self {
        Self { connection_manager }
    }

    /// Broadcast a message to a specific channel
    pub fn broadcast(&self, channel: &str, message_type: &str, data: serde_json::Value) {
        let msg = WsMessage::new(message_type, channel, data);
        if let Ok(payload) = serde_json::to_string(&msg) {
            self.connection_manager.broadcast(channel, &payload);
        }
    }

    /// Broadcast to customer-specific channel
    pub fn broadcast_to_customer(&self, customer_id: i64, message_type: &str, data: serde_json::Value) {
        let channel = format!("ws:customer:{}", customer_id);
        self.broadcast(&channel, message_type, data);
    }

    /// Broadcast to branch-wide channel
    pub fn broadcast_to_branch(&self, branch_id: i64, message_type: &str, data: serde_json::Value) {
        let channel = format!("ws:branch:{}", branch_id);
        self.broadcast(&channel, message_type, data);
    }

    /// Broadcast to NOC alerts channel
    pub fn broadcast_to_noc_alerts(&self, message_type: &str, data: serde_json::Value) {
        self.broadcast("ws:noc:alerts", message_type, data);
    }

    /// Broadcast to NOC devices channel
    pub fn broadcast_to_noc_devices(&self, message_type: &str, data: serde_json::Value) {
        self.broadcast("ws:noc:devices", message_type, data);
    }

    /// Broadcast to NOC sessions channel
    pub fn broadcast_to_noc_sessions(&self, message_type: &str, data: serde_json::Value) {
        self.broadcast("ws:noc:sessions", message_type, data);
    }

    /// Broadcast to NOC discovery channel
    pub fn broadcast_to_noc_discovery(&self, message_type: &str, data: serde_json::Value) {
        self.broadcast("ws:noc:discovery", message_type, data);
    }

    /// Broadcast to admin metrics channel
    pub fn broadcast_to_admin_metrics(&self, message_type: &str, data: serde_json::Value) {
        self.broadcast("ws:admin:metrics", message_type, data);
    }
}
