use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter, QueryOrder, Set};

use crate::common::errors::app_error::AppError;
use crate::modules::bandwidth::model::bandwidth_profile::{self as bandwidth_profile_entity, Model as BandwidthProfile};
use crate::modules::bandwidth::model::bandwidth_application::{self as bandwidth_application_entity, Model as BandwidthApplication};
use crate::modules::bandwidth::model::bandwidth_usage::{self as bandwidth_usage_entity, Model as BandwidthUsage};

pub struct BandwidthRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> BandwidthRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    // ── Profiles ────────────────────────────────────────────

    pub async fn list(&self, page: i64, per_page: i64) -> Result<(Vec<BandwidthProfile>, i64), AppError> {
        let page_size = per_page.max(1) as u64;
        let page_num = page.max(1) as u64;

        let paginator = bandwidth_profile_entity::Entity::find()
            .order_by_desc(bandwidth_profile_entity::Column::CreatedAt)
            .paginate(self.db, page_size);

        let total = paginator.num_items().await? as i64;
        let profiles = paginator.fetch_page(page_num - 1).await?;
        Ok((profiles, total))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<BandwidthProfile>, AppError> {
        Ok(bandwidth_profile_entity::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn create(
        &self, name: &str, description: Option<&str>, plan_id: Option<i64>,
        download: i32, upload: i32, burst_down: Option<i32>, burst_up: Option<i32>,
        burst_dur: Option<i32>, priority: Option<i32>,
    ) -> Result<BandwidthProfile, AppError> {
        let now = chrono::Utc::now();
        let active = bandwidth_profile_entity::ActiveModel {
            name: Set(name.to_owned()),
            description: Set(description.map(|s| s.to_owned())),
            plan_id: Set(plan_id),
            download_kbps: Set(download),
            upload_kbps: Set(upload),
            burst_download_kbps: Set(burst_down),
            burst_upload_kbps: Set(burst_up),
            burst_duration_seconds: Set(burst_dur),
            priority: Set(priority),
            is_active: Set(true),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn update(
        &self, id: i64, name: Option<&str>, description: Option<&str>,
        download: Option<i32>, upload: Option<i32>, burst_down: Option<i32>,
        burst_up: Option<i32>, is_active: Option<bool>,
    ) -> Result<BandwidthProfile, AppError> {
        let existing = bandwidth_profile_entity::Entity::find_by_id(id)
            .one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Profile not found".into()))?;
        let mut active = existing.into_active_model();
        if let Some(v) = name { active.name = Set(v.to_owned()); }
        if let Some(v) = description { active.description = Set(Some(v.to_owned())); }
        if let Some(v) = download { active.download_kbps = Set(v); }
        if let Some(v) = upload { active.upload_kbps = Set(v); }
        if let Some(v) = burst_down { active.burst_download_kbps = Set(Some(v)); }
        if let Some(v) = burst_up { active.burst_upload_kbps = Set(Some(v)); }
        if let Some(v) = is_active { active.is_active = Set(v); }
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn delete(&self, id: i64) -> Result<bool, AppError> {
        let result = bandwidth_profile_entity::Entity::delete_by_id(id).exec(self.db).await?;
        Ok(result.rows_affected > 0)
    }

    // ── Apply to Subscription ──────────────────────────────

    pub async fn apply_to_subscription(
        &self, profile_id: i64, subscription_id: i64, device_id: i64,
    ) -> Result<BandwidthApplication, AppError> {
        let now = chrono::Utc::now();
        let active = bandwidth_application_entity::ActiveModel {
            profile_id: Set(profile_id),
            subscription_id: Set(subscription_id),
            device_id: Set(device_id),
            status: Set("pending".to_owned()),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn list_applications(
        &self, profile_id: Option<i64>, page: i64, per_page: i64,
    ) -> Result<(Vec<BandwidthApplication>, i64), AppError> {
        let page_size = per_page.max(1) as u64;
        let page_num = page.max(1) as u64;

        let mut select = bandwidth_application_entity::Entity::find();
        if let Some(pid) = profile_id {
            select = select.filter(bandwidth_application_entity::Column::ProfileId.eq(pid));
        }

        let paginator = select
            .order_by_desc(bandwidth_application_entity::Column::CreatedAt)
            .paginate(self.db, page_size);

        let total = paginator.num_items().await? as i64;
        let apps = paginator.fetch_page(page_num - 1).await?;
        Ok((apps, total))
    }

    pub async fn update_application_status(
        &self, id: i64, status: &str, failed_reason: Option<&str>,
    ) -> Result<BandwidthApplication, AppError> {
        let existing = bandwidth_application_entity::Entity::find_by_id(id)
            .one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Application not found".into()))?;
        let mut active = existing.into_active_model();
        active.status = Set(status.to_owned());
        active.failed_reason = Set(failed_reason.map(|s| s.to_owned()));
        if status == "applied" {
            active.applied_at = Set(Some(chrono::Utc::now().into()));
        }
        if status == "failed" {
            let current = active.retry_count.clone().unwrap();
            active.retry_count = Set(current + 1);
        }
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    // ── Usage Tracking ─────────────────────────────────────

    pub async fn get_usage(
        &self, subscription_id: i64, page: i64, per_page: i64,
    ) -> Result<(Vec<BandwidthUsage>, i64), AppError> {
        let page_size = per_page.max(1) as u64;
        let page_num = page.max(1) as u64;

        let select = bandwidth_usage_entity::Entity::find()
            .filter(bandwidth_usage_entity::Column::SubscriptionId.eq(subscription_id));

        let paginator = select
            .order_by_desc(bandwidth_usage_entity::Column::RecordedAt)
            .paginate(self.db, page_size);

        let total = paginator.num_items().await? as i64;
        let usage = paginator.fetch_page(page_num - 1).await?;
        Ok((usage, total))
    }

    pub async fn get_usage_summary(&self, subscription_id: i64) -> Result<(i64, i64), AppError> {
        let usage = bandwidth_usage_entity::Entity::find()
            .filter(bandwidth_usage_entity::Column::SubscriptionId.eq(subscription_id))
            .all(self.db).await?;
        let total_download: i64 = usage.iter().map(|u| u.download_bytes).sum();
        let total_upload: i64 = usage.iter().map(|u| u.upload_bytes).sum();
        Ok((total_download, total_upload))
    }
}
