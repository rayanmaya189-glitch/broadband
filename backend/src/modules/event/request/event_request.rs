use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct EventQuery { pub event_type: Option<String>, pub page: Option<i64>, pub per_page: Option<i64> }
