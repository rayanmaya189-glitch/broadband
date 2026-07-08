use utoipa::ToSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, ToSchema)]
pub struct ChannelQuery {
    pub channel: Option<String>,
}
