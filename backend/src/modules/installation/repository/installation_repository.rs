use sqlx::PgPool;
use crate::modules::installation::model::installation::*;

pub struct InstallationRepository<'a> { pool: &'a PgPool }
impl<'a> InstallationRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }

    pub async fn list(&self, branch_id: Option<i64>, status: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<InstallationOrder>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM installation_orders WHERE ($1::bigint IS NULL OR branch_id = $1) AND ($2::text IS NULL OR status = $2)")
            .bind(branch_id).bind(status).fetch_one(self.pool).await?;
        let orders: Vec<InstallationOrder> = sqlx::query_as("SELECT id, customer_id, branch_id, subscription_id, assigned_technician_id, status, scheduled_date, scheduled_time_slot, completed_at, installation_type, equipment_issued, fiber_drop_length_meters, onu_power_dbm, notes, photos, created_at, updated_at FROM installation_orders WHERE ($1::bigint IS NULL OR branch_id = $1) AND ($2::text IS NULL OR status = $2) ORDER BY created_at DESC LIMIT $3 OFFSET $4")
            .bind(branch_id).bind(status).bind(per_page).bind(offset).fetch_all(self.pool).await?;
        Ok((orders, count_row.0))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<InstallationOrder>, sqlx::Error> {
        sqlx::query_as::<_, InstallationOrder>("SELECT id, customer_id, branch_id, subscription_id, assigned_technician_id, status, scheduled_date, scheduled_time_slot, completed_at, installation_type, equipment_issued, fiber_drop_length_meters, onu_power_dbm, notes, photos, created_at, updated_at FROM installation_orders WHERE id = $1")
            .bind(id).fetch_optional(self.pool).await
    }

    pub async fn create(&self, customer_id: i64, branch_id: i64, subscription_id: Option<i64>, installation_type: &str) -> Result<InstallationOrder, sqlx::Error> {
        sqlx::query_as::<_, InstallationOrder>("INSERT INTO installation_orders (customer_id, branch_id, subscription_id, installation_type) VALUES ($1,$2,$3,$4) RETURNING id, customer_id, branch_id, subscription_id, assigned_technician_id, status, scheduled_date, scheduled_time_slot, completed_at, installation_type, equipment_issued, fiber_drop_length_meters, onu_power_dbm, notes, photos, created_at, updated_at")
            .bind(customer_id).bind(branch_id).bind(subscription_id).bind(installation_type).fetch_one(self.pool).await
    }

    pub async fn schedule(&self, id: i64, scheduled_date: chrono::NaiveDate, time_slot: &str, technician_id: Option<i64>) -> Result<InstallationOrder, sqlx::Error> {
        sqlx::query_as::<_, InstallationOrder>("UPDATE installation_orders SET status = 'scheduled', scheduled_date = $2, scheduled_time_slot = $3, assigned_technician_id = COALESCE($4, assigned_technician_id), updated_at = NOW() WHERE id = $1 RETURNING id, customer_id, branch_id, subscription_id, assigned_technician_id, status, scheduled_date, scheduled_time_slot, completed_at, installation_type, equipment_issued, fiber_drop_length_meters, onu_power_dbm, notes, photos, created_at, updated_at")
            .bind(id).bind(scheduled_date).bind(time_slot).bind(technician_id).fetch_one(self.pool).await
    }

    pub async fn reschedule(&self, id: i64, scheduled_date: chrono::NaiveDate, time_slot: &str, reason: Option<&str>) -> Result<InstallationOrder, sqlx::Error> {
        if let Some(r) = reason {
            sqlx::query("UPDATE installation_orders SET notes = COALESCE(notes, '') || E'\n[Rescheduled] ' || $2, updated_at = NOW() WHERE id = $1")
                .bind(id).bind(r).execute(self.pool).await?;
        }
        sqlx::query_as::<_, InstallationOrder>("UPDATE installation_orders SET status = 'scheduled', scheduled_date = $2, scheduled_time_slot = $3, updated_at = NOW() WHERE id = $1 RETURNING id, customer_id, branch_id, subscription_id, assigned_technician_id, status, scheduled_date, scheduled_time_slot, completed_at, installation_type, equipment_issued, fiber_drop_length_meters, onu_power_dbm, notes, photos, created_at, updated_at")
            .bind(id).bind(scheduled_date).bind(time_slot).fetch_one(self.pool).await
    }

    pub async fn start(&self, id: i64) -> Result<InstallationOrder, sqlx::Error> {
        sqlx::query_as::<_, InstallationOrder>("UPDATE installation_orders SET status = 'in_progress', updated_at = NOW() WHERE id = $1 RETURNING id, customer_id, branch_id, subscription_id, assigned_technician_id, status, scheduled_date, scheduled_time_slot, completed_at, installation_type, equipment_issued, fiber_drop_length_meters, onu_power_dbm, notes, photos, created_at, updated_at")
            .bind(id).fetch_one(self.pool).await
    }

    pub async fn complete(&self, id: i64, fiber_length: Option<i32>, onu_power: Option<f64>, equipment: Option<serde_json::Value>, notes: Option<&str>) -> Result<InstallationOrder, sqlx::Error> {
        sqlx::query_as::<_, InstallationOrder>("UPDATE installation_orders SET status = 'completed', completed_at = NOW(), fiber_drop_length_meters = COALESCE($2, fiber_drop_length_meters), onu_power_dbm = COALESCE($3, onu_power_dbm), equipment_issued = COALESCE($4, equipment_issued), notes = COALESCE($5, notes), updated_at = NOW() WHERE id = $1 RETURNING id, customer_id, branch_id, subscription_id, assigned_technician_id, status, scheduled_date, scheduled_time_slot, completed_at, installation_type, equipment_issued, fiber_drop_length_meters, onu_power_dbm, notes, photos, created_at, updated_at")
            .bind(id).bind(fiber_length).bind(onu_power).bind(equipment).bind(notes).fetch_one(self.pool).await
    }

    pub async fn cancel(&self, id: i64) -> Result<InstallationOrder, sqlx::Error> {
        sqlx::query_as::<_, InstallationOrder>("UPDATE installation_orders SET status = 'cancelled', updated_at = NOW() WHERE id = $1 RETURNING id, customer_id, branch_id, subscription_id, assigned_technician_id, status, scheduled_date, scheduled_time_slot, completed_at, installation_type, equipment_issued, fiber_drop_length_meters, onu_power_dbm, notes, photos, created_at, updated_at")
            .bind(id).fetch_one(self.pool).await
    }

    pub async fn add_photo(&self, id: i64, photo_url: &str) -> Result<InstallationOrder, sqlx::Error> {
        sqlx::query_as::<_, InstallationOrder>("UPDATE installation_orders SET photos = COALESCE(photos, '{}') || ARRAY[$2]::text[], updated_at = NOW() WHERE id = $1 RETURNING id, customer_id, branch_id, subscription_id, assigned_technician_id, status, scheduled_date, scheduled_time_slot, completed_at, installation_type, equipment_issued, fiber_drop_length_meters, onu_power_dbm, notes, photos, created_at, updated_at")
            .bind(id).bind(photo_url).fetch_one(self.pool).await
    }

    pub async fn get_my_assignments(&self, technician_id: i64) -> Result<Vec<InstallationOrder>, sqlx::Error> {
        sqlx::query_as::<_, InstallationOrder>("SELECT id, customer_id, branch_id, subscription_id, assigned_technician_id, status, scheduled_date, scheduled_time_slot, completed_at, installation_type, equipment_issued, fiber_drop_length_meters, onu_power_dbm, notes, photos, created_at, updated_at FROM installation_orders WHERE assigned_technician_id = $1 AND status IN ('scheduled', 'in_progress') ORDER BY scheduled_date ASC")
            .bind(technician_id).fetch_all(self.pool).await
    }
}
