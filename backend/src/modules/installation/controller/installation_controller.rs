use axum::extract::{Json, Path, Query, State};
use validator::Validate;
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::UserContext;
use crate::modules::installation::request::installation_request::*;
use crate::modules::installation::response::installation_response::*;
use crate::modules::installation::service::installation_service::InstallationService;

#[utoipa::path(
    get,
    path = "/api/v1/installations",
    tag = "Installations",
    security(("bearer_auth" = [])),
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page"),
        ("status" = Option<String>, Query, description = "Filter by status")
    ),
    responses(
        (status = 200, description = "List of installations"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_installations(State(state): State<SharedState>, Query(q): Query<InstallationQuery>) -> Result<Json<InstallationListResponse>, AppError> {
    let svc = InstallationService::new(&state.db);
    Ok(Json(svc.list_installations(q).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/installations/{id}",
    tag = "Installations",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Installation ID")),
    responses(
        (status = 200, description = "Installation details", body = InstallationResponse),
        (status = 404, description = "Installation not found")
    )
)]
pub async fn get_installation(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<InstallationResponse>, AppError> {
    let svc = InstallationService::new(&state.db);
    Ok(Json(svc.get_installation(id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/installations",
    tag = "Installations",
    security(("bearer_auth" = [])),
    request_body = CreateInstallationRequest,
    responses(
        (status = 200, description = "Installation created", body = InstallationResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_installation(State(state): State<SharedState>, Json(req): Json<CreateInstallationRequest>) -> Result<Json<InstallationResponse>, AppError> {
    req.validate()?;
    let svc = InstallationService::new(&state.db);
    Ok(Json(svc.create_installation(req).await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/installations/{id}/schedule",
    tag = "Installations",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Installation ID")),
    request_body = ScheduleInstallationRequest,
    responses(
        (status = 200, description = "Installation scheduled", body = InstallationResponse),
        (status = 404, description = "Installation not found")
    )
)]
pub async fn schedule_installation(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<ScheduleInstallationRequest>) -> Result<Json<InstallationResponse>, AppError> {
    req.validate()?;
    let svc = InstallationService::new(&state.db);
    Ok(Json(svc.schedule_installation(id, req).await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/installations/{id}/reschedule",
    tag = "Installations",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Installation ID")),
    request_body = RescheduleInstallationRequest,
    responses(
        (status = 200, description = "Installation rescheduled", body = InstallationResponse),
        (status = 404, description = "Installation not found")
    )
)]
pub async fn reschedule_installation(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<RescheduleInstallationRequest>) -> Result<Json<InstallationResponse>, AppError> {
    let svc = InstallationService::new(&state.db);
    Ok(Json(svc.reschedule_installation(id, req).await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/installations/{id}/start",
    tag = "Installations",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Installation ID")),
    responses(
        (status = 200, description = "Installation started", body = InstallationResponse),
        (status = 404, description = "Installation not found")
    )
)]
pub async fn start_installation(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<InstallationResponse>, AppError> {
    let svc = InstallationService::new(&state.db);
    Ok(Json(svc.start_installation(id).await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/installations/{id}/complete",
    tag = "Installations",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Installation ID")),
    request_body = CompleteInstallationRequest,
    responses(
        (status = 200, description = "Installation completed", body = InstallationResponse),
        (status = 404, description = "Installation not found")
    )
)]
pub async fn complete_installation(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<CompleteInstallationRequest>) -> Result<Json<InstallationResponse>, AppError> {
    let svc = InstallationService::new(&state.db);
    Ok(Json(svc.complete_installation(id, req).await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/installations/{id}/cancel",
    tag = "Installations",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Installation ID")),
    responses(
        (status = 200, description = "Installation cancelled"),
        (status = 404, description = "Installation not found")
    )
)]
pub async fn cancel_installation(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = InstallationService::new(&state.db);
    Ok(Json(svc.cancel_installation(id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/installations/{id}/photos",
    tag = "Installations",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Installation ID")),
    request_body = UploadPhotoRequest,
    responses(
        (status = 200, description = "Photo uploaded"),
        (status = 404, description = "Installation not found")
    )
)]
pub async fn upload_photo(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UploadPhotoRequest>) -> Result<Json<MessageResponse>, AppError> {
    let svc = InstallationService::new(&state.db);
    Ok(Json(svc.upload_photo(id, req).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/installations/my-assignments",
    tag = "Installations",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "My assigned installations", body = Vec<InstallationResponse>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_my_assignments(State(state): State<SharedState>, user: UserContext) -> Result<Json<Vec<InstallationResponse>>, AppError> {
    let svc = InstallationService::new(&state.db);
    Ok(Json(svc.get_my_assignments(user.user_id).await?))
}
