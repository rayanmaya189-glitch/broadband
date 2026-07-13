use axum::extract::{Json, Path, Query, State};
use validator::Validate;
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::network::request::network_request::*;
use crate::modules::network::response::network_response::*;
use crate::modules::network::service::network_service::NetworkService;

// ── VLANs ────────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/network/vlans",
    tag = "Network",
    security(("bearer_auth" = [])),
    params(("branch_id" = Option<i64>, Query, description = "Filter by branch")),
    responses(
        (status = 200, description = "List of VLANs", body = Vec<VlanResponse>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_vlans(State(state): State<SharedState>, Query(q): Query<NetworkQuery>) -> Result<Json<Vec<VlanResponse>>, AppError> {
    let svc = NetworkService::new(&state.db);
    Ok(Json(svc.list_vlans(q.branch_id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/network/vlans",
    tag = "Network",
    security(("bearer_auth" = [])),
    request_body = CreateVlanRequest,
    responses(
        (status = 200, description = "VLAN created", body = VlanResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_vlan(State(state): State<SharedState>, Json(req): Json<CreateVlanRequest>) -> Result<Json<VlanResponse>, AppError> {
    req.validate()?;
    let svc = NetworkService::new(&state.db);
    Ok(Json(svc.create_vlan(req).await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/network/vlans/{id}",
    tag = "Network",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "VLAN ID")),
    request_body = UpdateVlanRequest,
    responses(
        (status = 200, description = "VLAN updated", body = VlanResponse),
        (status = 404, description = "VLAN not found")
    )
)]
pub async fn update_vlan(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateVlanRequest>) -> Result<Json<VlanResponse>, AppError> {
    let svc = NetworkService::new(&state.db);
    Ok(Json(svc.update_vlan(id, req).await?))
}

#[utoipa::path(
    delete,
    path = "/api/v1/network/vlans/{id}",
    tag = "Network",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "VLAN ID")),
    responses(
        (status = 200, description = "VLAN deleted"),
        (status = 404, description = "VLAN not found")
    )
)]
pub async fn delete_vlan(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = NetworkService::new(&state.db);
    Ok(Json(svc.delete_vlan(id).await?))
}

// ── IP Pools ─────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/network/ip-pools",
    tag = "Network",
    security(("bearer_auth" = [])),
    params(("branch_id" = Option<i64>, Query, description = "Filter by branch")),
    responses(
        (status = 200, description = "List of IP pools", body = Vec<IpPoolResponse>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_ip_pools(State(state): State<SharedState>, Query(q): Query<NetworkQuery>) -> Result<Json<Vec<IpPoolResponse>>, AppError> {
    let svc = NetworkService::new(&state.db);
    Ok(Json(svc.list_ip_pools(q.branch_id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/network/ip-pools",
    tag = "Network",
    security(("bearer_auth" = [])),
    request_body = CreateIpPoolRequest,
    responses(
        (status = 200, description = "IP pool created", body = IpPoolResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_ip_pool(State(state): State<SharedState>, Json(req): Json<CreateIpPoolRequest>) -> Result<Json<IpPoolResponse>, AppError> {
    req.validate()?;
    let svc = NetworkService::new(&state.db);
    Ok(Json(svc.create_ip_pool(req).await?))
}

// ── IP Addresses ─────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/network/ip-pools/{pool_id}/addresses",
    tag = "Network",
    security(("bearer_auth" = [])),
    params(("pool_id" = i64, Path, description = "IP Pool ID")),
    responses(
        (status = 200, description = "List of IP addresses", body = Vec<IpAddressResponse>),
        (status = 404, description = "Pool not found")
    )
)]
pub async fn list_ip_addresses(State(state): State<SharedState>, Path(pool_id): Path<i64>, Query(q): Query<IpPoolQuery>) -> Result<Json<Vec<IpAddressResponse>>, AppError> {
    let svc = NetworkService::new(&state.db);
    Ok(Json(svc.list_ip_addresses(pool_id, q.status.as_deref()).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/network/ip-pools/{pool_id}/allocate",
    tag = "Network",
    security(("bearer_auth" = [])),
    params(("pool_id" = i64, Path, description = "IP Pool ID")),
    request_body = AllocateIpRequest,
    responses(
        (status = 200, description = "IP allocated", body = IpAddressResponse),
        (status = 404, description = "No available IPs")
    )
)]
pub async fn allocate_ip(State(state): State<SharedState>, Json(req): Json<AllocateIpRequest>) -> Result<Json<IpAddressResponse>, AppError> {
    let svc = NetworkService::new(&state.db);
    Ok(Json(svc.allocate_ip(req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/network/ip-pools/{pool_id}/release",
    tag = "Network",
    security(("bearer_auth" = [])),
    params(("pool_id" = i64, Path, description = "IP Pool ID")),
    request_body = ReleaseIpRequest,
    responses(
        (status = 200, description = "IP released"),
        (status = 404, description = "IP not found")
    )
)]
pub async fn release_ip(State(state): State<SharedState>, Json(req): Json<ReleaseIpRequest>) -> Result<Json<MessageResponse>, AppError> {
    let svc = NetworkService::new(&state.db);
    Ok(Json(svc.release_ip(req.pool_id, req.ip_id).await?))
}

// ── PPPoE Sessions ───────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/network/pppoe/sessions",
    tag = "Network",
    security(("bearer_auth" = [])),
    params(
        ("branch_id" = Option<i64>, Query, description = "Filter by branch"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "List of PPPoE sessions"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_pppoe_sessions(State(state): State<SharedState>, Query(q): Query<NetworkQuery>) -> Result<Json<PaginatedResponse<PppoeSessionResponse>>, AppError> {
    let svc = NetworkService::new(&state.db);
    let page = q.page.unwrap_or(1);
    let per_page = q.per_page.unwrap_or(20);
    Ok(Json(svc.list_pppoe_sessions(q.branch_id, page, per_page).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/network/pppoe/sessions",
    tag = "Network",
    security(("bearer_auth" = [])),
    request_body = CreatePppoeSessionRequest,
    responses(
        (status = 200, description = "PPPoE session created", body = PppoeSessionResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_pppoe_session(State(state): State<SharedState>, Json(req): Json<CreatePppoeSessionRequest>) -> Result<Json<PppoeSessionResponse>, AppError> {
    req.validate()?;
    let svc = NetworkService::new(&state.db);
    Ok(Json(svc.create_pppoe_session(req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/network/pppoe/sessions/{id}/terminate",
    tag = "Network",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Session ID")),
    responses(
        (status = 200, description = "Session terminated"),
        (status = 404, description = "Session not found")
    )
)]
pub async fn terminate_session(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = NetworkService::new(&state.db);
    Ok(Json(svc.terminate_session(id).await?))
}

// ── MAC Bindings ─────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/network/mac-bindings",
    tag = "Network",
    security(("bearer_auth" = [])),
    params(
        ("branch_id" = Option<i64>, Query, description = "Filter by branch"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "List of MAC bindings"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_mac_bindings(State(state): State<SharedState>, Query(q): Query<NetworkQuery>) -> Result<Json<PaginatedResponse<MacBindingResponse>>, AppError> {
    let svc = NetworkService::new(&state.db);
    let page = q.page.unwrap_or(1);
    let per_page = q.per_page.unwrap_or(20);
    Ok(Json(svc.list_mac_bindings(q.branch_id, page, per_page).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/network/mac-bindings",
    tag = "Network",
    security(("bearer_auth" = [])),
    request_body = CreateMacBindingRequest,
    responses(
        (status = 200, description = "MAC binding created", body = MacBindingResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_mac_binding(State(state): State<SharedState>, Json(req): Json<CreateMacBindingRequest>) -> Result<Json<MacBindingResponse>, AppError> {
    let svc = NetworkService::new(&state.db);
    Ok(Json(svc.create_mac_binding(req).await?))
}

#[utoipa::path(
    delete,
    path = "/api/v1/network/mac-bindings/{id}",
    tag = "Network",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "MAC Binding ID")),
    responses(
        (status = 200, description = "MAC binding deleted"),
        (status = 404, description = "Binding not found")
    )
)]
pub async fn delete_mac_binding(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = NetworkService::new(&state.db);
    Ok(Json(svc.delete_mac_binding(id).await?))
}

// ── DHCP Leases ──────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/network/dhcp/leases",
    tag = "Network",
    security(("bearer_auth" = [])),
    params(
        ("branch_id" = Option<i64>, Query, description = "Filter by branch"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "List of DHCP leases"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_dhcp_leases(State(state): State<SharedState>, Query(q): Query<NetworkQuery>) -> Result<Json<PaginatedResponse<DhcpLeaseResponse>>, AppError> {
    let svc = NetworkService::new(&state.db);
    let page = q.page.unwrap_or(1);
    let per_page = q.per_page.unwrap_or(20);
    Ok(Json(svc.list_dhcp_leases(q.branch_id, page, per_page).await?))
}

// ── Customer Sessions ────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/network/sessions",
    tag = "Network",
    security(("bearer_auth" = [])),
    params(
        ("branch_id" = Option<i64>, Query, description = "Filter by branch"),
        ("is_online" = Option<bool>, Query, description = "Filter by online status"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "List of customer sessions"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_customer_sessions(State(state): State<SharedState>, Query(q): Query<NetworkQuery>) -> Result<Json<PaginatedResponse<CustomerSessionResponse>>, AppError> {
    let svc = NetworkService::new(&state.db);
    let page = q.page.unwrap_or(1);
    let per_page = q.per_page.unwrap_or(20);
    Ok(Json(svc.list_customer_sessions(q.branch_id, q.is_online, page, per_page).await?))
}

// ── Network Topology ─────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/network/topology",
    tag = "Network",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Network topology", body = NetworkTopologyResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_topology(State(state): State<SharedState>) -> Result<Json<NetworkTopologyResponse>, AppError> {
    let svc = NetworkService::new(&state.db);
    Ok(Json(svc.get_topology().await?))
}
