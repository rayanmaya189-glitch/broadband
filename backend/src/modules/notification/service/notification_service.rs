//! SeaORM-based service for the Notification domain.

use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::modules::notification::repository::notification_repository::NotificationRepository;
use crate::modules::notification::request::notification_request::*;
use crate::modules::notification::response::notification_response::*;

pub struct NotificationService<'a> {
    repo: NotificationRepository<'a>,
}

impl<'a> NotificationService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { repo: NotificationRepository::new(db) }
    }

    pub async fn list_templates(&self) -> Result<Vec<NotificationTemplateResponse>, AppError> {
        let templates = self.repo.list_templates().await?;
        Ok(templates.into_iter().map(|t| NotificationTemplateResponse {
            id: t.id, name: t.name, channel: t.channel, subject_template: t.subject_template,
            body_template: t.body_template, is_active: t.is_active,
            created_at: t.created_at.into(), updated_at: t.updated_at.into(),
        }).collect())
    }

    pub async fn create_template(&self, req: CreateNotificationTemplateRequest) -> Result<NotificationTemplateResponse, AppError> {
        let t = self.repo.create_template(&req.name, &req.channel, req.subject_template.as_deref(), &req.body_template).await?;
        Ok(NotificationTemplateResponse {
            id: t.id, name: t.name, channel: t.channel, subject_template: t.subject_template,
            body_template: t.body_template, is_active: t.is_active,
            created_at: t.created_at.into(), updated_at: t.updated_at.into(),
        })
    }

    pub async fn update_template(&self, id: i64, req: UpdateTemplateRequest) -> Result<NotificationTemplateResponse, AppError> {
        let t = self.repo.update_template(id, req.name.as_deref(), req.channel.as_deref(), req.subject_template.as_deref(), req.body_template.as_deref()).await?;
        Ok(NotificationTemplateResponse {
            id: t.id, name: t.name, channel: t.channel, subject_template: t.subject_template,
            body_template: t.body_template, is_active: t.is_active,
            created_at: t.created_at.into(), updated_at: t.updated_at.into(),
        })
    }

    pub async fn delete_template(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.delete_template(id).await? {
            return Err(AppError::NotFound("Template not found".into()));
        }
        Ok(MessageResponse { message: "Template deleted".into() })
    }

    pub async fn list_channels(&self) -> Result<Vec<NotificationChannelResponse>, AppError> {
        let channels = self.repo.list_channels().await?;
        Ok(channels.into_iter().map(|c| NotificationChannelResponse {
            id: c.id, channel: c.channel, provider: c.provider, config: c.config,
            is_active: c.is_active, created_at: c.created_at.into(), updated_at: c.updated_at.into(),
        }).collect())
    }

    pub async fn upsert_channel(&self, channel: &str, provider: &str, config: serde_json::Value) -> Result<NotificationChannelResponse, AppError> {
        let c = self.repo.upsert_channel(channel, provider, config).await?;
        Ok(NotificationChannelResponse {
            id: c.id, channel: c.channel, provider: c.provider, config: c.config,
            is_active: c.is_active, created_at: c.created_at.into(), updated_at: c.updated_at.into(),
        })
    }

    pub async fn send(&self, channel: &str, recipient_id: i64, address: &str, subject: Option<&str>, body: &str) -> Result<NotificationResponse, AppError> {
        let n = self.repo.send(channel, recipient_id, address, subject, body).await?;
        Ok(NotificationResponse {
            id: n.id, channel: n.channel, title: n.title, body: n.body,
            status: n.status, created_at: n.created_at.into(),
        })
    }

    pub async fn list_notifications(&self, channel: Option<&str>, status: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<NotificationResponse>, i64), AppError> {
        let (notifications, total) = self.repo.list_notifications(channel, status, page, per_page).await?;
        let responses = notifications.into_iter().map(|n| NotificationResponse {
            id: n.id, channel: n.channel, title: n.title, body: n.body,
            status: n.status, created_at: n.created_at.into(),
        }).collect();
        Ok((responses, total))
    }

    /// Reset a failed notification's status to "queued" so it can be retried.
    pub async fn retry_notification(&self, id: i64) -> Result<MessageResponse, AppError> {
        match self.repo.retry_notification(id).await? {
            true => Ok(MessageResponse { message: "Notification queued for retry".into() }),
            false => Err(AppError::NotFound("Notification not found or not in failed status".into())),
        }
    }

    /// Query notification history with optional notification_id filter and pagination.
    pub async fn list_history(
        &self,
        notification_id: Option<i64>,
        page: i64,
        per_page: i64,
    ) -> Result<HistoryListResponse, AppError> {
        let (rows, total) = self.repo.list_history(notification_id, page, per_page).await?;
        let history: Vec<HistoryResponse> = rows.into_iter().map(|r| HistoryResponse {
            id: r.id, notification_id: r.notification_id, event: r.event,
            details: r.details, recorded_at: r.recorded_at,
        }).collect();
        let total_pages = if per_page > 0 { (total as f64 / per_page as f64).ceil() as i64 } else { 0 };
        Ok(HistoryListResponse { history, total, page, per_page, total_pages })
    }
}
