//! SeaORM-based repository for the Network domain.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};

use crate::common::errors::app_error::AppError;
use crate::modules::network::model::vlan_entity::{self, Model as VlanModel};
use crate::modules::network::model::ip_pool_entity::{self, Model as IpPoolModel};
use crate::modules::network::model::ip_address_entity::{self, Model as IpAddressModel};
use crate::modules::network::model::pppoe_session_entity::{self, Model as PppoeSessionModel};
use crate::modules::network::model::mac_binding_entity::{self, Model as MacBindingModel};
use crate::modules::network::model::dhcp_lease_entity::{self, Model as DhcpLeaseModel};
use crate::modules::network::model::customer_session_entity::{self, Model as CustomerSessionModel};

pub struct NetworkRepositorySeaorm<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> NetworkRepositorySeaorm<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    // ── VLANs ────────────────────────────────────────────────

    pub async fn list_vlans(&self, branch_id: Option<i64>) -> Result<Vec<VlanModel>, AppError> {
        let mut select = vlan_entity::Entity::find();
        if let Some(bid) = branch_id { select = select.filter(vlan_entity::Column::BranchId.eq(bid)); }
        Ok(select.order_by_asc(vlan_entity::Column::VlanId).all(self.db).await?)
    }

    pub async fn create_vlan(&self, branch_id: i64, vlan_id: i32, name: &str, description: Option<&str>, vlan_type: &str) -> Result<VlanModel, AppError> {
        let now = chrono::Utc::now();
        let active = vlan_entity::ActiveModel {
            branch_id: Set(branch_id),
            vlan_id: Set(vlan_id),
            name: Set(name.to_owned()),
            description: Set(description.map(|s| s.to_owned())),
            vlan_type: Set(vlan_type.to_owned()),
            is_active: Set(true),
            created_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn update_vlan(&self, id: i64, name: Option<&str>, description: Option<&str>, vlan_type: Option<&str>) -> Result<VlanModel, AppError> {
        let existing = vlan_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("VLAN not found".into()))?;
        let mut active = existing.into_active_model();
        if let Some(v) = name { active.name = Set(v.to_owned()); }
        if let Some(v) = description { active.description = Set(Some(v.to_owned())); }
        if let Some(v) = vlan_type { active.vlan_type = Set(v.to_owned()); }
        Ok(active.update(self.db).await?)
    }

    pub async fn delete_vlan(&self, id: i64) -> Result<bool, AppError> {
        let result = vlan_entity::Entity::delete_by_id(id).exec(self.db).await?;
        Ok(result.rows_affected > 0)
    }

    // ── IP Pools ─────────────────────────────────────────────

    pub async fn list_ip_pools(&self, branch_id: Option<i64>) -> Result<Vec<IpPoolModel>, AppError> {
        let mut select = ip_pool_entity::Entity::find();
        if let Some(bid) = branch_id { select = select.filter(ip_pool_entity::Column::BranchId.eq(bid)); }
        Ok(select.order_by_desc(ip_pool_entity::Column::CreatedAt).all(self.db).await?)
    }

    pub async fn create_ip_pool(&self, branch_id: i64, name: &str, cidr: &str, gateway: &str, dns_primary: Option<&str>, dns_secondary: Option<&str>, vlan_id: Option<i64>, pool_type: &str, total_count: i32) -> Result<IpPoolModel, AppError> {
        let now = chrono::Utc::now();
        let active = ip_pool_entity::ActiveModel {
            branch_id: Set(branch_id),
            name: Set(name.to_owned()),
            cidr: Set(cidr.to_owned()),
            gateway: Set(gateway.to_owned()),
            dns_primary: Set(dns_primary.map(|s| s.to_owned())),
            dns_secondary: Set(dns_secondary.map(|s| s.to_owned())),
            vlan_id: Set(vlan_id),
            pool_type: Set(pool_type.to_owned()),
            total_count: Set(total_count),
            status: Set("active".to_owned()),
            is_active: Set(true),
            created_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    // ── IP Addresses ─────────────────────────────────────────

    pub async fn list_ip_addresses(&self, pool_id: i64, status: Option<&str>) -> Result<Vec<IpAddressModel>, AppError> {
        let mut select = ip_address_entity::Entity::find()
            .filter(ip_address_entity::Column::IpPoolId.eq(pool_id));
        if let Some(s) = status { select = select.filter(ip_address_entity::Column::Status.eq(s)); }
        Ok(select.order_by_asc(ip_address_entity::Column::IpAddress).all(self.db).await?)
    }

    pub async fn allocate_ip(&self, pool_id: i64, allocated_to_type: &str, allocated_to_id: i64) -> Result<IpAddressModel, AppError> {
        let available = ip_address_entity::Entity::find()
            .filter(ip_address_entity::Column::IpPoolId.eq(pool_id))
            .filter(ip_address_entity::Column::Status.eq("available"))
            .order_by_asc(ip_address_entity::Column::IpAddress)
            .one(self.db).await?
            .ok_or_else(|| AppError::NotFound("No available IP address".into()))?;
        let mut active = available.into_active_model();
        active.status = Set("allocated".to_owned());
        active.allocated_to_type = Set(Some(allocated_to_type.to_owned()));
        active.allocated_to_id = Set(Some(allocated_to_id));
        active.allocated_at = Set(Some(chrono::Utc::now().into()));
        let result = active.update(self.db).await?;
        // Update pool allocated count
        if let Some(pool) = ip_pool_entity::Entity::find_by_id(pool_id).one(self.db).await? {
            let mut pool_active = pool.into_active_model();
            pool_active.allocated_count = Set(pool_active.allocated_count.clone().unwrap_or(0) + 1);
            pool_active.update(self.db).await?;
        }
        Ok(result)
    }

    // ── PPPoE Sessions ───────────────────────────────────────

    pub async fn list_pppoe_sessions(&self, branch_id: Option<i64>, page: i64, per_page: i64) -> Result<(Vec<PppoeSessionModel>, i64), AppError> {
        let page_size = per_page as u64;
        let page_num = if per_page > 0 { ((page - 1).max(0) as u64 * page_size) / page_size } else { 0 };
        let mut select = pppoe_session_entity::Entity::find();
        if let Some(bid) = branch_id { select = select.filter(pppoe_session_entity::Column::BranchId.eq(bid)); }
        let total = select.clone().count(self.db).await?;
        let sessions = select.order_by_desc(pppoe_session_entity::Column::CreatedAt).paginate(self.db, page_size).fetch_page(page_num).await?;
        Ok((sessions, total as i64))
    }

    pub async fn create_pppoe_session(&self, branch_id: i64, customer_id: i64, subscription_id: i64, username: &str, password: &str) -> Result<PppoeSessionModel, AppError> {
        let now = chrono::Utc::now();
        let active = pppoe_session_entity::ActiveModel {
            branch_id: Set(branch_id),
            customer_id: Set(customer_id),
            subscription_id: Set(subscription_id),
            username: Set(username.to_owned()),
            password_encrypted: Set(Some(password.to_owned())),
            status: Set("active".to_owned()),
            created_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn terminate_session(&self, id: i64) -> Result<bool, AppError> {
        let existing = pppoe_session_entity::Entity::find_by_id(id).one(self.db).await?;
        match existing {
            Some(e) => {
                let mut active = e.into_active_model();
                active.status = Set("terminated".to_owned());
                active.update(self.db).await?;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    // ── MAC Bindings ─────────────────────────────────────────

    pub async fn list_mac_bindings(&self, branch_id: Option<i64>, page: i64, per_page: i64) -> Result<(Vec<MacBindingModel>, i64), AppError> {
        let page_size = per_page as u64;
        let page_num = if per_page > 0 { ((page - 1).max(0) as u64 * page_size) / page_size } else { 0 };
        let mut select = mac_binding_entity::Entity::find();
        if let Some(bid) = branch_id { select = select.filter(mac_binding_entity::Column::BranchId.eq(bid)); }
        let total = select.clone().count(self.db).await?;
        let bindings = select.order_by_desc(mac_binding_entity::Column::CreatedAt).paginate(self.db, page_size).fetch_page(page_num).await?;
        Ok((bindings, total as i64))
    }

    pub async fn create_mac_binding(&self, branch_id: i64, customer_id: i64, subscription_id: i64, mac_address: &str, assigned_ip: &str, vlan_id: Option<i64>) -> Result<MacBindingModel, AppError> {
        let now = chrono::Utc::now();
        let active = mac_binding_entity::ActiveModel {
            branch_id: Set(branch_id),
            customer_id: Set(customer_id),
            subscription_id: Set(subscription_id),
            mac_address: Set(mac_address.to_owned()),
            assigned_ip: Set(assigned_ip.to_owned()),
            vlan_id: Set(vlan_id),
            is_active: Set(true),
            bound_at: Set(now.into()),
            created_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn delete_mac_binding(&self, id: i64) -> Result<bool, AppError> {
        let existing = mac_binding_entity::Entity::find_by_id(id).one(self.db).await?;
        match existing {
            Some(e) => {
                let mut active = e.into_active_model();
                active.is_active = Set(false);
                active.update(self.db).await?;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    // ── DHCP Leases ──────────────────────────────────────────

    pub async fn list_dhcp_leases(&self, branch_id: Option<i64>, page: i64, per_page: i64) -> Result<(Vec<DhcpLeaseModel>, i64), AppError> {
        let page_size = per_page as u64;
        let page_num = if per_page > 0 { ((page - 1).max(0) as u64 * page_size) / page_size } else { 0 };
        let mut select = dhcp_lease_entity::Entity::find();
        if let Some(bid) = branch_id { select = select.filter(dhcp_lease_entity::Column::BranchId.eq(bid)); }
        let total = select.clone().count(self.db).await?;
        let leases = select.order_by_desc(dhcp_lease_entity::Column::CreatedAt).paginate(self.db, page_size).fetch_page(page_num).await?;
        Ok((leases, total as i64))
    }

    // ── Customer Sessions ────────────────────────────────────

    pub async fn list_customer_sessions(&self, branch_id: Option<i64>, is_online: Option<bool>, page: i64, per_page: i64) -> Result<(Vec<CustomerSessionModel>, i64), AppError> {
        let page_size = per_page as u64;
        let page_num = if per_page > 0 { ((page - 1).max(0) as u64 * page_size) / page_size } else { 0 };
        let mut select = customer_session_entity::Entity::find();
        if let Some(bid) = branch_id { select = select.filter(customer_session_entity::Column::BranchId.eq(bid)); }
        if let Some(online) = is_online { select = select.filter(customer_session_entity::Column::IsOnline.eq(online)); }
        let total = select.clone().count(self.db).await?;
        let sessions = select.order_by_desc(customer_session_entity::Column::LastActivityAt).paginate(self.db, page_size).fetch_page(page_num).await?;
        Ok((sessions, total as i64))
    }
}
