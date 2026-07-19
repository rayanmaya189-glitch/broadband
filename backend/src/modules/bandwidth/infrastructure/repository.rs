use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};

use crate::modules::bandwidth::domain::entities::{bandwidth_application, bandwidth_profile};
use crate::shared::errors::AppError;

pub struct BandwidthRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> BandwidthRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    // ── Profile CRUD ──────────────────────────────────────────────────

    pub async fn find_profile_by_id(
        &self,
        id: i64,
    ) -> Result<Option<bandwidth_profile::Model>, AppError> {
        Ok(bandwidth_profile::Entity::find_by_id(id)
            .one(self.db)
            .await?)
    }

    pub async fn find_all_active_profiles(
        &self,
    ) -> Result<Vec<bandwidth_profile::Model>, AppError> {
        Ok(bandwidth_profile::Entity::find()
            .filter(bandwidth_profile::Column::IsActive.eq(true))
            .order_by_desc(bandwidth_profile::Column::CreatedAt)
            .all(self.db)
            .await?)
    }

    pub async fn find_profiles_by_plan(
        &self,
        plan_id: i64,
    ) -> Result<Vec<bandwidth_profile::Model>, AppError> {
        Ok(bandwidth_profile::Entity::find()
            .filter(bandwidth_profile::Column::PlanId.eq(plan_id))
            .order_by_desc(bandwidth_profile::Column::CreatedAt)
            .all(self.db)
            .await?)
    }

    pub async fn count_profiles(&self) -> Result<u64, AppError> {
        Ok(bandwidth_profile::Entity::find().count(self.db).await?)
    }

    pub async fn create_profile(
        &self,
        name: String,
        description: Option<String>,
        plan_id: Option<i64>,
        download_kbps: i32,
        upload_kbps: i32,
        burst_download_kbps: Option<i32>,
        burst_upload_kbps: Option<i32>,
        burst_duration_seconds: Option<i32>,
        priority: Option<i32>,
    ) -> Result<bandwidth_profile::Model, AppError> {
        let now = chrono::Utc::now();
        let model = bandwidth_profile::ActiveModel {
            name: Set(name),
            description: Set(description),
            plan_id: Set(plan_id),
            download_kbps: Set(download_kbps),
            upload_kbps: Set(upload_kbps),
            burst_download_kbps: Set(burst_download_kbps),
            burst_upload_kbps: Set(burst_upload_kbps),
            burst_duration_seconds: Set(burst_duration_seconds),
            priority: Set(priority),
            is_active: Set(true),
            review_status: Set(Some("pending".to_string())),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(model.insert(self.db).await?)
    }

    pub async fn update_profile(
        &self,
        model: bandwidth_profile::Model,
        name: Option<String>,
        description: Option<String>,
        download_kbps: Option<i32>,
        upload_kbps: Option<i32>,
        burst_download_kbps: Option<i32>,
        burst_upload_kbps: Option<i32>,
        is_active: Option<bool>,
    ) -> Result<bandwidth_profile::Model, AppError> {
        let mut active: bandwidth_profile::ActiveModel = model.into();
        if let Some(v) = name {
            active.name = Set(v);
        }
        if let Some(v) = description {
            active.description = Set(Some(v));
        }
        if let Some(v) = download_kbps {
            active.download_kbps = Set(v);
        }
        if let Some(v) = upload_kbps {
            active.upload_kbps = Set(v);
        }
        if let Some(v) = burst_download_kbps {
            active.burst_download_kbps = Set(Some(v));
        }
        if let Some(v) = burst_upload_kbps {
            active.burst_upload_kbps = Set(Some(v));
        }
        if let Some(v) = is_active {
            active.is_active = Set(v);
        }
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(self.db).await?)
    }

    pub async fn delete_profile(&self, model: bandwidth_profile::Model) -> Result<(), AppError> {
        let active: bandwidth_profile::ActiveModel = model.into();
        active.delete(self.db).await?;
        Ok(())
    }

    // ── Application CRUD ──────────────────────────────────────────────

    pub async fn find_application_by_id(
        &self,
        id: i64,
    ) -> Result<Option<bandwidth_application::Model>, AppError> {
        Ok(bandwidth_application::Entity::find_by_id(id)
            .one(self.db)
            .await?)
    }

    pub async fn find_applications_by_subscription(
        &self,
        subscription_id: i64,
    ) -> Result<Vec<bandwidth_application::Model>, AppError> {
        Ok(bandwidth_application::Entity::find()
            .filter(bandwidth_application::Column::SubscriptionId.eq(subscription_id))
            .order_by_desc(bandwidth_application::Column::CreatedAt)
            .all(self.db)
            .await?)
    }

    pub async fn find_pending_applications(
        &self,
    ) -> Result<Vec<bandwidth_application::Model>, AppError> {
        Ok(bandwidth_application::Entity::find()
            .filter(bandwidth_application::Column::Status.eq("pending"))
            .order_by_asc(bandwidth_application::Column::CreatedAt)
            .all(self.db)
            .await?)
    }

    pub async fn create_application(
        &self,
        profile_id: i64,
        subscription_id: i64,
        device_id: Option<i64>,
    ) -> Result<bandwidth_application::Model, AppError> {
        let model = bandwidth_application::ActiveModel {
            profile_id: Set(profile_id),
            subscription_id: Set(subscription_id),
            device_id: Set(device_id),
            status: Set("pending".to_string()),
            retry_count: Set(0),
            created_at: Set(chrono::Utc::now()),
            ..Default::default()
        };
        Ok(model.insert(self.db).await?)
    }

    pub async fn update_application_status(
        &self,
        model: bandwidth_application::Model,
        status: String,
        error: Option<String>,
    ) -> Result<bandwidth_application::Model, AppError> {
        let old_retry_count = model.retry_count;
        let mut active: bandwidth_application::ActiveModel = model.into();
        active.status = Set(status.clone());
        if status == "applied" {
            active.applied_at = Set(Some(chrono::Utc::now()));
        }
        if let Some(e) = error {
            active.last_error = Set(Some(e));
        }
        active.retry_count = Set(old_retry_count + 1);
        Ok(active.update(self.db).await?)
    }
}
