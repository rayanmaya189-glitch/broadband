use sqlx::PgPool;
use crate::common::errors::app_error::AppError;
use crate::modules::bandwidth::repository::bandwidth_repository::BandwidthRepository;
use crate::modules::bandwidth::request::bandwidth_request::*;
use crate::modules::bandwidth::response::bandwidth_response::*;

pub struct BandwidthService<'a> { repo: BandwidthRepository<'a> }
impl<'a> BandwidthService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: BandwidthRepository::new(pool) } }

    pub async fn list_profiles(&self, page: i64, per_page: i64) -> Result<BandwidthProfileListResponse, AppError> {
        let (profiles, total) = self.repo.list(page, per_page).await?;
        let responses: Vec<BandwidthProfileResponse> = profiles.iter().map(|p| BandwidthProfileResponse { id: p.id, name: p.name.clone(), description: p.description.clone(), plan_id: p.plan_id, download_kbps: p.download_kbps, upload_kbps: p.upload_kbps, burst_download_kbps: p.burst_download_kbps, burst_upload_kbps: p.burst_upload_kbps, burst_duration_seconds: p.burst_duration_seconds, priority: p.priority, is_active: p.is_active, created_at: p.created_at }).collect();
        Ok(BandwidthProfileListResponse { profiles: responses, total })
    }

    pub async fn get_profile(&self, id: i64) -> Result<BandwidthProfileResponse, AppError> {
        let p = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Profile not found".into()))?;
        Ok(BandwidthProfileResponse { id: p.id, name: p.name, description: p.description, plan_id: p.plan_id, download_kbps: p.download_kbps, upload_kbps: p.upload_kbps, burst_download_kbps: p.burst_download_kbps, burst_upload_kbps: p.burst_upload_kbps, burst_duration_seconds: p.burst_duration_seconds, priority: p.priority, is_active: p.is_active, created_at: p.created_at })
    }

    pub async fn create_profile(&self, req: CreateBandwidthProfileRequest) -> Result<BandwidthProfileResponse, AppError> {
        let p = self.repo.create(&req.name, req.description.as_deref(), req.plan_id, req.download_kbps, req.upload_kbps, req.burst_download_kbps, req.burst_upload_kbps, req.burst_duration_seconds, req.priority).await?;
        Ok(BandwidthProfileResponse { id: p.id, name: p.name, description: p.description, plan_id: p.plan_id, download_kbps: p.download_kbps, upload_kbps: p.upload_kbps, burst_download_kbps: p.burst_download_kbps, burst_upload_kbps: p.burst_upload_kbps, burst_duration_seconds: p.burst_duration_seconds, priority: p.priority, is_active: p.is_active, created_at: p.created_at })
    }

    pub async fn update_profile(&self, id: i64, req: UpdateBandwidthProfileRequest) -> Result<BandwidthProfileResponse, AppError> {
        let p = self.repo.update(id, req.name.as_deref(), req.description.as_deref(), req.download_kbps, req.upload_kbps, req.burst_download_kbps, req.burst_upload_kbps, req.is_active).await.map_err(|_| AppError::NotFound("Profile not found".into()))?;
        Ok(BandwidthProfileResponse { id: p.id, name: p.name, description: p.description, plan_id: p.plan_id, download_kbps: p.download_kbps, upload_kbps: p.upload_kbps, burst_download_kbps: p.burst_download_kbps, burst_upload_kbps: p.burst_upload_kbps, burst_duration_seconds: p.burst_duration_seconds, priority: p.priority, is_active: p.is_active, created_at: p.created_at })
    }

    pub async fn delete_profile(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.delete(id).await? { return Err(AppError::NotFound("Profile not found".into())); }
        Ok(MessageResponse { message: "Profile deleted".into() })
    }

    // ── Apply to Subscription ──────────────────────────────

    pub async fn apply_to_subscription(&self, profile_id: i64, req: ApplyProfileRequest) -> Result<BandwidthApplicationResponse, AppError> {
        let _ = self.repo.get_by_id(profile_id).await?.ok_or_else(|| AppError::NotFound("Profile not found".into()))?;
        let app = self.repo.apply_to_subscription(profile_id, req.subscription_id, req.device_id).await?;
        Ok(BandwidthApplicationResponse { id: app.id, profile_id: app.profile_id, subscription_id: app.subscription_id, device_id: app.device_id, status: app.status, applied_at: app.applied_at, failed_reason: app.failed_reason, retry_count: app.retry_count, created_at: app.created_at })
    }

    pub async fn list_applications(&self, profile_id: Option<i64>, page: i64, per_page: i64) -> Result<Vec<BandwidthApplicationResponse>, AppError> {
        let (apps, _) = self.repo.list_applications(profile_id, page, per_page).await?;
        Ok(apps.iter().map(|a| BandwidthApplicationResponse { id: a.id, profile_id: a.profile_id, subscription_id: a.subscription_id, device_id: a.device_id, status: a.status.clone(), applied_at: a.applied_at, failed_reason: a.failed_reason.clone(), retry_count: a.retry_count, created_at: a.created_at }).collect())
    }

    // ── Usage Tracking ─────────────────────────────────────

    pub async fn get_usage(&self, subscription_id: i64, page: i64, per_page: i64) -> Result<BandwidthUsageResponse, AppError> {
        let (total_download, total_upload) = self.repo.get_usage_summary(subscription_id).await?;
        let (records, _) = self.repo.get_usage(subscription_id, page, per_page).await?;
        Ok(BandwidthUsageResponse {
            subscription_id,
            total_download_bytes: total_download,
            total_upload_bytes: total_upload,
            records: records.iter().map(|r| BandwidthUsageRecord { id: r.id, download_bytes: r.download_bytes, upload_bytes: r.upload_bytes, recorded_at: r.recorded_at }).collect(),
        })
    }
}
