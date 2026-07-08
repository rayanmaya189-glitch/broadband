use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTemplateRequest { pub name: String, pub channel: String, pub subject_template: Option<String>, pub body_template: String }
#[derive(Debug, Deserialize, Validate)]
pub struct SendNotificationRequest { pub channel: String, pub recipient_id: i64, pub recipient_address: String, pub subject: Option<String>, pub body: String }
