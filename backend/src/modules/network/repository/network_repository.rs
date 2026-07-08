use sqlx::PgPool;
use crate::modules::network::model::network::{Vlan, IpPool, PppoeSession};

pub struct NetworkRepository<'a> { pool: &'a PgPool }
impl<'a> NetworkRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }
    pub fn pool(&self) -> &'a PgPool { self.pool }

    pub async fn list_vlans(&self, branch_id: Option<i64>) -> Result<Vec<Vlan>, sqlx::Error> {
        sqlx::query_as::<_, Vlan>("SELECT * FROM vlans WHERE ($1::bigint IS NULL OR branch_id = $1) ORDER BY vlan_id").bind(branch_id).fetch_all(self.pool).await
    }

    pub async fn create_vlan(&self, branch_id: i64, vlan_id: i32, name: &str, description: Option<&str>, vlan_type: &str) -> Result<Vlan, sqlx::Error> {
        sqlx::query_as::<_, Vlan>("INSERT INTO vlans (branch_id, vlan_id, name, description, vlan_type) VALUES ($1,$2,$3,$4,$5) RETURNING *")
            .bind(branch_id).bind(vlan_id).bind(name).bind(description).bind(vlan_type).fetch_one(self.pool).await
    }

    pub async fn delete_vlan(&self, id: i64) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("DELETE FROM vlans WHERE id = $1").bind(id).execute(self.pool).await?;
        Ok(r.rows_affected() > 0)
    }

    pub async fn list_ip_pools(&self, branch_id: Option<i64>) -> Result<Vec<IpPool>, sqlx::Error> {
        sqlx::query_as::<_, IpPool>("SELECT * FROM ip_pools WHERE ($1::bigint IS NULL OR branch_id = $1) ORDER BY created_at DESC").bind(branch_id).fetch_all(self.pool).await
    }

    pub async fn create_ip_pool(&self, branch_id: i64, name: &str, cidr: &str, gateway: &str, dns_primary: Option<&str>, dns_secondary: Option<&str>, vlan_id: Option<i64>, pool_type: &str, total_count: i32) -> Result<IpPool, sqlx::Error> {
        sqlx::query_as::<_, IpPool>("INSERT INTO ip_pools (branch_id, name, cidr, gateway, dns_primary, dns_secondary, vlan_id, pool_type, total_count) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) RETURNING *")
            .bind(branch_id).bind(name).bind(cidr).bind(gateway).bind(dns_primary).bind(dns_secondary).bind(vlan_id).bind(pool_type).bind(total_count).fetch_one(self.pool).await
    }

    pub async fn list_pppoe_sessions(&self, branch_id: Option<i64>, page: i64, per_page: i64) -> Result<(Vec<PppoeSession>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM pppoe_sessions WHERE ($1::bigint IS NULL OR branch_id = $1)").bind(branch_id).fetch_one(self.pool).await?;
        let sessions: Vec<PppoeSession> = sqlx::query_as("SELECT * FROM pppoe_sessions WHERE ($1::bigint IS NULL OR branch_id = $1) ORDER BY created_at DESC LIMIT $2 OFFSET $3")
            .bind(branch_id).bind(per_page).bind(offset).fetch_all(self.pool).await?;
        Ok((sessions, count_row.0))
    }

    pub async fn terminate_session(&self, id: i64) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("UPDATE pppoe_sessions SET status = 'terminated' WHERE id = $1").bind(id).execute(self.pool).await?;
        Ok(r.rows_affected() > 0)
    }
}
