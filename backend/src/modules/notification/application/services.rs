use crate::modules::notification::domain::entities::{
    DeliveryHistory, DeliveryHistoryColumn, Notification, NotificationActiveModel,
    NotificationColumn,     NotificationChannel, NotificationChannelActiveModel,
    NotificationTemplate, NotificationTemplateActiveModel,
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

    pub async fn update_template(
        db: &DatabaseConnection,
        id: i64,
        subject: Option<String>,
        body: Option<String>,
        channel: Option<String>,
    ) -> Result<crate::modules::notification::domain::entities::notification_template::Model, AppError> {
        let tmpl = NotificationTemplate::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound("Notification template not found".into()))?;
        let mut active: NotificationTemplateActiveModel = tmpl.into();
        if let Some(s) = subject {
            active.subject_template = Set(Some(s));
        }
        if let Some(b) = body {
            active.body_template = Set(b);
        }
        if let Some(c) = channel {
            active.channel = Set(c);
        }
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(db).await?)
    }

    pub async fn list_channels(
        db: &DatabaseConnection,
    ) -> Result<Vec<crate::modules::notification::domain::entities::notification_channel::Model>, AppError>
    {
        Ok(NotificationChannel::find().all(db).await?)
    }

    pub async fn update_channel(
        db: &DatabaseConnection,
        id: i64,
        is_active: Option<bool>,
        config: Option<serde_json::Value>,
    ) -> Result<crate::modules::notification::domain::entities::notification_channel::Model, AppError> {
        let ch = NotificationChannel::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound("Notification channel not found".into()))?;
        let mut active: NotificationChannelActiveModel = ch.into();
        if let Some(a) = is_active {
            active.is_active = Set(a);
        }
        if let Some(c) = config {
            active.config = Set(Some(c));
        }
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(db).await?)
    }

    pub async fn list_delivery_history(
        db: &DatabaseConnection,
        page: u64,
        limit: u64,
        status: Option<String>,
        channel: Option<String>,
    ) -> Result<
        (
            Vec<crate::modules::notification::domain::entities::delivery_history::Model>,
            u64,
        ),
        AppError,
    > {
        let mut q = DeliveryHistory::find();
        if let Some(s) = status {
            q = q.filter(DeliveryHistoryColumn::Status.eq(s));
        }
        if let Some(c) = channel {
            q = q.filter(DeliveryHistoryColumn::Channel.eq(c));
        }
        let total = q.clone().count(db).await?;
        let items = q
            .order_by_desc(DeliveryHistoryColumn::CreatedAt)
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

    pub async fn delete_template(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<(), AppError> {
        let tmpl = NotificationTemplate::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound("Notification template not found".into()))?;
        let active: NotificationTemplateActiveModel = tmpl.into();
        active.delete(db).await?;
        Ok(())
    }

    pub async fn retry_notification(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<crate::modules::notification::domain::entities::notification::Model, AppError> {
        let notif = Notification::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound("Notification not found".into()))?;

        if notif.status != "failed" {
            return Err(AppError::Validation(format!(
                "Cannot retry notification in '{}' status; must be 'failed'",
                notif.status
            )));
        }

        let mut active: NotificationActiveModel = notif.into();
        let current_retry = match &active.retry_count {
            sea_orm::Set(v) => *v,
            sea_orm::NotSet => 0,
            sea_orm::Unchanged(v) => *v,
        };
        active.status = Set("queued".to_string());
        active.retry_count = Set(current_retry + 1);
        active.last_error = Set(None);
        Ok(active.update(db).await?)
    }
}
