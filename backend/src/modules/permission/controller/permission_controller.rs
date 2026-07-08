use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::permission::request::permission_request::*;
use crate::modules::permission::response::permission_response::*;
use crate::modules::permission::service::permission_service::PermissionService;

pub async fn list_permissions(State(state): State<SharedState>, Query(query): Query<ListPermissionsQuery>) -> Result<Json<crate::common::utils::helpers::PaginatedResponse<PermissionResponse>>, AppError> {
    let svc = PermissionService::new(&state.db);
    Ok(Json(svc.list_permissions(&query).await?))
}

pub async fn create_permission(State(state): State<SharedState>, Json(req): Json<CreatePermissionRequest>) -> Result<Json<PermissionResponse>, AppError> {
    req.validate()?;
    let svc = PermissionService::new(&state.db);
    Ok(Json(svc.create_permission(&req).await?))
}

pub async fn delete_permission(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = PermissionService::new(&state.db);
    Ok(Json(svc.delete_permission(id).await?))
}
