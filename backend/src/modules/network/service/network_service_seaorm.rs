//! SeaORM-based service for the Network domain.

use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::modules::network::repository::network_repository_seaorm::NetworkRepositorySeaorm;
use crate::modules::network::request::network_request::*;
use crate::modules::network::response::network_response::*;

pub struct NetworkServiceSeaorm<'a> {
    repo: NetworkRepositorySeaorm<'a>,
}

impl<'a> NetworkServiceSeaorm<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { repo: NetworkRepositorySeaorm::new(db) }
    }

    pub async fn list_vlans(&self, branch_id: Option<i64>) -> Result<Vec<VlanResponse>, AppError> {
        let vlans = self.repo.list_vlans(branch_id).await?;
        Ok(vlans.into_iter().map(|v| VlanResponse { id: v.id, branch_id: v.branch_id, vlan_id: v.vlan_id, name: v.name, description: v.description, vlan_type: v.vlan_type, is_active: v.is_active, created_at: v.created_at.into() }).collect())
    }

    pub async fn create_vlan(&self, branch_id: i64, req: CreateVlanRequest) -> Result<VlanResponse, AppError> {
        let v = self.repo.create_vlan(branch_id, req.vlan_id, &req.name, req.description.as_deref(), &req.vlan_type).await?;
        Ok(VlanResponse { id: v.id, branch_id: v.branch_id, vlan_id: v.vlan_id, name: v.name, description: v.description, vlan_type: v.vlan_type, is_active: v.is_active, created_at: v.created_at.into() })
    }

    pub async fn list_ip_pools(&self, branch_id: Option<i64>) -> Result<Vec<IpPoolResponse>, AppError> {
        let pools = self.repo.list_ip_pools(branch_id).await?;
        Ok(pools.into_iter().map(|p| IpPoolResponse { id: p.id, branch_id: p.branch_id, name: p.name, cidr: p.cidr, gateway: p.gateway, pool_type: p.pool_type, allocated_count: p.allocated_count, total_count: p.total_count, status: p.status, is_active: p.is_active, created_at: p.created_at.into() }).collect())
    }

    pub async fn create_ip_pool(&self, branch_id: i64, req: CreateIpPoolRequest) -> Result<IpPoolResponse, AppError> {
        let p = self.repo.create_ip_pool(branch_id, &req.name, &req.cidr, &req.gateway, req.dns_primary.as_deref(), req.dns_secondary.as_deref(), req.vlan_id, &req.pool_type, req.total_count).await?;
        Ok(IpPoolResponse { id: p.id, branch_id: p.branch_id, name: p.name, cidr: p.cidr, gateway: p.gateway, pool_type: p.pool_type, allocated_count: p.allocated_count, total_count: p.total_count, status: p.status, is_active: p.is_active, created_at: p.created_at.into() })
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
}
