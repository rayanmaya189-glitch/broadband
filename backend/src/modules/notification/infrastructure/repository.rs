use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};

use crate::modules::notification::domain::entities::{notification, notification_template};
use crate::shared::errors::AppError;

pub struct NotificationRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> NotificationRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    // ── Notifications ─────────────────────────────────────────────────

    pub async fn find_by_id(&self, id: i64) -> Result<Option<notification::Model>, AppError> {
        Ok(notification::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn find_queued(&self, limit: i64) -> Result<Vec<notification::Model>, AppError> {
        Ok(notification::Entity::find()
            .filter(notification::Column::Status.eq("queued"))
            .order_by_asc(notification::Column::CreatedAt)
            .limit(limit as u64)
            .all(self.db)
            .await?)
    }

    pub async fn find_failed(&self, limit: i64) -> Result<Vec<notification::Model>, AppError> {
        Ok(notification::Entity::find()
            .filter(notification::Column::Status.eq("failed"))
            .order_by_asc(notification::Column::CreatedAt)
            .limit(limit as u64)
            .all(self.db)
            .await?)
    }

    pub async fn list_notifications(
        &self,
        channel: Option<&str>,
        status: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<notification::Model>, AppError> {
        let mut query = notification::Entity::find();
        if let Some(ch) = channel {
            query = query.filter(notification::Column::Channel.eq(ch));
        }
        if let Some(s) = status {
            query = query.filter(notification::Column::Status.eq(s));
        }
        Ok(query
            .order_by_desc(notification::Column::CreatedAt)
            .limit(limit as u64)
            .offset(offset as u64)
            .all(self.db)
            .await?)
    }

    pub async fn create_notification(
        &self,
        channel: String,
        recipient_type: String,
        recipient_id: i64,
        recipient_address: String,
        subject: Option<String>,
        body: String,
        variables: Option<serde_json::Value>,
    ) -> Result<notification::Model, AppError> {
        let model = notification::ActiveModel {
            channel: Set(channel),
            recipient_type: Set(recipient_type),
            recipient_id: Set(recipient_id),
            recipient_address: Set(recipient_address),
            subject: Set(subject),
            body: Set(body),
            variables: Set(variables),
            status: Set("queued".to_string()),
            retry_count: Set(0),
            max_retries: Set(3),
            ..Default::default()
        };
        Ok(model.insert(self.db).await?)
    }

    pub async fn mark_sent(&self, id: i64) -> Result<(), AppError> {
        if let Some(model) = notification::Entity::find_by_id(id).one(self.db).await? {
            let mut active: notification::ActiveModel = model.into();
            active.status = Set("sent".to_string());
            active.sent_at = Set(Some(chrono::Utc::now()));
            active.update(self.db).await?;
        }
        Ok(())
    }

    pub async fn mark_delivered(&self, id: i64) -> Result<(), AppError> {
        if let Some(model) = notification::Entity::find_by_id(id).one(self.db).await? {
            let mut active: notification::ActiveModel = model.into();
            active.status = Set("delivered".to_string());
            active.delivered_at = Set(Some(chrono::Utc::now()));
            active.update(self.db).await?;
        }
        Ok(())
    }

    pub async fn mark_failed(&self, id: i64, error: &str) -> Result<(), AppError> {
        if let Some(model) = notification::Entity::find_by_id(id).one(self.db).await? {
            let retry_count = model.retry_count;
            let mut active: notification::ActiveModel = model.into();
            active.status = Set("failed".to_string());
            active.last_error = Set(Some(error.to_string()));
            active.retry_count = Set(retry_count + 1);
            active.update(self.db).await?;
        }
        Ok(())
    }

    pub async fn count_by_status(&self, status: &str) -> Result<i64, AppError> {
        Ok(notification::Entity::find()
            .filter(notification::Column::Status.eq(status))
            .count(self.db)
            .await? as i64)
    }

    // ── Templates ─────────────────────────────────────────────────────

    pub async fn find_template_by_id(
        &self,
        id: i64,
    ) -> Result<Option<notification_template::Model>, AppError> {
        Ok(notification_template::Entity::find_by_id(id)
            .one(self.db)
            .await?)
    }

    pub async fn find_template_by_name(
        &self,
        name: &str,
    ) -> Result<Option<notification_template::Model>, AppError> {
        Ok(notification_template::Entity::find()
            .filter(notification_template::Column::Name.eq(name))
            .one(self.db)
            .await?)
    }

    pub async fn list_templates(&self) -> Result<Vec<notification_template::Model>, AppError> {
        Ok(notification_template::Entity::find()
            .order_by_desc(notification_template::Column::CreatedAt)
            .all(self.db)
            .await?)
    }

    pub async fn create_template(
        &self,
        name: String,
        channel: String,
        subject_template: Option<String>,
        body_template: String,
        variables: Option<serde_json::Value>,
    ) -> Result<notification_template::Model, AppError> {
        let now = chrono::Utc::now();
        let model = notification_template::ActiveModel {
            name: Set(name),
            channel: Set(channel),
            subject_template: Set(subject_template),
            body_template: Set(body_template),
            variables: Set(variables),
            is_active: Set(true),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(model.insert(self.db).await?)
    }

    pub async fn update_template(
        &self,
        model: notification_template::Model,
        body_template: Option<String>,
        subject_template: Option<String>,
        is_active: Option<bool>,
    ) -> Result<notification_template::Model, AppError> {
        let mut active: notification_template::ActiveModel = model.into();
        if let Some(v) = body_template {
            active.body_template = Set(v);
        }
        if let Some(v) = subject_template {
            active.subject_template = Set(Some(v));
        }
        if let Some(v) = is_active {
            active.is_active = Set(v);
        }
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(self.db).await?)
    }
}
