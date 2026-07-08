use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub connections: usize,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ChannelInfo {
    pub name: String,
    pub description: String,
    pub subscribers: usize,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ConnectionStats {
    pub total_connections: usize,
    pub channels: Vec<ChannelInfo>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WsMessageResponse {
    pub message_type: String,
    pub channel: String,
    pub data: serde_json::Value,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}
