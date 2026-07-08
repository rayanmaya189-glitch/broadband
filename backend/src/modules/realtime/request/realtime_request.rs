use utoipa::ToSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[derive(ToSchema)]
pub struct ChannelQuery {
    pub channel: Option<String>,
}
