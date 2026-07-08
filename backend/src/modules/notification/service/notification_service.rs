use sqlx::PgPool;
use crate::common::errors::app_error::AppError;
use crate::modules::notification::repository::notification_repository::NotificationRepository;
use crate::modules::notification::request::notification_request::*;
use crate::modules::notification::response::notification_response::*;

pub struct NotificationService<'a> {
    repo: NotificationRepository<'a>,
}

impl<'a> NotificationService<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { repo: NotificationRepository::new(pool) }
    }

    pub async fn list_templates(&self) -> Result<Vec<TemplateResponse>, AppError> {
        let t = self.repo.list_templates().await?;
        Ok(t.iter().map(|x| TemplateResponse {
            id: x.id,
            name: x.name.clone(),
            channel: x.channel.clone(),
            is_active: x.is_active,
            created_at: x.created_at,
        }).collect())
    }

    pub async fn create_template(&self, req: CreateTemplateRequest) -> Result<TemplateResponse, AppError> {
        let t = self.repo.create_template(&req.name, &req.channel, req.subject_template.as_deref(), &req.body_template).await?;
        Ok(TemplateResponse {
            id: t.id,
            name: t.name,
            channel: t.channel,
            is_active: t.is_active,
            created_at: t.created_at,
        })
    }

    pub async fn send(&self, req: SendNotificationRequest) -> Result<NotificationResponse, AppError> {
        let n = self.repo.send(&req.channel, req.recipient_id, &req.recipient_address, req.subject.as_deref(), &req.body).await?;
        Ok(NotificationResponse {
            id: n.id,
            channel: n.channel,
            recipient_address: n.recipient_address,
            status: n.status,
            created_at: n.created_at,
        })
    }
}
