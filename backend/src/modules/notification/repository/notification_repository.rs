//! SeaORM-based repository for the Notification domain.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection,
    EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use chrono::{DateTime, Utc};

use crate::common::errors::app_error::AppError;
use crate::modules::notification::model::notification_channel_entity::{self, Model as NotificationChannelModel};
use crate::modules::notification::model::notification_entity::{self, Model as NotificationModel};
use crate::modules::notification::model::notification_template_entity::{self, Model as NotificationTemplateModel};

/// Row type for notification history queries (no dedicated entity).
#[derive(Debug, Clone)]
pub struct NotificationHistoryRow {
    pub id: i64,
    pub notification_id: i64,
    pub event: String,
    pub details: Option<serde_json::Value>,
    pub recorded_at: DateTime<Utc>,
}

pub struct NotificationRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> NotificationRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    // ── Templates ─────────────────────────────────────────

    pub async fn list_templates(&self) -> Result<Vec<NotificationTemplateModel>, AppError> {
        Ok(notification_template_entity::Entity::find()
            .order_by_asc(notification_template_entity::Column::Name)
            .all(self.db).await?)
    }

    pub async fn get_template(&self, id: i64) -> Result<Option<NotificationTemplateModel>, AppError> {
        Ok(notification_template_entity::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn create_template(&self, name: &str, channel: &str, subject: Option<&str>, body: &str) -> Result<NotificationTemplateModel, AppError> {
        let now = chrono::Utc::now();
        let active = notification_template_entity::ActiveModel {
            name: Set(name.to_owned()),
            channel: Set(channel.to_owned()),
            subject_template: Set(subject.map(|s| s.to_owned())),
            body_template: Set(body.to_owned()),
            is_active: Set(true),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn update_template(&self, id: i64, name: Option<&str>, channel: Option<&str>, subject: Option<&str>, body: Option<&str>) -> Result<NotificationTemplateModel, AppError> {
        let existing = notification_template_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Template not found".into()))?;
        let mut active = existing.into_active_model();
        if let Some(v) = name { active.name = Set(v.to_owned()); }
        if let Some(v) = channel { active.channel = Set(v.to_owned()); }
        if let Some(v) = subject { active.subject_template = Set(Some(v.to_owned())); }
        if let Some(v) = body { active.body_template = Set(v.to_owned()); }
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn delete_template(&self, id: i64) -> Result<bool, AppError> {
        let result = notification_template_entity::Entity::delete_by_id(id).exec(self.db).await?;
        Ok(result.rows_affected > 0)
    }

    // ── Channels ──────────────────────────────────────────

    pub async fn list_channels(&self) -> Result<Vec<NotificationChannelModel>, AppError> {
        Ok(notification_channel_entity::Entity::find()
            .order_by_asc(notification_channel_entity::Column::Channel)
            .all(self.db).await?)
    }

    pub async fn get_channel(&self, id: i64) -> Result<Option<NotificationChannelModel>, AppError> {
        Ok(notification_channel_entity::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn upsert_channel(&self, channel: &str, provider: &str, config: serde_json::Value) -> Result<NotificationChannelModel, AppError> {
        let existing = notification_channel_entity::Entity::find()
            .filter(notification_channel_entity::Column::Channel.eq(channel))
            .one(self.db).await?;
        match existing {
            Some(e) => {
                let mut active = e.into_active_model();
                active.provider = Set(provider.to_owned());
                active.config = Set(config);
                active.updated_at = Set(chrono::Utc::now().into());
                Ok(active.update(self.db).await?)
            }
            None => {
                let now = chrono::Utc::now();
                let active = notification_channel_entity::ActiveModel {
                    channel: Set(channel.to_owned()),
                    provider: Set(provider.to_owned()),
                    config: Set(config),
                    is_active: Set(true),
                    created_at: Set(now.into()),
                    updated_at: Set(now.into()),
                    ..Default::default()
                };
                Ok(active.insert(self.db).await?)
            }
        }
    }

    // ── Notifications ─────────────────────────────────────

    pub async fn send(&self, channel: &str, _recipient_id: i64, _address: &str, subject: Option<&str>, body: &str) -> Result<NotificationModel, AppError> {
        let now = chrono::Utc::now();
        let active = notification_entity::ActiveModel {
            r#type: Set("direct".to_owned()),
            channel: Set(channel.to_owned()),
            title: Set(subject.map(|s| s.to_owned())),
            body: Set(Some(body.to_owned())),
            status: Set("queued".to_owned()),
            created_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn list_notifications(&self, channel: Option<&str>, status: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<NotificationModel>, i64), AppError> {
        let page_size = per_page as u64;
        let page_num = if per_page > 0 { ((page - 1).max(0) as u64 * page_size) / page_size } else { 0 };
        let mut select = notification_entity::Entity::find();
        if let Some(ch) = channel { select = select.filter(notification_entity::Column::Channel.eq(ch)); }
        if let Some(s) = status { select = select.filter(notification_entity::Column::Status.eq(s)); }
        let total = select.clone().count(self.db).await?;
        let notifications = select.order_by_desc(notification_entity::Column::CreatedAt).paginate(self.db, page_size).fetch_page(page_num).await?;
        Ok((notifications, total as i64))
    }

    /// Reset a failed notification's status to "queued" for retry.
    pub async fn retry_notification(&self, id: i64) -> Result<bool, AppError> {
        let existing = notification_entity::Entity::find_by_id(id).one(self.db).await?;
        match existing {
            Some(n) if n.status == "failed" => {
                let mut active = n.into_active_model();
                active.status = Set("queued".to_owned());
                active.update(self.db).await?;
                Ok(true)
            }
            Some(_) => Ok(false), // notification exists but is not in "failed" status
            None => Ok(false),
        }
    }

    /// Query notification history using pure SeaORM queries with app-level combination.
    ///
    /// Derives history events from the notifications table:
    /// - Every notification produces a 'created' event
    /// - Notifications with status in (sent, delivered, failed) also produce a 'status_changed' event
    ///
    /// Returns paginated history records with optional notification_id filter.
    pub async fn list_history(
        &self,
        notification_id: Option<i64>,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<NotificationHistoryRow>, i64), AppError> {
        let status_events = ["sent", "delivered", "failed"];

        // 1. Fetch relevant notifications using SeaORM
        let mut select = notification_entity::Entity::find();
        if let Some(nid) = notification_id {
            select = select.filter(notification_entity::Column::Id.eq(nid));
        }
        let notifications = select
            .order_by_desc(notification_entity::Column::CreatedAt)
            .all(self.db).await?;

        // 2. Build history rows in application code (simulates UNION ALL)
        let mut all_events: Vec<NotificationHistoryRow> = Vec::new();
        for n in &notifications {
            // 'created' event for every notification
            all_events.push(NotificationHistoryRow {
                id: n.id,
                notification_id: n.id,
                event: "created".to_owned(),
                details: None,
                recorded_at: n.created_at.into(),
            });
            // 'status_changed' event for sent/delivered/failed
            if status_events.contains(&n.status.as_str()) {
                all_events.push(NotificationHistoryRow {
                    id: n.id,
                    notification_id: n.id,
                    event: "status_changed".to_owned(),
                    details: Some(serde_json::json!({"status": n.status})),
                    recorded_at: n.created_at.into(),
                });
            }
        }

        // 3. Sort by recorded_at descending
        all_events.sort_by(|a, b| b.recorded_at.cmp(&a.recorded_at));

        // 4. Compute total and paginate
        let total = all_events.len() as i64;
        let start = ((page - 1).max(0) as usize) * per_page as usize;
        let end = (start + per_page as usize).min(all_events.len());
        let page_events = if start < all_events.len() {
            all_events[start..end].to_vec()
        } else {
            Vec::new()
        };

        Ok((page_events, total))
    }
}
