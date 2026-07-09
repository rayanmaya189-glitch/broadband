use sqlx::PgPool;
use crate::common::errors::app_error::AppError;
use crate::modules::notification::repository::notification_repository::NotificationRepository;
use crate::modules::notification::request::notification_request::*;
use crate::modules::notification::response::notification_response::*;

pub struct NotificationService<'a> {
    repo: NotificationRepository<'a>,
}

impl<'a> NotificationService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: NotificationRepository::new(pool) } }

    // ── Templates ──────────────────────────────────────────

    pub async fn list_templates(&self) -> Result<Vec<TemplateResponse>, AppError> {
        let t = self.repo.list_templates().await?;
        Ok(t.iter().map(|x| TemplateResponse { id: x.id, name: x.name.clone(), channel: x.channel.clone(), is_active: x.is_active, created_at: x.created_at }).collect())
    }

    pub async fn create_template(&self, req: CreateTemplateRequest) -> Result<TemplateResponse, AppError> {
        let t = self.repo.create_template(&req.name, &req.channel, req.subject_template.as_deref(), &req.body_template).await?;
        Ok(TemplateResponse { id: t.id, name: t.name, channel: t.channel, is_active: t.is_active, created_at: t.created_at })
    }

    pub async fn update_template(&self, id: i64, req: UpdateTemplateRequest) -> Result<TemplateResponse, AppError> {
        let t = self.repo.update_template(id, req.name.as_deref(), req.channel.as_deref(), req.subject_template.as_deref(), req.body_template.as_deref()).await.map_err(|_| AppError::NotFound("Template not found".into()))?;
        Ok(TemplateResponse { id: t.id, name: t.name, channel: t.channel, is_active: t.is_active, created_at: t.created_at })
    }

    pub async fn delete_template(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.delete_template(id).await? { return Err(AppError::NotFound("Template not found".into())); }
        Ok(MessageResponse { message: "Template deleted".into() })
    }

    // ── Channels ───────────────────────────────────────────

    pub async fn list_channels(&self) -> Result<Vec<ChannelResponse>, AppError> {
        let channels = self.repo.list_channels().await?;
        Ok(channels.iter().map(|c| ChannelResponse { id: c.id, channel: c.channel.clone(), provider: c.provider.clone(), config: c.config.clone(), is_active: c.is_active, created_at: c.created_at, updated_at: c.updated_at }).collect())
    }

    pub async fn upsert_channel(&self, req: UpsertChannelRequest) -> Result<ChannelResponse, AppError> {
        let c = self.repo.upsert_channel(&req.channel, &req.provider, req.config).await?;
        Ok(ChannelResponse { id: c.id, channel: c.channel, provider: c.provider, config: c.config, is_active: c.is_active, created_at: c.created_at, updated_at: c.updated_at })
    }

    // ── Send ───────────────────────────────────────────────

    pub async fn send(&self, req: SendNotificationRequest) -> Result<NotificationResponse, AppError> {
        let n = self.repo.send(&req.channel, req.recipient_id, &req.recipient_address, req.subject.as_deref(), &req.body).await?;
        Ok(NotificationResponse { id: n.id, channel: n.channel, recipient_address: n.recipient_address, status: n.status, created_at: n.created_at })
    }

    pub async fn list_notifications(&self, query: NotificationQuery) -> Result<NotificationListResponse, AppError> {
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(20);
        let (notifications, total) = self.repo.list_notifications(query.channel.as_deref(), query.status.as_deref(), page, per_page).await?;
        let responses: Vec<NotificationDetailResponse> = notifications.iter().map(|n| NotificationDetailResponse { id: n.id, template_id: n.template_id, channel: n.channel.clone(), recipient_type: n.recipient_type.clone(), recipient_id: n.recipient_id, recipient_address: n.recipient_address.clone(), subject: n.subject.clone(), body: n.body.clone(), status: n.status.clone(), retry_count: n.retry_count, max_retries: n.max_retries, last_error: n.last_error.clone(), sent_at: n.sent_at, delivered_at: n.delivered_at, created_at: n.created_at }).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok(NotificationListResponse { notifications: responses, total, page, per_page, total_pages })
    }

    pub async fn retry_notification(&self, id: i64) -> Result<NotificationDetailResponse, AppError> {
        let n = self.repo.retry_notification(id).await.map_err(|_| AppError::NotFound("Notification not found or max retries reached".into()))?;
        Ok(NotificationDetailResponse { id: n.id, template_id: n.template_id, channel: n.channel, recipient_type: n.recipient_type, recipient_id: n.recipient_id, recipient_address: n.recipient_address, subject: n.subject, body: n.body, status: n.status, retry_count: n.retry_count, max_retries: n.max_retries, last_error: n.last_error, sent_at: n.sent_at, delivered_at: n.delivered_at, created_at: n.created_at })
    }

    // ── History ────────────────────────────────────────────

    pub async fn list_history(&self, query: HistoryQuery) -> Result<HistoryListResponse, AppError> {
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(20);
        let (history, total) = self.repo.list_history(query.notification_id, page, per_page).await?;
        let responses: Vec<HistoryResponse> = history.iter().map(|h| HistoryResponse { id: h.id, notification_id: h.notification_id, event: h.event.clone(), details: h.details.clone(), recorded_at: h.recorded_at }).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok(HistoryListResponse { history: responses, total, page, per_page, total_pages })
    }
}
