use axum::extract::{Json, Path, Query, State};
use validator::Validate;
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::bandwidth::request::bandwidth_request::*;
use crate::modules::bandwidth::response::bandwidth_response::*;
use crate::modules::bandwidth::service::bandwidth_service::BandwidthService;

// ── Profiles ────────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/bandwidth/profiles",
    tag = "Bandwidth",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "List of bandwidth profiles"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_profiles(State(state): State<SharedState>) -> Result<Json<BandwidthProfileListResponse>, AppError> {
    let svc = BandwidthService::new(&state.db);
    Ok(Json(svc.list_profiles(1, 100).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/bandwidth/profiles/{id}",
    tag = "Bandwidth",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Profile ID")),
    responses(
        (status = 200, description = "Profile details", body = BandwidthProfileResponse),
        (status = 404, description = "Profile not found")
    )
)]
pub async fn get_profile(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<BandwidthProfileResponse>, AppError> {
    let svc = BandwidthService::new(&state.db);
    Ok(Json(svc.get_profile(id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/bandwidth/profiles",
    tag = "Bandwidth",
    security(("bearer_auth" = [])),
    request_body = CreateBandwidthProfileRequest,
    responses(
        (status = 200, description = "Profile created", body = BandwidthProfileResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_profile(State(state): State<SharedState>, Json(req): Json<CreateBandwidthProfileRequest>) -> Result<Json<BandwidthProfileResponse>, AppError> {
    req.validate()?;
    let svc = BandwidthService::new(&state.db);
    Ok(Json(svc.create_profile(req).await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/bandwidth/profiles/{id}",
    tag = "Bandwidth",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Profile ID")),
    request_body = UpdateBandwidthProfileRequest,
    responses(
        (status = 200, description = "Profile updated", body = BandwidthProfileResponse),
        (status = 404, description = "Profile not found")
    )
)]
pub async fn update_profile(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateBandwidthProfileRequest>) -> Result<Json<BandwidthProfileResponse>, AppError> {
    let svc = BandwidthService::new(&state.db);
    Ok(Json(svc.update_profile(id, req).await?))
}

#[utoipa::path(
    delete,
    path = "/api/v1/bandwidth/profiles/{id}",
    tag = "Bandwidth",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Profile ID")),
    responses(
        (status = 200, description = "Profile deleted"),
        (status = 404, description = "Profile not found")
    )
)]
pub async fn delete_profile(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = BandwidthService::new(&state.db);
    Ok(Json(svc.delete_profile(id).await?))
}

// ── Apply to Subscription ───────────────────────────────────

#[utoipa::path(
    post,
    path = "/api/v1/bandwidth/apply/{subscription_id}",
    tag = "Bandwidth",
    security(("bearer_auth" = [])),
    params(("subscription_id" = i64, Path, description = "Subscription ID")),
    request_body = ApplyProfileRequest,
    responses(
        (status = 200, description = "Profile applied", body = BandwidthApplicationResponse),
        (status = 404, description = "Subscription not found")
    )
)]
pub async fn apply_to_subscription(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<ApplyProfileRequest>) -> Result<Json<BandwidthApplicationResponse>, AppError> {
    req.validate()?;
    let svc = BandwidthService::new(&state.db);
    Ok(Json(svc.apply_to_subscription(id, req).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/bandwidth/applications",
    tag = "Bandwidth",
    security(("bearer_auth" = [])),
    params(
        ("profile_id" = Option<i64>, Query, description = "Filter by profile"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "List of applications", body = Vec<BandwidthApplicationResponse>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_applications(State(state): State<SharedState>, Query(q): Query<ApplicationQuery>) -> Result<Json<Vec<BandwidthApplicationResponse>>, AppError> {
    let svc = BandwidthService::new(&state.db);
    Ok(Json(svc.list_applications(q.profile_id, q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?))
}

// ── Usage ───────────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/bandwidth/usage/{subscription_id}",
    tag = "Bandwidth",
    security(("bearer_auth" = [])),
    params(("subscription_id" = i64, Path, description = "Subscription ID")),
    responses(
        (status = 200, description = "Bandwidth usage data", body = BandwidthUsageResponse),
        (status = 404, description = "Subscription not found")
    )
)]
pub async fn get_usage(State(state): State<SharedState>, Path(subscription_id): Path<i64>, Query(q): Query<UsageQuery>) -> Result<Json<BandwidthUsageResponse>, AppError> {
    let svc = BandwidthService::new(&state.db);
    Ok(Json(svc.get_usage(subscription_id, q.page.unwrap_or(1), q.per_page.unwrap_or(50)).await?))
}
