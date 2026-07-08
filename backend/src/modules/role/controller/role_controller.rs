use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::role::request::role_request::*;
use crate::modules::role::response::role_response::*;
use crate::modules::role::service::role_service::RoleService;

pub async fn list_roles(State(state): State<SharedState>, Query(query): Query<ListRolesQuery>) -> Result<Json<crate::common::utils::helpers::PaginatedResponse<RoleResponse>>, AppError> {
    let svc = RoleService::new(&state.db);
    Ok(Json(svc.list_roles(&query).await?))
}

pub async fn create_role(State(state): State<SharedState>, Json(req): Json<CreateRoleRequest>) -> Result<Json<RoleResponse>, AppError> {
    req.validate()?;
    let svc = RoleService::new(&state.db);
    Ok(Json(svc.create_role(&req).await?))
}

pub async fn get_role(State(state): State<SharedState>, Path(role_id): Path<i64>) -> Result<Json<RoleResponse>, AppError> {
    let svc = RoleService::new(&state.db);
    Ok(Json(svc.get_role(role_id).await?))
}

pub async fn update_role(State(state): State<SharedState>, Path(role_id): Path<i64>, Json(req): Json<UpdateRoleRequest>) -> Result<Json<RoleResponse>, AppError> {
    req.validate()?;
    let svc = RoleService::new(&state.db);
    Ok(Json(svc.update_role(role_id, &req).await?))
}

pub async fn deactivate_role(State(state): State<SharedState>, Path(role_id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = RoleService::new(&state.db);
    Ok(Json(svc.deactivate_role(role_id).await?))
}
