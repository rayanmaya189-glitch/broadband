//! SeaORM-based service for the Network domain.

use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::modules::network::repository::network_repository::NetworkRepository;
use crate::modules::network::request::network_request::*;
use crate::modules::network::response::network_response::*;

pub struct NetworkService<'a> {
    repo: NetworkRepository<'a>,
}

impl<'a> NetworkService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { repo: NetworkRepository::new(db) }
    }

    pub async fn list_vlans(&self, branch_id: Option<i64>) -> Result<Vec<VlanResponse>, AppError> {
        let vlans = self.repo.list_vlans(branch_id).await?;
        Ok(vlans.into_iter().map(|v| VlanResponse { id: v.id, branch_id: v.branch_id, vlan_id: v.vlan_id, name: v.name, description: v.description, vlan_type: v.vlan_type, is_active: v.is_active, created_at: v.created_at.into() }).collect())
    }

    pub async fn create_vlan(&self, branch_id: i64, req: CreateVlanRequest) -> Result<VlanResponse, AppError> {
        let v = self.repo.create_vlan(branch_id, req.vlan_id, &req.name, req.description.as_deref(), &req.vlan_type).await?;
        Ok(VlanResponse { id: v.id, branch_id: v.branch_id, vlan_id: v.vlan_id, name: v.name, description: v.description, vlan_type: v.vlan_type, is_active: v.is_active, created_at: v.created_at.into() })
    }

    pub async fn update_vlan(&self, id: i64, req: UpdateVlanRequest) -> Result<VlanResponse, AppError> {
        let v = self.repo.update_vlan(id, req.name.as_deref(), req.description.as_deref(), req.vlan_type.as_deref()).await?;
        Ok(VlanResponse { id: v.id, branch_id: v.branch_id, vlan_id: v.vlan_id, name: v.name, description: v.description, vlan_type: v.vlan_type, is_active: v.is_active, created_at: v.created_at.into() })
    }

    pub async fn delete_vlan(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.delete_vlan(id).await? { return Err(AppError::NotFound("VLAN not found".into())); }
        Ok(MessageResponse { message: "VLAN deleted".into() })
    }

    pub async fn list_ip_pools(&self, branch_id: Option<i64>) -> Result<Vec<IpPoolResponse>, AppError> {
        let pools = self.repo.list_ip_pools(branch_id).await?;
        Ok(pools.into_iter().map(|p| IpPoolResponse { id: p.id, branch_id: p.branch_id, name: p.name, cidr: p.cidr, gateway: p.gateway, vlan_id: p.vlan_id, pool_type: p.pool_type, allocated_count: p.allocated_count, total_count: p.total_count, status: p.status, is_active: p.is_active, created_at: p.created_at.into() }).collect())
    }

    pub async fn create_ip_pool(&self, branch_id: i64, req: CreateIpPoolRequest) -> Result<IpPoolResponse, AppError> {
        let pool_type = req.pool_type.as_deref().unwrap_or("private");
        let p = self.repo.create_ip_pool(branch_id, &req.name, &req.cidr, &req.gateway, req.dns_primary.as_deref(), req.dns_secondary.as_deref(), req.vlan_id, pool_type, req.total_count).await?;
        Ok(IpPoolResponse { id: p.id, branch_id: p.branch_id, name: p.name, cidr: p.cidr, gateway: p.gateway, vlan_id: p.vlan_id, pool_type: p.pool_type, allocated_count: p.allocated_count, total_count: p.total_count, status: p.status, is_active: p.is_active, created_at: p.created_at.into() })
    }

    pub async fn list_ip_addresses(&self, pool_id: i64) -> Result<Vec<IpAddressResponse>, AppError> {
        let addrs = self.repo.list_ip_addresses(pool_id, None).await?;
        Ok(addrs.into_iter().map(|a| IpAddressResponse { id: a.id, ip_pool_id: a.ip_pool_id, ip_address: a.ip_address, status: a.status, allocated_to_type: a.allocated_to_type, allocated_to_id: a.allocated_to_id, allocated_at: a.allocated_at.map(|v| v.into()), created_at: a.created_at.into() }).collect())
    }

    pub async fn allocate_ip(&self, pool_id: i64, allocated_to_type: &str, allocated_to_id: i64) -> Result<IpAddressResponse, AppError> {
        let a = self.repo.allocate_ip(pool_id, allocated_to_type, allocated_to_id).await?;
        Ok(IpAddressResponse { id: a.id, ip_pool_id: a.ip_pool_id, ip_address: a.ip_address, status: a.status, allocated_to_type: a.allocated_to_type, allocated_to_id: a.allocated_to_id, allocated_at: a.allocated_at.map(|v| v.into()), created_at: a.created_at.into() })
    }

    pub async fn release_ip(&self, pool_id: i64, ip_id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.release_ip(pool_id, ip_id).await? {
            return Err(AppError::NotFound("IP address not found, not allocated, or does not belong to this pool".into()));
        }
        Ok(MessageResponse { message: "IP released".into() })
    }

    pub async fn list_pppoe_sessions(&self, branch_id: Option<i64>, page: i64, per_page: i64) -> Result<(Vec<PppoeSessionResponse>, i64), AppError> {
        let (sessions, total) = self.repo.list_pppoe_sessions(branch_id, page, per_page).await?;
        let responses = sessions.into_iter().map(|s| PppoeSessionResponse {
            id: s.id, branch_id: s.branch_id, customer_id: s.customer_id, subscription_id: s.subscription_id,
            username: s.username, assigned_ip: s.assigned_ip, status: s.status,
            session_start: s.session_start.map(|v| v.into()), bytes_in: s.bytes_in, bytes_out: s.bytes_out,
            created_at: s.created_at.into(),
        }).collect();
        Ok((responses, total))
    }

    pub async fn create_pppoe_session(&self, branch_id: i64, customer_id: i64, subscription_id: i64, username: &str, password: &str) -> Result<PppoeSessionResponse, AppError> {
        let s = self.repo.create_pppoe_session(branch_id, customer_id, subscription_id, username, password).await?;
        Ok(PppoeSessionResponse {
            id: s.id, branch_id: s.branch_id, customer_id: s.customer_id, subscription_id: s.subscription_id,
            username: s.username, assigned_ip: s.assigned_ip, status: s.status,
            session_start: s.session_start.map(|v| v.into()), bytes_in: s.bytes_in, bytes_out: s.bytes_out,
            created_at: s.created_at.into(),
        })
    }

    pub async fn terminate_session(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.terminate_session(id).await? { return Err(AppError::NotFound("Session not found".into())); }
        Ok(MessageResponse { message: "Session terminated".into() })
    }

    pub async fn list_mac_bindings(&self, branch_id: Option<i64>, page: i64, per_page: i64) -> Result<(Vec<MacBindingResponse>, i64), AppError> {
        let (bindings, total) = self.repo.list_mac_bindings(branch_id, page, per_page).await?;
        let responses = bindings.into_iter().map(|b| MacBindingResponse {
            id: b.id, branch_id: b.branch_id, customer_id: b.customer_id, subscription_id: b.subscription_id,
            mac_address: b.mac_address, assigned_ip: b.assigned_ip, vlan_id: b.vlan_id,
            is_active: b.is_active, bound_at: b.bound_at.into(), created_at: b.created_at.into(),
        }).collect();
        Ok((responses, total))
    }

    pub async fn create_mac_binding(&self, branch_id: i64, req: CreateMacBindingRequest) -> Result<MacBindingResponse, AppError> {
        let b = self.repo.create_mac_binding(branch_id, req.customer_id, req.subscription_id, &req.mac_address, &req.assigned_ip, req.vlan_id).await?;
        Ok(MacBindingResponse {
            id: b.id, branch_id: b.branch_id, customer_id: b.customer_id, subscription_id: b.subscription_id,
            mac_address: b.mac_address, assigned_ip: b.assigned_ip, vlan_id: b.vlan_id,
            is_active: b.is_active, bound_at: b.bound_at.into(), created_at: b.created_at.into(),
        })
    }

    pub async fn delete_mac_binding(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.delete_mac_binding(id).await? { return Err(AppError::NotFound("MAC binding not found".into())); }
        Ok(MessageResponse { message: "MAC binding deleted".into() })
    }

    pub async fn list_dhcp_leases(&self, branch_id: Option<i64>, page: i64, per_page: i64) -> Result<(Vec<DhcpLeaseResponse>, i64), AppError> {
        let (leases, total) = self.repo.list_dhcp_leases(branch_id, page, per_page).await?;
        let responses = leases.into_iter().map(|l| DhcpLeaseResponse {
            id: l.id, mac_address: l.mac_address, ip_address: l.ip_address,
            hostname: l.hostname, lease_type: l.lease_type, status: l.status,
            lease_start: l.lease_start.into(), lease_end: l.lease_end.into(), created_at: l.created_at.into(),
        }).collect();
        Ok((responses, total))
    }

    pub async fn list_customer_sessions(&self, branch_id: Option<i64>, is_online: Option<bool>, page: i64, per_page: i64) -> Result<(Vec<CustomerSessionResponse>, i64), AppError> {
        let (sessions, total) = self.repo.list_customer_sessions(branch_id, is_online, page, per_page).await?;
        let responses = sessions.into_iter().map(|s| CustomerSessionResponse {
            id: s.id, customer_id: s.customer_id, mac_address: s.mac_address, ip_address: s.ip_address,
            connected_at: s.connected_at.map(|v| v.into()), last_activity_at: s.last_activity_at.map(|v| v.into()),
            bytes_in: s.bytes_in, bytes_out: s.bytes_out, is_online: s.is_online,
        }).collect();
        Ok((responses, total))
    }

    pub async fn get_topology(&self) -> Result<NetworkTopologyResponse, AppError> {
        let vlans = self.repo.list_vlans(None).await?;
        let total_vlans = vlans.len() as i64;
        let pools = self.repo.list_ip_pools(None).await?;
        let total_pools = pools.len() as i64;
        let (sessions, _total_sessions) = self.repo.list_pppoe_sessions(None, 1, 100000).await?;
        let (bindings, total_bindings) = self.repo.list_mac_bindings(None, 1, 100000).await?;
        let active_sessions = sessions.iter().filter(|s| s.status == "active").count() as i64;
        let active_bindings = bindings.iter().filter(|b| b.is_active).count() as i64;
        Ok(NetworkTopologyResponse {
            total_vlans,
            total_ip_pools: total_pools,
            total_active_sessions: active_sessions,
            total_mac_bindings: total_bindings,
            active_pppoe_sessions: active_sessions,
            active_dhcp_leases: 0,
            online_customers: active_bindings,
        })
    }
}
