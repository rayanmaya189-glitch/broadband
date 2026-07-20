use crate::modules::network::domain::entities::{
    IpPool, IpPoolActiveModel, IpPoolColumn, MacBinding, MacBindingActiveModel, MacBindingColumn,
    PppoeSession, PppoeSessionActiveModel, PppoeSessionColumn, Vlan, VlanActiveModel, VlanColumn,
};
use crate::shared::errors::AppError;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub struct NetworkService;

impl NetworkService {
    // --- VLANs ---
    pub async fn list_vlans(
        db: &DatabaseConnection,
        branch_id: Option<i64>,
    ) -> Result<Vec<crate::modules::network::domain::entities::vlan::Model>, AppError> {
        let mut query = Vlan::find();
        if let Some(bid) = branch_id {
            query = query.filter(VlanColumn::BranchId.eq(bid));
        }
        Ok(query.all(db).await?)
    }

    pub async fn create_vlan(
        db: &DatabaseConnection,
        branch_id: i64,
        vlan_id: i32,
        name: String,
        vlan_type: String,
    ) -> Result<crate::modules::network::domain::entities::vlan::Model, AppError> {
        let now = chrono::Utc::now();
        let new_vlan = VlanActiveModel {
            branch_id: Set(branch_id),
            vlan_id: Set(vlan_id),
            name: Set(name),
            vlan_type: Set(vlan_type),
            is_active: Set(true),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(new_vlan.insert(db).await?)
    }

    pub async fn delete_vlan(db: &DatabaseConnection, id: i64) -> Result<(), AppError> {
        let vlan = Vlan::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("VLAN {} not found", id)))?;
        let mut active = <crate::modules::network::domain::entities::vlan::Entity as sea_orm::EntityTrait>::ActiveModel::from(vlan);
        active.is_active = Set(false);
        active.updated_at = Set(chrono::Utc::now());
        active.update(db).await?;
        Ok(())
    }

    // --- IP Pools ---
    pub async fn list_ip_pools(
        db: &DatabaseConnection,
        branch_id: Option<i64>,
    ) -> Result<Vec<crate::modules::network::domain::entities::ip_pool::Model>, AppError> {
        let mut query = IpPool::find();
        if let Some(bid) = branch_id {
            query = query.filter(IpPoolColumn::BranchId.eq(bid));
        }
        Ok(query.all(db).await?)
    }

    pub async fn create_ip_pool(
        db: &DatabaseConnection,
        branch_id: i64,
        name: String,
        cidr: String,
        gateway: String,
        vlan_id: Option<i64>,
        pool_type: String,
        total_count: i32,
    ) -> Result<crate::modules::network::domain::entities::ip_pool::Model, AppError> {
        let now = chrono::Utc::now();
        let new_pool = IpPoolActiveModel {
            branch_id: Set(branch_id),
            name: Set(name),
            cidr: Set(cidr),
            gateway: Set(gateway),
            vlan_id: Set(vlan_id),
            pool_type: Set(pool_type),
            total_count: Set(total_count),
            status: Set("healthy".to_string()),
            is_active: Set(true),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(new_pool.insert(db).await?)
    }

    // --- PPPoE Sessions ---
    pub async fn list_pppoe_sessions(
        db: &DatabaseConnection,
        branch_id: Option<i64>,
    ) -> Result<Vec<crate::modules::network::domain::entities::pppoe_session::Model>, AppError>
    {
        let mut query = PppoeSession::find();
        if let Some(bid) = branch_id {
            query = query.filter(PppoeSessionColumn::BranchId.eq(bid));
        }
        Ok(query.all(db).await?)
    }

    pub async fn create_pppoe_session(
        db: &DatabaseConnection,
        branch_id: i64,
        customer_id: i64,
        subscription_id: i64,
        username: String,
        password_encrypted: String,
    ) -> Result<crate::modules::network::domain::entities::pppoe_session::Model, AppError> {
        let now = chrono::Utc::now();
        let session = PppoeSessionActiveModel {
            branch_id: Set(branch_id),
            customer_id: Set(customer_id),
            subscription_id: Set(subscription_id),
            username: Set(username),
            password_encrypted: Set(password_encrypted),
            status: Set("active".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(session.insert(db).await?)
    }

    pub async fn terminate_pppoe_session(db: &DatabaseConnection, id: i64) -> Result<(), AppError> {
        let session = PppoeSession::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Session {} not found", id)))?;
        let mut active = <crate::modules::network::domain::entities::pppoe_session::Entity as sea_orm::EntityTrait>::ActiveModel::from(session);
        active.status = Set("terminated".to_string());
        active.updated_at = Set(chrono::Utc::now());
        active.update(db).await?;
        Ok(())
    }

    // --- MAC Bindings ---
    pub async fn list_mac_bindings(
        db: &DatabaseConnection,
        branch_id: Option<i64>,
    ) -> Result<Vec<crate::modules::network::domain::entities::mac_binding::Model>, AppError> {
        let mut query = MacBinding::find();
        if let Some(bid) = branch_id {
            query = query.filter(MacBindingColumn::BranchId.eq(bid));
        }
        Ok(query.all(db).await?)
    }

    pub async fn create_mac_binding(
        db: &DatabaseConnection,
        branch_id: i64,
        customer_id: i64,
        subscription_id: i64,
        mac_address: String,
        assigned_ip: String,
    ) -> Result<crate::modules::network::domain::entities::mac_binding::Model, AppError> {
        let now = chrono::Utc::now();
        let binding = MacBindingActiveModel {
            branch_id: Set(branch_id),
            customer_id: Set(customer_id),
            subscription_id: Set(subscription_id),
            mac_address: Set(mac_address),
            assigned_ip: Set(assigned_ip),
            bound_at: Set(now),
            is_active: Set(true),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(binding.insert(db).await?)
    }

    pub async fn get_topology(db: &DatabaseConnection, branch_id: Option<i64>) -> Result<serde_json::Value, AppError> {
        let vlans = Self::list_vlans(db, branch_id).await?;
        let pools = Self::list_ip_pools(db, branch_id).await?;
        let sessions = Self::list_pppoe_sessions(db, branch_id).await?;
        let bindings = Self::list_mac_bindings(db, branch_id).await?;

        Ok(serde_json::json!({
            "vlans": {
                "count": vlans.len(),
                "items": vlans.iter().map(|v| serde_json::json!({
                    "id": v.id,
                    "vlan_id": v.vlan_id,
                    "name": v.name,
                    "type": v.vlan_type,
                    "is_active": v.is_active,
                })).collect::<Vec<_>>(),
            },
            "ip_pools": {
                "count": pools.len(),
                "items": pools.iter().map(|p| serde_json::json!({
                    "id": p.id,
                    "name": p.name,
                    "cidr": p.cidr,
                    "gateway": p.gateway,
                    "allocated": p.allocated_count,
                    "total": p.total_count,
                    "status": p.status,
                })).collect::<Vec<_>>(),
            },
            "pppoe_sessions": {
                "count": sessions.len(),
                "active": sessions.iter().filter(|s| s.status == "active").count(),
                "items": sessions.iter().map(|s| serde_json::json!({
                    "id": s.id,
                    "username": s.username,
                    "assigned_ip": s.assigned_ip,
                    "status": s.status,
                })).collect::<Vec<_>>(),
            },
            "mac_bindings": {
                "count": bindings.len(),
                "active": bindings.iter().filter(|b| b.is_active).count(),
                "items": bindings.iter().map(|b| serde_json::json!({
                    "id": b.id,
                    "mac_address": b.mac_address,
                    "assigned_ip": b.assigned_ip,
                    "is_active": b.is_active,
                })).collect::<Vec<_>>(),
            },
        }))
    }
}
