use sqlx::PgPool;
use crate::common::errors::app_error::AppError;
use crate::modules::network::repository::network_repository::NetworkRepository;
use crate::modules::network::request::network_request::*;
use crate::modules::network::response::network_response::*;

pub struct NetworkService<'a> { repo: NetworkRepository<'a> }
impl<'a> NetworkService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: NetworkRepository::new(pool) } }

    // ── VLANs ────────────────────────────────────────────────

    pub async fn list_vlans(&self, branch_id: Option<i64>) -> Result<Vec<VlanResponse>, AppError> {
        let vlans = self.repo.list_vlans(branch_id).await?;
        Ok(vlans.into_iter().map(|v| VlanResponse { id: v.id, branch_id: v.branch_id, vlan_id: v.vlan_id, name: v.name, description: v.description, vlan_type: v.vlan_type, is_active: v.is_active, created_at: v.created_at }).collect())
    }

    pub async fn create_vlan(&self, req: CreateVlanRequest) -> Result<VlanResponse, AppError> {
        let v = self.repo.create_vlan(req.branch_id, req.vlan_id, &req.name, req.description.as_deref(), &req.vlan_type).await?;
        Ok(VlanResponse { id: v.id, branch_id: v.branch_id, vlan_id: v.vlan_id, name: v.name, description: v.description, vlan_type: v.vlan_type, is_active: v.is_active, created_at: v.created_at })
    }

    pub async fn update_vlan(&self, id: i64, req: UpdateVlanRequest) -> Result<VlanResponse, AppError> {
        let v = self.repo.update_vlan(id, req.name.as_deref(), req.description.as_deref(), req.vlan_type.as_deref()).await.map_err(|_| AppError::NotFound("VLAN not found".into()))?;
        Ok(VlanResponse { id: v.id, branch_id: v.branch_id, vlan_id: v.vlan_id, name: v.name, description: v.description, vlan_type: v.vlan_type, is_active: v.is_active, created_at: v.created_at })
    }

    pub async fn delete_vlan(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.delete_vlan(id).await? { return Err(AppError::NotFound("VLAN not found".into())); }
        Ok(MessageResponse { message: "VLAN deleted".into() })
    }

    // ── IP Pools ─────────────────────────────────────────────

    pub async fn list_ip_pools(&self, branch_id: Option<i64>) -> Result<Vec<IpPoolResponse>, AppError> {
        let pools = self.repo.list_ip_pools(branch_id).await?;
        Ok(pools.into_iter().map(|p| IpPoolResponse { id: p.id, branch_id: p.branch_id, name: p.name, cidr: p.cidr, gateway: p.gateway, vlan_id: p.vlan_id, pool_type: p.pool_type, allocated_count: p.allocated_count, total_count: p.total_count, status: p.status, is_active: p.is_active, created_at: p.created_at }).collect())
    }

    pub async fn create_ip_pool(&self, req: CreateIpPoolRequest) -> Result<IpPoolResponse, AppError> {
        let p = self.repo.create_ip_pool(req.branch_id, &req.name, &req.cidr, &req.gateway, req.dns_primary.as_deref(), req.dns_secondary.as_deref(), req.vlan_id, &req.pool_type.unwrap_or_else(|| "customer".into()), req.total_count).await?;
        Ok(IpPoolResponse { id: p.id, branch_id: p.branch_id, name: p.name, cidr: p.cidr, gateway: p.gateway, vlan_id: p.vlan_id, pool_type: p.pool_type, allocated_count: p.allocated_count, total_count: p.total_count, status: p.status, is_active: p.is_active, created_at: p.created_at })
    }

    // ── IP Addresses ─────────────────────────────────────────

    pub async fn list_ip_addresses(&self, pool_id: i64, status: Option<&str>) -> Result<Vec<IpAddressResponse>, AppError> {
        let addrs = self.repo.list_ip_addresses(pool_id, status).await?;
        Ok(addrs.into_iter().map(|a| IpAddressResponse { id: a.id, ip_pool_id: a.ip_pool_id, ip_address: a.ip_address, status: a.status, allocated_to_type: a.allocated_to_type, allocated_to_id: a.allocated_to_id, allocated_at: a.allocated_at, created_at: a.created_at }).collect())
    }

    pub async fn allocate_ip(&self, req: AllocateIpRequest) -> Result<IpAddressResponse, AppError> {
        let a = self.repo.allocate_ip(req.pool_id, &req.allocated_to_type, req.allocated_to_id).await.map_err(|_| AppError::BadRequest("No available IP addresses in pool".into()))?;
        Ok(IpAddressResponse { id: a.id, ip_pool_id: a.ip_pool_id, ip_address: a.ip_address, status: a.status, allocated_to_type: a.allocated_to_type, allocated_to_id: a.allocated_to_id, allocated_at: a.allocated_at, created_at: a.created_at })
    }

    pub async fn release_ip(&self, pool_id: i64, ip_id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.release_ip(ip_id, pool_id).await? { return Err(AppError::NotFound("IP address not found".into())); }
        Ok(MessageResponse { message: "IP address released".into() })
    }

    // ── PPPoE Sessions ───────────────────────────────────────

    pub async fn list_pppoe_sessions(&self, branch_id: Option<i64>, page: i64, per_page: i64) -> Result<PaginatedResponse<PppoeSessionResponse>, AppError> {
        let (sessions, total) = self.repo.list_pppoe_sessions(branch_id, page, per_page).await?;
        let items: Vec<PppoeSessionResponse> = sessions.into_iter().map(|s| PppoeSessionResponse { id: s.id, customer_id: s.customer_id, username: s.username, assigned_ip: s.assigned_ip, status: s.status, session_start: s.session_start, bytes_in: s.bytes_in, bytes_out: s.bytes_out }).collect();
        Ok(PaginatedResponse { items, total, page, per_page })
    }

    pub async fn create_pppoe_session(&self, req: CreatePppoeSessionRequest) -> Result<PppoeSessionResponse, AppError> {
        let s = self.repo.create_pppoe_session(req.branch_id, req.customer_id, req.subscription_id, &req.username, &req.password).await?;
        Ok(PppoeSessionResponse { id: s.id, customer_id: s.customer_id, username: s.username, assigned_ip: s.assigned_ip, status: s.status, session_start: s.session_start, bytes_in: s.bytes_in, bytes_out: s.bytes_out })
    }

    pub async fn terminate_session(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.terminate_session(id).await? { return Err(AppError::NotFound("Session not found".into())); }
        Ok(MessageResponse { message: "Session terminated".into() })
    }

    // ── MAC Bindings ─────────────────────────────────────────

    pub async fn list_mac_bindings(&self, branch_id: Option<i64>, page: i64, per_page: i64) -> Result<PaginatedResponse<MacBindingResponse>, AppError> {
        let (bindings, total) = self.repo.list_mac_bindings(branch_id, page, per_page).await?;
        let items: Vec<MacBindingResponse> = bindings.into_iter().map(|b| MacBindingResponse { id: b.id, branch_id: b.branch_id, customer_id: b.customer_id, mac_address: b.mac_address, assigned_ip: b.assigned_ip, vlan_id: b.vlan_id, is_active: b.is_active, bound_at: b.bound_at, created_at: b.created_at }).collect();
        Ok(PaginatedResponse { items, total, page, per_page })
    }

    pub async fn create_mac_binding(&self, req: CreateMacBindingRequest) -> Result<MacBindingResponse, AppError> {
        let b = self.repo.create_mac_binding(req.branch_id, req.customer_id, req.subscription_id, &req.mac_address, &req.assigned_ip, req.vlan_id).await?;
        Ok(MacBindingResponse { id: b.id, branch_id: b.branch_id, customer_id: b.customer_id, mac_address: b.mac_address, assigned_ip: b.assigned_ip, vlan_id: b.vlan_id, is_active: b.is_active, bound_at: b.bound_at, created_at: b.created_at })
    }

    pub async fn delete_mac_binding(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.delete_mac_binding(id).await? { return Err(AppError::NotFound("MAC binding not found".into())); }
        Ok(MessageResponse { message: "MAC binding removed".into() })
    }

    // ── DHCP Leases ──────────────────────────────────────────

    pub async fn list_dhcp_leases(&self, branch_id: Option<i64>, page: i64, per_page: i64) -> Result<PaginatedResponse<DhcpLeaseResponse>, AppError> {
        let (leases, total) = self.repo.list_dhcp_leases(branch_id, page, per_page).await?;
        let items: Vec<DhcpLeaseResponse> = leases.into_iter().map(|l| DhcpLeaseResponse { id: l.id, mac_address: l.mac_address, ip_address: l.ip_address, hostname: l.hostname, lease_type: l.lease_type, status: l.status, lease_start: l.lease_start, lease_end: l.lease_end, created_at: l.created_at }).collect();
        Ok(PaginatedResponse { items, total, page, per_page })
    }

    // ── Customer Sessions ────────────────────────────────────

    pub async fn list_customer_sessions(&self, branch_id: Option<i64>, is_online: Option<bool>, page: i64, per_page: i64) -> Result<PaginatedResponse<CustomerSessionResponse>, AppError> {
        let (sessions, total) = self.repo.list_customer_sessions(branch_id, is_online, page, per_page).await?;
        let items: Vec<CustomerSessionResponse> = sessions.into_iter().map(|s| CustomerSessionResponse { id: s.id, customer_id: s.customer_id, mac_address: s.mac_address, ip_address: s.ip_address, connected_at: s.connected_at, last_activity_at: s.last_activity_at, bytes_in: s.bytes_in, bytes_out: s.bytes_out, is_online: s.is_online }).collect();
        Ok(PaginatedResponse { items, total, page, per_page })
    }

    // ── Network Topology ─────────────────────────────────────

    pub async fn get_topology(&self) -> Result<NetworkTopologyResponse, AppError> {
        let t = self.repo.get_topology().await?;
        Ok(NetworkTopologyResponse { total_vlans: t.total_vlans, total_ip_pools: t.total_ip_pools, total_active_sessions: t.total_active_sessions, total_mac_bindings: t.total_mac_bindings, active_pppoe_sessions: t.active_pppoe_sessions, active_dhcp_leases: t.active_dhcp_leases, online_customers: t.online_customers })
    }
}
