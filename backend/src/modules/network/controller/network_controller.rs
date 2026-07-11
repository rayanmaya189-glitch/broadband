//! SeaORM-based controller for the Network domain.

use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::network::request::network_request::*;
use crate::modules::network::response::network_response::*;
use crate::modules::network::service::network_service::NetworkService;

pub async fn list_vlans(State(state): State<SharedState>, Query(q): Query<NetworkQuery>) -> Result<Json<Vec<VlanResponse>>, AppError> {
    let svc = NetworkService::new(&state.db_seaorm);
    Ok(Json(svc.list_vlans(q.branch_id).await?))
}

pub async fn create_vlan(State(state): State<SharedState>, Path(branch_id): Path<i64>, Json(req): Json<CreateVlanRequest>) -> Result<Json<VlanResponse>, AppError> {
    req.validate()?;
    let svc = NetworkService::new(&state.db_seaorm);
    Ok(Json(svc.create_vlan(branch_id, req).await?))
}

pub async fn update_vlan(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateVlanRequest>) -> Result<Json<VlanResponse>, AppError> {
    let svc = NetworkService::new(&state.db_seaorm);
    Ok(Json(svc.update_vlan(id, req).await?))
}

pub async fn delete_vlan(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = NetworkService::new(&state.db_seaorm);
    Ok(Json(svc.delete_vlan(id).await?))
}

pub async fn list_ip_pools(State(state): State<SharedState>, Query(q): Query<NetworkQuery>) -> Result<Json<Vec<IpPoolResponse>>, AppError> {
    let svc = NetworkService::new(&state.db_seaorm);
    Ok(Json(svc.list_ip_pools(q.branch_id).await?))
}

pub async fn create_ip_pool(State(state): State<SharedState>, Path(branch_id): Path<i64>, Json(req): Json<CreateIpPoolRequest>) -> Result<Json<IpPoolResponse>, AppError> {
    req.validate()?;
    let svc = NetworkService::new(&state.db_seaorm);
    Ok(Json(svc.create_ip_pool(branch_id, req).await?))
}

pub async fn list_ip_addresses(State(state): State<SharedState>, Path(pool_id): Path<i64>) -> Result<Json<Vec<IpAddressResponse>>, AppError> {
    let svc = NetworkService::new(&state.db_seaorm);
    Ok(Json(svc.list_ip_addresses(pool_id).await?))
}

pub async fn allocate_ip(State(state): State<SharedState>, Json(req): Json<AllocateIpRequest>) -> Result<Json<IpAddressResponse>, AppError> {
    let svc = NetworkService::new(&state.db_seaorm);
    Ok(Json(svc.allocate_ip(req.pool_id, &req.allocated_to_type, req.allocated_to_id).await?))
}

pub async fn release_ip(State(state): State<SharedState>, Json(req): Json<ReleaseIpRequest>) -> Result<Json<MessageResponse>, AppError> {
    let svc = NetworkService::new(&state.db_seaorm);
    Ok(Json(svc.release_ip(req.pool_id, req.ip_id).await?))
}

pub async fn list_pppoe_sessions(State(state): State<SharedState>, Query(q): Query<SessionQuery>) -> Result<Json<Vec<PppoeSessionResponse>>, AppError> {
    let svc = NetworkService::new(&state.db_seaorm);
    let (sessions, _) = svc.list_pppoe_sessions(q.branch_id, q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?;
    Ok(Json(sessions))
}

pub async fn create_pppoe_session(State(state): State<SharedState>, Path(branch_id): Path<i64>, Json(req): Json<CreatePppoeSessionRequest>) -> Result<Json<PppoeSessionResponse>, AppError> {
    req.validate()?;
    let svc = NetworkService::new(&state.db_seaorm);
    Ok(Json(svc.create_pppoe_session(branch_id, req.customer_id, req.subscription_id, &req.username, &req.password).await?))
}

pub async fn terminate_session(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = NetworkService::new(&state.db_seaorm);
    Ok(Json(svc.terminate_session(id).await?))
}

pub async fn list_mac_bindings(State(state): State<SharedState>, Query(q): Query<SessionQuery>) -> Result<Json<Vec<MacBindingResponse>>, AppError> {
    let svc = NetworkService::new(&state.db_seaorm);
    let (bindings, _) = svc.list_mac_bindings(q.branch_id, q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?;
    Ok(Json(bindings))
}

pub async fn create_mac_binding(State(state): State<SharedState>, Path(branch_id): Path<i64>, Json(req): Json<CreateMacBindingRequest>) -> Result<Json<MacBindingResponse>, AppError> {
    req.validate()?;
    let svc = NetworkService::new(&state.db_seaorm);
    Ok(Json(svc.create_mac_binding(branch_id, req).await?))
}

pub async fn delete_mac_binding(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = NetworkService::new(&state.db_seaorm);
    Ok(Json(svc.delete_mac_binding(id).await?))
}

pub async fn list_dhcp_leases(State(state): State<SharedState>, Query(q): Query<SessionQuery>) -> Result<Json<Vec<DhcpLeaseResponse>>, AppError> {
    let svc = NetworkService::new(&state.db_seaorm);
    let (leases, _) = svc.list_dhcp_leases(q.branch_id, q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?;
    Ok(Json(leases))
}

pub async fn list_customer_sessions(State(state): State<SharedState>, Query(q): Query<NetworkQuery>) -> Result<Json<Vec<CustomerSessionResponse>>, AppError> {
    let svc = NetworkService::new(&state.db_seaorm);
    let (sessions, _) = svc.list_customer_sessions(q.branch_id, q.is_online, q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?;
    Ok(Json(sessions))
}

pub async fn get_topology(State(state): State<SharedState>) -> Result<Json<NetworkTopologyResponse>, AppError> {
    let svc = NetworkService::new(&state.db_seaorm);
    Ok(Json(svc.get_topology().await?))
}
