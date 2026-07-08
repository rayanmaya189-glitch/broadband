use crate::common::errors::app_error::AppError;
use crate::modules::realtime::response::realtime_response::*;

pub struct RealtimeService;

impl RealtimeService {
    pub fn new() -> Self { Self }

    pub async fn health(&self) -> Result<HealthResponse, AppError> {
        Ok(HealthResponse { status: "ok".into(), connections: 0 })
    }

    pub async fn list_channels(&self) -> Result<Vec<ChannelInfo>, AppError> {
        Ok(vec![
            ChannelInfo { name: "ws:noc:alerts".into(), subscribers: 0 },
            ChannelInfo { name: "ws:noc:devices".into(), subscribers: 0 },
            ChannelInfo { name: "ws:noc:sessions".into(), subscribers: 0 },
        ])
    }
}
