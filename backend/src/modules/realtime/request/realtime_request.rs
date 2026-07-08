use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChannelQuery {
    pub channel: Option<String>,
}
