use crate::modules::notification::domain::entities::{
    Notification, NotificationActiveModel, NotificationColumn, NotificationTemplate,
    NotificationTemplateActiveModel,
};
use crate::shared::errors::AppError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};

pub struct NotificationService;

impl NotificationService {
    pub async fn list_templates(
        db: &DatabaseConnection,
        _page: u64,
        _limit: u64,
    ) -> Result<
        (
            Vec<crate::modules::notification::domain::entities::notification_template::Model>,
            u64,
        ),
        AppError,
    > {
        let q = NotificationTemplate::find();
        let t = q.clone().count(db).await?;
        Ok((q.all(db).await?, t))
    }

    pub async fn create_template(
        db: &DatabaseConnection,
        name: String,
        channel: String,
        body_template: String,
        subject_template: Option<String>,
    ) -> Result<
        crate::modules::notification::domain::entities::notification_template::Model,
        AppError,
    > {
        let now = chrono::Utc::now();
        let tmpl = NotificationTemplateActiveModel {
            name: Set(name),
            channel: Set(channel),
            body_template: Set(body_template),
            subject_template: Set(subject_template),
            is_active: Set(true),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(tmpl.insert(db).await?)
    }

    pub async fn send_notification(
        db: &DatabaseConnection,
        channel: String,
        recipient_type: String,
        recipient_id: i64,
        recipient_address: String,
        subject: Option<String>,
        body: String,
    ) -> Result<crate::modules::notification::domain::entities::notification::Model, AppError> {
        let now = chrono::Utc::now();
        let notif = NotificationActiveModel {
            channel: Set(channel.clone()),
            recipient_type: Set(recipient_type),
            recipient_id: Set(recipient_id),
            recipient_address: Set(recipient_address),
            subject: Set(subject),
            body: Set(body),
            status: Set("queued".to_string()),
            retry_count: Set(0),
            max_retries: Set(3),
            created_at: Set(now),
            ..Default::default()
        };
        Ok(notif.insert(db).await?)
    }

    pub async fn list_notifications(
        db: &DatabaseConnection,
        page: u64,
        limit: u64,
    ) -> Result<
        (
            Vec<crate::modules::notification::domain::entities::notification::Model>,
            u64,
        ),
        AppError,
    > {
        let q = Notification::find();
        let total = q.clone().count(db).await?;
        let items = q
            .order_by_desc(NotificationColumn::CreatedAt)
            .paginate(db, limit)
            .fetch_page(page.saturating_sub(1))
            .await?;
        Ok((items, total))
    }

    /// Retry failed notifications (status = 'failed' and retry_count < max_retries)
    pub async fn retry_failed_notifications(db: &DatabaseConnection) -> Result<u64, AppError> {
        let failed = Notification::find()
            .filter(NotificationColumn::Status.eq("failed"))
            .filter(NotificationColumn::RetryCount.lt(3))
            .all(db)
            .await?;

        let mut retried = 0u64;
        for notif in failed {
            let mut active: NotificationActiveModel = notif.into();
            let current_retry = match active.retry_count {
                sea_orm::Set(v) => v,
                sea_orm::NotSet => 0,
                sea_orm::Unchanged(v) => v,
            };
            active.status = Set("queued".to_string());
            active.retry_count = Set(current_retry + 1);
            if active.update(db).await.is_ok() {
                retried += 1;
            }
        }
        Ok(retried)
    }
}
