use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::permission::request::permission_request::*;
use crate::modules::permission::response::permission_response::*;
use crate::modules::permission::service::permission_service::PermissionService;

#[utoipa::path(
    get,
    path = "/api/v1/permissions",
    tag = "Permissions",
    security(("bearer_auth" = [])),
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page"),
        ("module" = Option<String>, Query, description = "Filter by module")
    ),
    responses(
        (status = 200, description = "List of permissions"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_permissions(State(state): State<SharedState>, Query(query): Query<ListPermissionsQuery>) -> Result<Json<crate::common::utils::helpers::PaginatedResponse<PermissionResponse>>, AppError> {
    let svc = PermissionService::new(&state.db);
    Ok(Json(svc.list_permissions(&query).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/permissions",
    tag = "Permissions",
    security(("bearer_auth" = [])),
    request_body = CreatePermissionRequest,
    responses(
        (status = 200, description = "Permission created", body = PermissionResponse),
        (status = 409, description = "Permission already exists"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_permission(State(state): State<SharedState>, Json(req): Json<CreatePermissionRequest>) -> Result<Json<PermissionResponse>, AppError> {
    req.validate()?;
    let svc = PermissionService::new(&state.db);
    Ok(Json(svc.create_permission(&req).await?))
}

#[utoipa::path(
    delete,
    path = "/api/v1/permissions/{id}",
    tag = "Permissions",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Permission ID")),
    responses(
        (status = 200, description = "Permission deleted"),
        (status = 404, description = "Permission not found")
    )
)]
pub async fn delete_permission(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = PermissionService::new(&state.db);
    Ok(Json(svc.delete_permission(id).await?))
}
