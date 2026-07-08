use sqlx::PgPool;
use crate::common::errors::app_error::AppError;
use crate::modules::network::repository::network_repository::NetworkRepository;
use crate::modules::network::request::network_request::*;
use crate::modules::network::response::network_response::*;

pub struct NetworkService<'a> { repo: NetworkRepository<'a> }
impl<'a> NetworkService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: NetworkRepository::new(pool) } }

    pub async fn list_vlans(&self, branch_id: Option<i64>) -> Result<Vec<VlanResponse>, AppError> {
        let vlans = self.repo.list_vlans(branch_id).await?;
        Ok(vlans.iter().map(|v| VlanResponse { id: v.id, branch_id: v.branch_id, vlan_id: v.vlan_id, name: v.name.clone(), description: v.description.clone(), vlan_type: v.vlan_type.clone(), is_active: v.is_active, created_at: v.created_at }).collect())
    }

    pub async fn create_vlan(&self, req: CreateVlanRequest) -> Result<VlanResponse, AppError> {
        let v = self.repo.create_vlan(req.branch_id, req.vlan_id, &req.name, req.description.as_deref(), &req.vlan_type).await?;
        Ok(VlanResponse { id: v.id, branch_id: v.branch_id, vlan_id: v.vlan_id, name: v.name, description: v.description, vlan_type: v.vlan_type, is_active: v.is_active, created_at: v.created_at })
    }

    pub async fn delete_vlan(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.delete_vlan(id).await? { return Err(AppError::NotFound("VLAN not found".into())); }
        Ok(MessageResponse { message: "VLAN deleted".into() })
    }

    pub async fn list_ip_pools(&self, branch_id: Option<i64>) -> Result<Vec<IpPoolResponse>, AppError> {
        let pools = self.repo.list_ip_pools(branch_id).await?;
        Ok(pools.iter().map(|p| IpPoolResponse { id: p.id, branch_id: p.branch_id, name: p.name.clone(), cidr: p.cidr.clone(), gateway: p.gateway.clone(), vlan_id: p.vlan_id, pool_type: p.pool_type.clone(), allocated_count: p.allocated_count, total_count: p.total_count, status: p.status.clone(), is_active: p.is_active, created_at: p.created_at }).collect())
    }

    pub async fn create_ip_pool(&self, req: CreateIpPoolRequest) -> Result<IpPoolResponse, AppError> {
        let p = self.repo.create_ip_pool(req.branch_id, &req.name, &req.cidr, &req.gateway, req.dns_primary.as_deref(), req.dns_secondary.as_deref(), req.vlan_id, &req.pool_type.unwrap_or_else(|| "customer".into()), req.total_count).await?;
        Ok(IpPoolResponse { id: p.id, branch_id: p.branch_id, name: p.name, cidr: p.cidr, gateway: p.gateway, vlan_id: p.vlan_id, pool_type: p.pool_type, allocated_count: p.allocated_count, total_count: p.total_count, status: p.status, is_active: p.is_active, created_at: p.created_at })
    }

    pub async fn list_pppoe_sessions(&self, branch_id: Option<i64>, page: i64, per_page: i64) -> Result<(Vec<PppoeSessionResponse>, i64), AppError> {
        let (sessions, total) = self.repo.list_pppoe_sessions(branch_id, page, per_page).await?;
        let responses: Vec<PppoeSessionResponse> = sessions.iter().map(|s| PppoeSessionResponse { id: s.id, customer_id: s.customer_id, username: s.username.clone(), assigned_ip: s.assigned_ip.clone(), status: s.status.clone(), session_start: s.session_start, bytes_in: s.bytes_in, bytes_out: s.bytes_out }).collect();
        Ok((responses, total))
    }

    pub async fn terminate_session(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.terminate_session(id).await? { return Err(AppError::NotFound("Session not found".into())); }
        Ok(MessageResponse { message: "Session terminated".into() })
    }
}
