use axum::extract::{Json, Path, Query, State};
use validator::Validate;
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::network::request::network_request::*;
use crate::modules::network::response::network_response::*;
use crate::modules::network::service::network_service::NetworkService;

pub async fn list_vlans(State(state): State<SharedState>, Query(q): Query<NetworkQuery>) -> Result<Json<Vec<VlanResponse>>, AppError> {
    let svc = NetworkService::new(&state.db);
    Ok(Json(svc.list_vlans(q.branch_id).await?))
}

pub async fn create_vlan(State(state): State<SharedState>, Json(req): Json<CreateVlanRequest>) -> Result<Json<VlanResponse>, AppError> {
    req.validate()?;
    let svc = NetworkService::new(&state.db);
    Ok(Json(svc.create_vlan(req).await?))
}

pub async fn delete_vlan(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = NetworkService::new(&state.db);
    Ok(Json(svc.delete_vlan(id).await?))
}

pub async fn list_ip_pools(State(state): State<SharedState>, Query(q): Query<NetworkQuery>) -> Result<Json<Vec<IpPoolResponse>>, AppError> {
    let svc = NetworkService::new(&state.db);
    Ok(Json(svc.list_ip_pools(q.branch_id).await?))
}

pub async fn create_ip_pool(State(state): State<SharedState>, Json(req): Json<CreateIpPoolRequest>) -> Result<Json<IpPoolResponse>, AppError> {
    req.validate()?;
    let svc = NetworkService::new(&state.db);
    Ok(Json(svc.create_ip_pool(req).await?))
}

pub async fn list_pppoe_sessions(State(state): State<SharedState>, Query(q): Query<NetworkQuery>) -> Result<Json<Vec<PppoeSessionResponse>>, AppError> {
    let svc = NetworkService::new(&state.db);
    let page = q.page.unwrap_or(1);
    let per_page = q.per_page.unwrap_or(20);
    let (sessions, _) = svc.list_pppoe_sessions(q.branch_id, page, per_page).await?;
    Ok(Json(sessions))
}

pub async fn terminate_session(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = NetworkService::new(&state.db);
    Ok(Json(svc.terminate_session(id).await?))
}
