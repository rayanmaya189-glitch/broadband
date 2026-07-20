use crate::modules::bandwidth::domain::entities::{
    BandwidthProfile, BandwidthProfileActiveModel,
};
use crate::shared::errors::AppError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};

pub struct BandwidthService;

impl BandwidthService {
    pub async fn list_profiles(
        db: &DatabaseConnection,
        _page: u64,
        _limit: u64,
    ) -> Result<
        (
            Vec<crate::modules::bandwidth::domain::entities::bandwidth_profile::Model>,
            u64,
        ),
        AppError,
    > {
        let q = BandwidthProfile::find();
        let t = q.clone().count(db).await?;
        Ok((q.all(db).await?, t))
    }

    pub async fn get_profile(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<crate::modules::bandwidth::domain::entities::bandwidth_profile::Model, AppError>
    {
        BandwidthProfile::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Profile {} not found", id)))
    }

    pub async fn create_profile(
        db: &DatabaseConnection,
        name: String,
        download_kbps: i32,
        upload_kbps: i32,
    ) -> Result<crate::modules::bandwidth::domain::entities::bandwidth_profile::Model, AppError>
    {
        let now = chrono::Utc::now();
        let profile = BandwidthProfileActiveModel {
            name: Set(name),
            download_kbps: Set(download_kbps),
            upload_kbps: Set(upload_kbps),
            is_active: Set(true),
            review_status: Set(Some("pending".to_string())),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(profile.insert(db).await?)
    }

    pub async fn update_profile(
        db: &DatabaseConnection,
        id: i64,
        name: Option<String>,
        download_kbps: Option<i32>,
        upload_kbps: Option<i32>,
    ) -> Result<crate::modules::bandwidth::domain::entities::bandwidth_profile::Model, AppError>
    {
        let profile = Self::get_profile(db, id).await?;
        let mut active = <crate::modules::bandwidth::domain::entities::bandwidth_profile::Entity as sea_orm::EntityTrait>::ActiveModel::from(profile);
        if let Some(n) = name {
            active.name = Set(n);
        }
        if let Some(d) = download_kbps {
            active.download_kbps = Set(d);
        }
        if let Some(u) = upload_kbps {
            active.upload_kbps = Set(u);
        }
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(db).await?)
    }

    pub async fn delete_profile(db: &DatabaseConnection, id: i64) -> Result<(), AppError> {
        let profile = Self::get_profile(db, id).await?;
        let mut active = <crate::modules::bandwidth::domain::entities::bandwidth_profile::Entity as sea_orm::EntityTrait>::ActiveModel::from(profile);
        active.is_active = Set(false);
        active.updated_at = Set(chrono::Utc::now());
        active.update(db).await?;
        Ok(())
    }

    // ─── Policies ─────────────────────────────────────────────────────

    pub async fn list_policies(
        db: &DatabaseConnection,
    ) -> Result<Vec<crate::modules::bandwidth::domain::entities::bandwidth_policy::Model>, AppError> {
        use crate::modules::bandwidth::domain::entities::BandwidthPolicy;
        use crate::modules::bandwidth::domain::entities::bandwidth_policy::Column;
        let items = BandwidthPolicy::find()
            .order_by_desc(Column::Priority)
            .all(db)
            .await?;
        Ok(items)
    }

    pub async fn create_policy(
        db: &DatabaseConnection,
        name: String,
        policy_type: String,
        config: serde_json::Value,
        priority: i32,
    ) -> Result<crate::modules::bandwidth::domain::entities::bandwidth_policy::Model, AppError> {
        use crate::modules::bandwidth::domain::entities::BandwidthPolicyActiveModel;
        let now = chrono::Utc::now();
        let policy = BandwidthPolicyActiveModel {
            name: Set(name),
            policy_type: Set(policy_type),
            config: Set(config),
            priority: Set(priority),
            is_active: Set(true),
            review_status: Set(Some("pending".to_string())),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(policy.insert(db).await?)
    }

    pub async fn update_policy(
        db: &DatabaseConnection,
        id: i64,
        name: Option<String>,
        config: Option<serde_json::Value>,
        priority: Option<i32>,
        is_active: Option<bool>,
    ) -> Result<crate::modules::bandwidth::domain::entities::bandwidth_policy::Model, AppError> {
        use crate::modules::bandwidth::domain::entities::{BandwidthPolicy, bandwidth_policy};
        let existing = BandwidthPolicy::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Policy {} not found", id)))?;
        let mut active: bandwidth_policy::ActiveModel = existing.into();
        if let Some(n) = name { active.name = Set(n); }
        if let Some(c) = config { active.config = Set(c); }
        if let Some(p) = priority { active.priority = Set(p); }
        if let Some(a) = is_active { active.is_active = Set(a); }
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(db).await?)
    }

    pub async fn delete_policy(db: &DatabaseConnection, id: i64) -> Result<(), AppError> {
        use crate::modules::bandwidth::domain::entities::{BandwidthPolicy, bandwidth_policy};
        let existing = BandwidthPolicy::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Policy {} not found", id)))?;
        let mut active: bandwidth_policy::ActiveModel = existing.into();
        active.is_active = Set(false);
        active.updated_at = Set(chrono::Utc::now());
        active.update(db).await?;
        Ok(())
    }

    // ─── Apply Profile ──────────────────────────────────────────────────

    pub async fn apply_profile(
        db: &DatabaseConnection,
        profile_id: i64,
    ) -> Result<u64, AppError> {
        Self::get_profile(db, profile_id).await?;
        let applications = crate::modules::bandwidth::domain::entities::BandwidthApplication::find()
            .filter(
                crate::modules::bandwidth::domain::entities::bandwidth_application::Column::ProfileId
                    .eq(profile_id),
            )
            .all(db)
            .await?;
        let now = chrono::Utc::now();
        let mut updated = 0u64;
        for app in applications {
            let mut active: crate::modules::bandwidth::domain::entities::bandwidth_application::ActiveModel = app.into();
            active.status = Set("applied".to_string());
            active.applied_at = Set(Some(now));
            active.update(db).await?;
            updated += 1;
        }
        Ok(updated)
    }

    pub async fn apply_to_subscription(
        db: &DatabaseConnection,
        subscription_id: i64,
        profile_id: i64,
    ) -> Result<crate::modules::bandwidth::domain::entities::bandwidth_application::Model, AppError>
    {
        Self::get_profile(db, profile_id).await?;
        let now = chrono::Utc::now();
        let active = crate::modules::bandwidth::domain::entities::bandwidth_application::ActiveModel
        {
            profile_id: Set(profile_id),
            subscription_id: Set(subscription_id),
            status: Set("applied".to_string()),
            applied_at: Set(Some(now)),
            retry_count: Set(0),
            created_at: Set(now),
            ..Default::default()
        };
        let result = active.insert(db).await?;
        Ok(result)
    }

    // ─── Applications ───────────────────────────────────────────────────

    pub async fn list_applications(
        db: &DatabaseConnection,
    ) -> Result<
        Vec<crate::modules::bandwidth::domain::entities::bandwidth_application::Model>,
        AppError,
    > {
        let items = crate::modules::bandwidth::domain::entities::BandwidthApplication::find()
            .order_by_desc(
                crate::modules::bandwidth::domain::entities::bandwidth_application::Column::CreatedAt,
            )
            .all(db)
            .await?;
        Ok(items)
    }

    // ─── Usage ──────────────────────────────────────────────────────────

    pub async fn get_usage(
        db: &DatabaseConnection,
        subscription_id: i64,
    ) -> Result<serde_json::Value, AppError> {
        let apps = crate::modules::bandwidth::domain::entities::BandwidthApplication::find()
            .filter(
                crate::modules::bandwidth::domain::entities::bandwidth_application::Column::SubscriptionId
                    .eq(subscription_id),
            )
            .order_by_desc(
                crate::modules::bandwidth::domain::entities::bandwidth_application::Column::CreatedAt,
            )
            .all(db)
            .await?;

        Ok(serde_json::json!({
            "subscription_id": subscription_id,
            "applications": apps.into_iter().map(|a| serde_json::json!({
                "id": a.id,
                "profile_id": a.profile_id,
                "status": a.status,
                "applied_at": a.applied_at,
                "created_at": a.created_at,
            })).collect::<Vec<_>>(),
        }))
    }
}
