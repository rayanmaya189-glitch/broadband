use crate::modules::bandwidth::domain::entities::{BandwidthProfile, BandwidthProfileActiveModel};
use crate::shared::errors::AppError;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

pub struct BandwidthService;

impl BandwidthService {
    pub async fn list_profiles(
        db: &DatabaseConnection,
    ) -> Result<Vec<crate::modules::bandwidth::domain::entities::bandwidth_profile::Model>, AppError>
    {
        Ok(BandwidthProfile::find().all(db).await?)
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
}
