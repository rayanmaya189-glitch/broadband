use crate::shared::errors::AppError;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;

pub type NotificationModel = crate::modules::notification::domain::entities::notification::Model;
pub type NotificationTemplateModel =
    crate::modules::notification::domain::entities::notification_template::Model;

#[async_trait]
pub trait NotificationServiceTrait: Send + Sync {
    async fn list_templates(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<NotificationTemplateModel>, AppError>;

    async fn create_template(
        &self,
        db: &DatabaseConnection,
        name: String,
        channel: String,
        body_template: String,
        subject_template: Option<String>,
    ) -> Result<NotificationTemplateModel, AppError>;

    async fn send_notification(
        &self,
        db: &DatabaseConnection,
        channel: String,
        recipient_type: String,
        recipient_id: i64,
        recipient_address: String,
        subject: Option<String>,
        body: String,
    ) -> Result<NotificationModel, AppError>;

    async fn list_notifications(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Vec<NotificationModel>, AppError>;
}
