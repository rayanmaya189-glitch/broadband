use sqlx::PgPool;
use crate::modules::network::model::network::*;

pub struct NetworkRepository<'a> { pool: &'a PgPool }
impl<'a> NetworkRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }
    pub fn pool(&self) -> &'a PgPool { self.pool }

    // ── VLANs ────────────────────────────────────────────────

    pub async fn list_vlans(&self, branch_id: Option<i64>) -> Result<Vec<Vlan>, sqlx::Error> {
        sqlx::query_as::<_, Vlan>(
            "SELECT id, branch_id, vlan_id, name, description, vlan_type, is_active, created_at FROM vlans WHERE ($1::bigint IS NULL OR branch_id = $1) ORDER BY vlan_id"
        ).bind(branch_id).fetch_all(self.pool).await
    }

    pub async fn create_vlan(&self, branch_id: i64, vlan_id: i32, name: &str, description: Option<&str>, vlan_type: &str) -> Result<Vlan, sqlx::Error> {
        sqlx::query_as::<_, Vlan>(
            "INSERT INTO vlans (branch_id, vlan_id, name, description, vlan_type) VALUES ($1,$2,$3,$4,$5) RETURNING id, branch_id, vlan_id, name, description, vlan_type, is_active, created_at"
        ).bind(branch_id).bind(vlan_id).bind(name).bind(description).bind(vlan_type).fetch_one(self.pool).await
    }

    pub async fn update_vlan(&self, id: i64, name: Option<&str>, description: Option<&str>, vlan_type: Option<&str>) -> Result<Vlan, sqlx::Error> {
        sqlx::query_as::<_, Vlan>(
            "UPDATE vlans SET name = COALESCE($2, name), description = COALESCE($3, description), vlan_type = COALESCE($4, vlan_type) WHERE id = $1 RETURNING id, branch_id, vlan_id, name, description, vlan_type, is_active, created_at"
        ).bind(id).bind(name).bind(description).bind(vlan_type).fetch_one(self.pool).await
    }

    pub async fn delete_vlan(&self, id: i64) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("DELETE FROM vlans WHERE id = $1").bind(id).execute(self.pool).await?;
        Ok(r.rows_affected() > 0)
    }

    // ── IP Pools ─────────────────────────────────────────────

    pub async fn list_ip_pools(&self, branch_id: Option<i64>) -> Result<Vec<IpPool>, sqlx::Error> {
        sqlx::query_as::<_, IpPool>(
            "SELECT id, branch_id, name, cidr, gateway, dns_primary, dns_secondary, vlan_id, pool_type, allocated_count, total_count, status, is_active, created_at FROM ip_pools WHERE ($1::bigint IS NULL OR branch_id = $1) ORDER BY created_at DESC"
        ).bind(branch_id).fetch_all(self.pool).await
    }

    pub async fn create_ip_pool(&self, branch_id: i64, name: &str, cidr: &str, gateway: &str, dns_primary: Option<&str>, dns_secondary: Option<&str>, vlan_id: Option<i64>, pool_type: &str, total_count: i32) -> Result<IpPool, sqlx::Error> {
        sqlx::query_as::<_, IpPool>(
            "INSERT INTO ip_pools (branch_id, name, cidr, gateway, dns_primary, dns_secondary, vlan_id, pool_type, total_count) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) RETURNING id, branch_id, name, cidr, gateway, dns_primary, dns_secondary, vlan_id, pool_type, allocated_count, total_count, status, is_active, created_at"
        ).bind(branch_id).bind(name).bind(cidr).bind(gateway).bind(dns_primary).bind(dns_secondary).bind(vlan_id).bind(pool_type).bind(total_count).fetch_one(self.pool).await
    }

    // ── IP Addresses ─────────────────────────────────────────

    pub async fn list_ip_addresses(&self, pool_id: i64, status: Option<&str>) -> Result<Vec<IpAddress>, sqlx::Error> {
        sqlx::query_as::<_, IpAddress>(
            "SELECT id, ip_pool_id, ip_address, status, allocated_to_type, allocated_to_id, allocated_at, released_at, created_at FROM ip_addresses WHERE ip_pool_id = $1 AND ($2::text IS NULL OR status = $2) ORDER BY ip_address"
        ).bind(pool_id).bind(status).fetch_all(self.pool).await
    }

    pub async fn allocate_ip(&self, pool_id: i64, allocated_to_type: &str, allocated_to_id: i64) -> Result<IpAddress, sqlx::Error> {
        let row = sqlx::query_as::<_, IpAddress>(
            "UPDATE ip_addresses SET status = 'allocated', allocated_to_type = $2, allocated_to_id = $3, allocated_at = NOW() WHERE ip_pool_id = $1 AND status = 'available' ORDER BY ip_address LIMIT 1 RETURNING id, ip_pool_id, ip_address, status, allocated_to_type, allocated_to_id, allocated_at, released_at, created_at"
        ).bind(pool_id).bind(allocated_to_type).bind(allocated_to_id).fetch_one(self.pool).await?;
        sqlx::query("UPDATE ip_pools SET allocated_count = allocated_count + 1 WHERE id = $1").bind(pool_id).execute(self.pool).await?;
        Ok(row)
    }

    pub async fn release_ip(&self, ip_id: i64, pool_id: i64) -> Result<bool, sqlx::Error> {
        let r = sqlx::query(
            "UPDATE ip_addresses SET status = 'available', allocated_to_type = NULL, allocated_to_id = NULL, released_at = NOW() WHERE id = $1 AND ip_pool_id = $2"
        ).bind(ip_id).bind(pool_id).execute(self.pool).await?;
        if r.rows_affected() > 0 {
            sqlx::query("UPDATE ip_pools SET allocated_count = GREATEST(allocated_count - 1, 0) WHERE id = $1").bind(pool_id).execute(self.pool).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // ── PPPoE Sessions ───────────────────────────────────────

    pub async fn list_pppoe_sessions(&self, branch_id: Option<i64>, page: i64, per_page: i64) -> Result<(Vec<PppoeSession>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM pppoe_sessions WHERE ($1::bigint IS NULL OR branch_id = $1)"
        ).bind(branch_id).fetch_one(self.pool).await?;
        let sessions: Vec<PppoeSession> = sqlx::query_as(
            "SELECT id, branch_id, customer_id, subscription_id, username, assigned_ip, status, session_start, bytes_in, bytes_out, created_at FROM pppoe_sessions WHERE ($1::bigint IS NULL OR branch_id = $1) ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        ).bind(branch_id).bind(per_page).bind(offset).fetch_all(self.pool).await?;
        Ok((sessions, count_row.0))
    }

    pub async fn create_pppoe_session(&self, branch_id: i64, customer_id: i64, subscription_id: i64, username: &str, password: &str) -> Result<PppoeSession, sqlx::Error> {
        sqlx::query_as::<_, PppoeSession>(
            "INSERT INTO pppoe_sessions (branch_id, customer_id, subscription_id, username, password_encrypted, status) VALUES ($1,$2,$3,$4,$5,'active') RETURNING id, branch_id, customer_id, subscription_id, username, assigned_ip, status, session_start, bytes_in, bytes_out, created_at"
        ).bind(branch_id).bind(customer_id).bind(subscription_id).bind(username).bind(password).fetch_one(self.pool).await
    }

    pub async fn terminate_session(&self, id: i64) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("UPDATE pppoe_sessions SET status = 'terminated' WHERE id = $1").bind(id).execute(self.pool).await?;
        Ok(r.rows_affected() > 0)
    }

    // ── MAC Bindings ─────────────────────────────────────────

    pub async fn list_mac_bindings(&self, branch_id: Option<i64>, page: i64, per_page: i64) -> Result<(Vec<MacBinding>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM mac_bindings WHERE ($1::bigint IS NULL OR branch_id = $1)"
        ).bind(branch_id).fetch_one(self.pool).await?;
        let bindings: Vec<MacBinding> = sqlx::query_as(
            "SELECT id, branch_id, customer_id, subscription_id, mac_address, assigned_ip, vlan_id, is_active, bound_at, created_at FROM mac_bindings WHERE ($1::bigint IS NULL OR branch_id = $1) ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        ).bind(branch_id).bind(per_page).bind(offset).fetch_all(self.pool).await?;
        Ok((bindings, count_row.0))
    }

    pub async fn create_mac_binding(&self, branch_id: i64, customer_id: i64, subscription_id: i64, mac_address: &str, assigned_ip: &str, vlan_id: Option<i64>) -> Result<MacBinding, sqlx::Error> {
        sqlx::query_as::<_, MacBinding>(
            "INSERT INTO mac_bindings (branch_id, customer_id, subscription_id, mac_address, assigned_ip, vlan_id) VALUES ($1,$2,$3,$4,$5,$6) RETURNING id, branch_id, customer_id, subscription_id, mac_address, assigned_ip, vlan_id, is_active, bound_at, created_at"
        ).bind(branch_id).bind(customer_id).bind(subscription_id).bind(mac_address).bind(assigned_ip).bind(vlan_id).fetch_one(self.pool).await
    }

    pub async fn delete_mac_binding(&self, id: i64) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("UPDATE mac_bindings SET is_active = false WHERE id = $1").bind(id).execute(self.pool).await?;
        Ok(r.rows_affected() > 0)
    }

    // ── DHCP Leases ──────────────────────────────────────────

    pub async fn list_dhcp_leases(&self, branch_id: Option<i64>, page: i64, per_page: i64) -> Result<(Vec<DhcpLease>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM dhcp_leases WHERE ($1::bigint IS NULL OR branch_id = $1)"
        ).bind(branch_id).fetch_one(self.pool).await?;
        let leases: Vec<DhcpLease> = sqlx::query_as(
            "SELECT id, branch_id, mac_address, ip_address, hostname, vlan_id, ip_pool_id, lease_start, lease_end, lease_type, customer_id, subscription_id, status, created_at FROM dhcp_leases WHERE ($1::bigint IS NULL OR branch_id = $1) ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        ).bind(branch_id).bind(per_page).bind(offset).fetch_all(self.pool).await?;
        Ok((leases, count_row.0))
    }

    // ── Customer Sessions ────────────────────────────────────

    pub async fn list_customer_sessions(&self, branch_id: Option<i64>, is_online: Option<bool>, page: i64, per_page: i64) -> Result<(Vec<CustomerSession>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM customer_sessions WHERE ($1::bigint IS NULL OR branch_id = $1) AND ($2::bool IS NULL OR is_online = $2)"
        ).bind(branch_id).bind(is_online).fetch_one(self.pool).await?;
        let sessions: Vec<CustomerSession> = sqlx::query_as(
            "SELECT id, branch_id, customer_id, subscription_id, mac_address, ip_address, connected_at, disconnected_at, last_activity_at, bytes_in, bytes_out, is_online, created_at FROM customer_sessions WHERE ($1::bigint IS NULL OR branch_id = $1) AND ($2::bool IS NULL OR is_online = $2) ORDER BY last_activity_at DESC LIMIT $3 OFFSET $4"
        ).bind(branch_id).bind(is_online).bind(per_page).bind(offset).fetch_all(self.pool).await?;
        Ok((sessions, count_row.0))
    }

    // ── Network Topology ─────────────────────────────────────

    pub async fn get_topology(&self) -> Result<NetworkTopology, sqlx::Error> {
        let row = sqlx::query_as::<_, (i64, i64, i64, i64, i64, i64, i64)>(
            "SELECT
                (SELECT COUNT(*) FROM vlans WHERE is_active = true) as total_vlans,
                (SELECT COUNT(*) FROM ip_pools WHERE is_active = true) as total_ip_pools,
                (SELECT COUNT(*) FROM customer_sessions WHERE is_online = true) as total_active_sessions,
                (SELECT COUNT(*) FROM mac_bindings WHERE is_active = true) as total_mac_bindings,
                (SELECT COUNT(*) FROM pppoe_sessions WHERE status = 'active') as active_pppoe_sessions,
                (SELECT COUNT(*) FROM dhcp_leases WHERE status = 'active') as active_dhcp_leases,
                (SELECT COUNT(DISTINCT customer_id) FROM customer_sessions WHERE is_online = true) as online_customers"
        ).fetch_one(self.pool).await?;
        Ok(NetworkTopology {
            total_vlans: row.0,
            total_ip_pools: row.1,
            total_active_sessions: row.2,
            total_mac_bindings: row.3,
            active_pppoe_sessions: row.4,
            active_dhcp_leases: row.5,
            online_customers: row.6,
        })
    }
}
