//! SeaORM-based repository for the Notification domain.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};

use crate::common::errors::app_error::AppError;
use crate::modules::notification::model::notification_template_entity::{self, Model as NotificationTemplateModel};
use crate::modules::notification::model::notification_channel_entity::{self, Model as NotificationChannelModel};
use crate::modules::notification::model::notification_entity::{self, Model as NotificationModel};

pub struct NotificationRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> NotificationRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    pub async fn list_templates(&self) -> Result<Vec<NotificationTemplateModel>, AppError> {
        Ok(notification_template_entity::Entity::find().order_by_asc(notification_template_entity::Column::Name).all(self.db).await?)
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

    pub async fn list_channels(&self) -> Result<Vec<NotificationChannelModel>, AppError> {
        Ok(notification_channel_entity::Entity::find().order_by_asc(notification_channel_entity::Column::Channel).all(self.db).await?)
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

    pub async fn send(&self, channel: &str, recipient_id: i64, address: &str, subject: Option<&str>, body: &str) -> Result<NotificationModel, AppError> {
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
}
