use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set};
use crate::shared::errors::AppError;
use crate::modules::notification::domain::entities::{NotificationTemplate, Notification, NotificationTemplateActiveModel, NotificationActiveModel};

pub struct NotificationService;

impl NotificationService {
    pub async fn list_templates(db: &DatabaseConnection) -> Result<Vec<crate::modules::notification::domain::entities::notification_template::Model>, AppError> {
        Ok(NotificationTemplate::find().all(db).await?)
    }

    pub async fn create_template(db: &DatabaseConnection, name: String, channel: String, body_template: String, subject_template: Option<String>) -> Result<crate::modules::notification::domain::entities::notification_template::Model, AppError> {
        let now = chrono::Utc::now();
        let tmpl = NotificationTemplateActiveModel {
            name: Set(name), channel: Set(channel), body_template: Set(body_template),
            subject_template: Set(subject_template), is_active: Set(true),
            created_at: Set(now), updated_at: Set(now), ..Default::default()
        };
        Ok(tmpl.insert(db).await?)
    }

    pub async fn send_notification(db: &DatabaseConnection, channel: String, recipient_type: String, recipient_id: i64, recipient_address: String, subject: Option<String>, body: String) -> Result<crate::modules::notification::domain::entities::notification::Model, AppError> {
        let now = chrono::Utc::now();
        let notif = NotificationActiveModel {
            channel: Set(channel), recipient_type: Set(recipient_type), recipient_id: Set(recipient_id),
            recipient_address: Set(recipient_address), subject: Set(subject), body: Set(body),
            status: Set("queued".to_string()), retry_count: Set(0), max_retries: Set(3),
            created_at: Set(now), ..Default::default()
        };
        Ok(notif.insert(db).await?)
    }

    pub async fn list_notifications(db: &DatabaseConnection) -> Result<Vec<crate::modules::notification::domain::entities::notification::Model>, AppError> {
        Ok(Notification::find().all(db).await?)
    }
}
