use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::modules::network::application::services::NetworkService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};

#[derive(Debug, Serialize)]
pub struct VlanResponse {
    pub id: i64,
    pub branch_id: i64,
    pub vlan_id: i32,
    pub name: String,
    pub vlan_type: String,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateVlanRequest {
    pub branch_id: i64,
    pub vlan_id: i32,
    pub name: String,
    pub vlan_type: String,
}

#[derive(Debug, Serialize)]
pub struct IpPoolResponse {
    pub id: i64,
    pub name: String,
    pub cidr: String,
    pub gateway: String,
    pub allocated_count: i32,
    pub total_count: i32,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateIpPoolRequest {
    pub branch_id: i64,
    pub name: String,
    pub cidr: String,
    pub gateway: String,
    #[serde(default)]
    pub vlan_id: Option<i64>,
    pub pool_type: String,
    pub total_count: i32,
}

#[derive(Debug, Serialize)]
pub struct PppoeSessionResponse {
    pub id: i64,
    pub username: String,
    pub assigned_ip: Option<String>,
    pub status: String,
    pub customer_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreatePppoeRequest {
    pub branch_id: i64,
    pub customer_id: i64,
    pub subscription_id: i64,
    pub username: String,
    pub password_encrypted: String,
}

#[derive(Debug, Serialize)]
pub struct MacBindingResponse {
    pub id: i64,
    pub mac_address: String,
    pub assigned_ip: String,
    pub customer_id: i64,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateMacBindingRequest {
    pub branch_id: i64,
    pub customer_id: i64,
    pub subscription_id: i64,
    pub mac_address: String,
    pub assigned_ip: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateVlanRequest {
    pub name: String,
    pub vlan_type: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateIpPoolRequest {
    pub name: String,
    pub gateway: String,
}

#[derive(Debug, Deserialize)]
pub struct AllocateIpRequest {
    pub customer_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct ReleaseIpRequest {
    pub customer_id: i64,
}

#[derive(Debug, Serialize)]
pub struct DhcpLeaseResponse {
    pub id: i64,
    pub branch_id: i64,
    pub ip_address: String,
    pub mac_address: String,
    pub hostname: Option<String>,
    pub lease_start: chrono::DateTime<chrono::Utc>,
    pub lease_end: Option<chrono::DateTime<chrono::Utc>>,
    pub status: String,
}

// --- VLANs ---
pub async fn list_vlans(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<Vec<VlanResponse>>, AppError> {
    require_permission(&user, "network.vlan.view").map_err(|e| AppError::Forbidden(e.1))?;
    let bid = if user.is_company_wide {
        None
    } else {
        user.branch_id
    };
    let vlans = NetworkService::list_vlans(&state.db, bid).await?;
    Ok(Json(
        vlans
            .into_iter()
            .map(|v| VlanResponse {
                id: v.id,
                branch_id: v.branch_id,
                vlan_id: v.vlan_id,
                name: v.name,
                vlan_type: v.vlan_type,
                is_active: v.is_active,
            })
            .collect(),
    ))
}

pub async fn create_vlan(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreateVlanRequest>,
) -> Result<(StatusCode, Json<VlanResponse>), AppError> {
    require_permission(&user, "network.vlan.create").map_err(|e| AppError::Forbidden(e.1))?;
    let vlan = NetworkService::create_vlan(
        &state.db,
        req.branch_id,
        req.vlan_id,
        req.name,
        req.vlan_type,
    )
    .await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "network.vlan.created",
        "vlan",
        vlan.id,
        serde_json::json!({"vlan_id": vlan.id, "name": vlan.name}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish network.vlan.created event");
    }
    Ok((
        StatusCode::CREATED,
        Json(VlanResponse {
            id: vlan.id,
            branch_id: vlan.branch_id,
            vlan_id: vlan.vlan_id,
            name: vlan.name,
            vlan_type: vlan.vlan_type,
            is_active: vlan.is_active,
        }),
    ))
}

pub async fn delete_vlan(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "network.vlan.delete").map_err(|e| AppError::Forbidden(e.1))?;
    NetworkService::delete_vlan(&state.db, id).await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "network.vlan.deleted",
        "vlan",
        id,
        serde_json::json!({"vlan_id": id}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish network.vlan.deleted event");
    }
    Ok(StatusCode::NO_CONTENT)
}

// --- IP Pools ---
pub async fn list_ip_pools(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<Vec<IpPoolResponse>>, AppError> {
    require_permission(&user, "network.ippool.view").map_err(|e| AppError::Forbidden(e.1))?;
    let bid = if user.is_company_wide {
        None
    } else {
        user.branch_id
    };
    let pools = NetworkService::list_ip_pools(&state.db, bid).await?;
    Ok(Json(
        pools
            .into_iter()
            .map(|p| IpPoolResponse {
                id: p.id,
                name: p.name,
                cidr: p.cidr,
                gateway: p.gateway,
                allocated_count: p.allocated_count,
                total_count: p.total_count,
                status: p.status,
            })
            .collect(),
    ))
}

pub async fn create_ip_pool(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreateIpPoolRequest>,
) -> Result<(StatusCode, Json<IpPoolResponse>), AppError> {
    require_permission(&user, "network.ippool.create").map_err(|e| AppError::Forbidden(e.1))?;
    let pool = NetworkService::create_ip_pool(
        &state.db,
        req.branch_id,
        req.name,
        req.cidr,
        req.gateway,
        req.vlan_id,
        req.pool_type,
        req.total_count,
    )
    .await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "network.ippool.created",
        "ip_pool",
        pool.id,
        serde_json::json!({"pool_id": pool.id, "name": pool.name}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish network.ippool.created event");
    }
    Ok((
        StatusCode::CREATED,
        Json(IpPoolResponse {
            id: pool.id,
            name: pool.name,
            cidr: pool.cidr,
            gateway: pool.gateway,
            allocated_count: pool.allocated_count,
            total_count: pool.total_count,
            status: pool.status,
        }),
    ))
}

// --- PPPoE ---
pub async fn list_pppoe_sessions(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<Vec<PppoeSessionResponse>>, AppError> {
    require_permission(&user, "network.pppoe.view").map_err(|e| AppError::Forbidden(e.1))?;
    let bid = if user.is_company_wide {
        None
    } else {
        user.branch_id
    };
    let sessions = NetworkService::list_pppoe_sessions(&state.db, bid).await?;
    Ok(Json(
        sessions
            .into_iter()
            .map(|s| PppoeSessionResponse {
                id: s.id,
                username: s.username,
                assigned_ip: s.assigned_ip,
                status: s.status,
                customer_id: s.customer_id,
            })
            .collect(),
    ))
}

pub async fn create_pppoe_session(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreatePppoeRequest>,
) -> Result<(StatusCode, Json<PppoeSessionResponse>), AppError> {
    require_permission(&user, "network.pppoe.create").map_err(|e| AppError::Forbidden(e.1))?;
    let session = NetworkService::create_pppoe_session(
        &state.db,
        req.branch_id,
        req.customer_id,
        req.subscription_id,
        req.username,
        req.password_encrypted,
    )
    .await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "network.pppoe.created",
        "pppoe_session",
        session.id,
        serde_json::json!({"session_id": session.id}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish network.pppoe.created event");
    }
    Ok((
        StatusCode::CREATED,
        Json(PppoeSessionResponse {
            id: session.id,
            username: session.username,
            assigned_ip: session.assigned_ip,
            status: session.status,
            customer_id: session.customer_id,
        }),
    ))
}

pub async fn terminate_pppoe_session(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "network.pppoe.terminate").map_err(|e| AppError::Forbidden(e.1))?;
    NetworkService::terminate_pppoe_session(&state.db, id).await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "network.pppoe.terminated",
        "pppoe_session",
        id,
        serde_json::json!({"session_id": id}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish network.pppoe.terminated event");
    }
    Ok(StatusCode::OK)
}

// --- MAC Bindings ---
pub async fn list_mac_bindings(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<Vec<MacBindingResponse>>, AppError> {
    require_permission(&user, "network.mac_binding.view").map_err(|e| AppError::Forbidden(e.1))?;
    let bid = if user.is_company_wide {
        None
    } else {
        user.branch_id
    };
    let bindings = NetworkService::list_mac_bindings(&state.db, bid).await?;
    Ok(Json(
        bindings
            .into_iter()
            .map(|b| MacBindingResponse {
                id: b.id,
                mac_address: b.mac_address,
                assigned_ip: b.assigned_ip,
                customer_id: b.customer_id,
                is_active: b.is_active,
            })
            .collect(),
    ))
}

pub async fn create_mac_binding(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreateMacBindingRequest>,
) -> Result<(StatusCode, Json<MacBindingResponse>), AppError> {
    require_permission(&user, "network.mac_binding.create")
        .map_err(|e| AppError::Forbidden(e.1))?;
    let binding = NetworkService::create_mac_binding(
        &state.db,
        req.branch_id,
        req.customer_id,
        req.subscription_id,
        req.mac_address,
        req.assigned_ip,
    )
    .await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "network.mac_binding.created",
        "mac_binding",
        binding.id,
        serde_json::json!({"binding_id": binding.id}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish network.mac_binding.created event");
    }
    Ok((
        StatusCode::CREATED,
        Json(MacBindingResponse {
            id: binding.id,
            mac_address: binding.mac_address,
            assigned_ip: binding.assigned_ip,
            customer_id: binding.customer_id,
            is_active: binding.is_active,
        }),
    ))
}

// --- Update VLAN ---
pub async fn update_vlan(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<UpdateVlanRequest>,
) -> Result<Json<VlanResponse>, AppError> {
    require_permission(&user, "network.vlan.create").map_err(|e| AppError::Forbidden(e.1))?;
    let vlan = NetworkService::update_vlan(&state.db, id, req.name, req.vlan_type).await?;
    Ok(Json(VlanResponse {
        id: vlan.id,
        branch_id: vlan.branch_id,
        vlan_id: vlan.vlan_id,
        name: vlan.name,
        vlan_type: vlan.vlan_type,
        is_active: vlan.is_active,
    }))
}

// --- Update IP Pool ---
pub async fn update_ip_pool(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<UpdateIpPoolRequest>,
) -> Result<Json<IpPoolResponse>, AppError> {
    require_permission(&user, "network.ippool.create").map_err(|e| AppError::Forbidden(e.1))?;
    let pool = NetworkService::update_ip_pool(&state.db, id, req.name, req.gateway).await?;
    Ok(Json(IpPoolResponse {
        id: pool.id,
        name: pool.name,
        cidr: pool.cidr,
        gateway: pool.gateway,
        allocated_count: pool.allocated_count,
        total_count: pool.total_count,
        status: pool.status,
    }))
}

// --- List Pool Addresses ---
pub async fn list_pool_addresses(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "network.ippool.view").map_err(|e| AppError::Forbidden(e.1))?;
    let info = NetworkService::list_pool_addresses(&state.db, id).await?;
    Ok(Json(info))
}

// --- Allocate IP ---
pub async fn allocate_ip(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<AllocateIpRequest>,
) -> Result<Json<IpPoolResponse>, AppError> {
    require_permission(&user, "network.ippool.create").map_err(|e| AppError::Forbidden(e.1))?;
    let pool = NetworkService::allocate_ip(&state.db, id, req.customer_id).await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "network.ippool.ip_allocated",
        "ip_pool",
        pool.id,
        serde_json::json!({"pool_id": pool.id, "customer_id": req.customer_id}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish network.ippool.ip_allocated event");
    }
    Ok(Json(IpPoolResponse {
        id: pool.id,
        name: pool.name,
        cidr: pool.cidr,
        gateway: pool.gateway,
        allocated_count: pool.allocated_count,
        total_count: pool.total_count,
        status: pool.status,
    }))
}

// --- Release IP ---
pub async fn release_ip(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<ReleaseIpRequest>,
) -> Result<Json<IpPoolResponse>, AppError> {
    require_permission(&user, "network.ippool.create").map_err(|e| AppError::Forbidden(e.1))?;
    let pool = NetworkService::release_ip(&state.db, id, req.customer_id).await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "network.ippool.ip_released",
        "ip_pool",
        pool.id,
        serde_json::json!({"pool_id": pool.id, "customer_id": req.customer_id}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish network.ippool.ip_released event");
    }
    Ok(Json(IpPoolResponse {
        id: pool.id,
        name: pool.name,
        cidr: pool.cidr,
        gateway: pool.gateway,
        allocated_count: pool.allocated_count,
        total_count: pool.total_count,
        status: pool.status,
    }))
}

// --- DHCP Leases ---
pub async fn list_dhcp_leases(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<Vec<DhcpLeaseResponse>>, AppError> {
    require_permission(&user, "network.vlan.view").map_err(|e| AppError::Forbidden(e.1))?;
    let bid = if user.is_company_wide {
        None
    } else {
        user.branch_id
    };
    let leases = NetworkService::list_dhcp_leases(&state.db, bid).await?;
    Ok(Json(
        leases
            .into_iter()
            .map(|l| DhcpLeaseResponse {
                id: l.id,
                branch_id: l.branch_id,
                ip_address: l.ip_address,
                mac_address: l.mac_address,
                hostname: l.hostname,
                lease_start: l.lease_start,
                lease_end: l.lease_end,
                status: l.status,
            })
            .collect(),
    ))
}

// --- Customer Network Sessions ---
pub async fn list_customer_sessions(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "network.pppoe.view").map_err(|e| AppError::Forbidden(e.1))?;
    let bid = if user.is_company_wide {
        None
    } else {
        user.branch_id
    };
    let sessions = NetworkService::list_customer_sessions(&state.db, bid).await?;
    Ok(Json(sessions))
}

/// GET /api/v1/network/topology
pub async fn get_topology(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "network.vlan.view").map_err(|e| AppError::Forbidden(e.1))?;
    let bid = if user.is_company_wide { None } else { user.branch_id };
    let topology = NetworkService::get_topology(&state.db, bid).await?;
    Ok(Json(topology))
}
