use sqlx::PgPool;
use crate::modules::device::model::device::{DeviceModel, NetworkDevice};

pub struct DeviceRepository<'a> { pool: &'a PgPool }
impl<'a> DeviceRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }
    pub fn pool(&self) -> &'a PgPool { self.pool }

    pub async fn list_devices(&self, branch_id: Option<i64>, status: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<NetworkDevice>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM network_devices WHERE ($1::bigint IS NULL OR branch_id = $1) AND ($2::text IS NULL OR status = $2)")
            .bind(branch_id).bind(status).fetch_one(self.pool).await?;
        let devices: Vec<NetworkDevice> = sqlx::query_as("SELECT * FROM network_devices WHERE ($1::bigint IS NULL OR branch_id = $1) AND ($2::text IS NULL OR status = $2) ORDER BY created_at DESC LIMIT $3 OFFSET $4")
            .bind(branch_id).bind(status).bind(per_page).bind(offset).fetch_all(self.pool).await?;
        Ok((devices, count_row.0))
    }

    pub async fn get_device(&self, id: i64) -> Result<Option<NetworkDevice>, sqlx::Error> {
        sqlx::query_as::<_, NetworkDevice>("SELECT * FROM network_devices WHERE id = $1").bind(id).fetch_optional(self.pool).await
    }

    pub async fn create_device(&self, branch_id: i64, name: &str, model_id: i64, serial: &str, ip: &str, port: Option<i32>, firmware: Option<&str>, city: Option<&str>, area: Option<&str>) -> Result<NetworkDevice, sqlx::Error> {
        sqlx::query_as::<_, NetworkDevice>("INSERT INTO network_devices (branch_id, name, device_model_id, serial_number, management_ip, management_port, firmware_version, location_city, location_area) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) RETURNING *")
            .bind(branch_id).bind(name).bind(model_id).bind(serial).bind(ip).bind(port).bind(firmware).bind(city).bind(area).fetch_one(self.pool).await
    }

    pub async fn update_device(&self, id: i64, name: Option<&str>, firmware: Option<&str>, status: Option<&str>, city: Option<&str>, area: Option<&str>) -> Result<NetworkDevice, sqlx::Error> {
        sqlx::query_as::<_, NetworkDevice>("UPDATE network_devices SET name = COALESCE($2, name), firmware_version = COALESCE($3, firmware_version), status = COALESCE($4, status), location_city = COALESCE($5, location_city), location_area = COALESCE($6, location_area), updated_at = NOW() WHERE id = $1 RETURNING *")
            .bind(id).bind(name).bind(firmware).bind(status).bind(city).bind(area).fetch_one(self.pool).await
    }

    pub async fn delete_device(&self, id: i64) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("DELETE FROM network_devices WHERE id = $1").bind(id).execute(self.pool).await?;
        Ok(r.rows_affected() > 0)
    }

    pub async fn list_models(&self) -> Result<Vec<DeviceModel>, sqlx::Error> {
        sqlx::query_as::<_, DeviceModel>("SELECT * FROM device_models ORDER BY vendor, model").fetch_all(self.pool).await
    }

    pub async fn create_model(&self, vendor: &str, model: &str, device_type: &str, protocol: &str, port: Option<i32>) -> Result<DeviceModel, sqlx::Error> {
        sqlx::query_as::<_, DeviceModel>("INSERT INTO device_models (vendor, model, device_type, management_protocol, default_port) VALUES ($1,$2,$3,$4,$5) RETURNING *")
            .bind(vendor).bind(model).bind(device_type).bind(protocol).bind(port).fetch_one(self.pool).await
    }
}
