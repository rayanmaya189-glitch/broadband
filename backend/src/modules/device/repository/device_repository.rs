use chrono::{DateTime, Utc};
use sqlx::PgPool;
use crate::modules::device::model::device::{DeviceModel, NetworkDevice, DevicePort, FirmwareUpdate, DeviceLog};

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

    // ── Ports ──────────────────────────────────────────────

    pub async fn list_ports(&self, device_id: i64) -> Result<Vec<DevicePort>, sqlx::Error> {
        sqlx::query_as::<_, DevicePort>("SELECT * FROM device_ports WHERE device_id = $1 ORDER BY port_number")
            .bind(device_id).fetch_all(self.pool).await
    }

    pub async fn update_port_status(&self, device_id: i64, port_id: i64, status: &str) -> Result<DevicePort, sqlx::Error> {
        sqlx::query_as::<_, DevicePort>("UPDATE device_ports SET status = $3 WHERE id = $2 AND device_id = $1 RETURNING *")
            .bind(device_id).bind(port_id).bind(status).fetch_one(self.pool).await
    }

    // ── Device Control ─────────────────────────────────────

    pub async fn restart_device(&self, id: i64) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("UPDATE network_devices SET status = 'maintenance', updated_at = NOW() WHERE id = $1")
            .bind(id).execute(self.pool).await?;
        Ok(r.rows_affected() > 0)
    }

    pub async fn shutdown_device(&self, id: i64) -> Result<bool, sqlx::Error> {
        let r = sqlx::query("UPDATE network_devices SET status = 'offline', updated_at = NOW() WHERE id = $1")
            .bind(id).execute(self.pool).await?;
        Ok(r.rows_affected() > 0)
    }

    // ── Firmware ───────────────────────────────────────────

    pub async fn list_firmware_updates(&self, device_id: i64) -> Result<Vec<FirmwareUpdate>, sqlx::Error> {
        sqlx::query_as::<_, FirmwareUpdate>("SELECT * FROM firmware_updates WHERE device_id = $1 ORDER BY created_at DESC")
            .bind(device_id).fetch_all(self.pool).await
    }

    pub async fn create_firmware_update(&self, device_id: i64, to_version: &str, initiated_by: Option<i64>) -> Result<FirmwareUpdate, sqlx::Error> {
        // Get current firmware version
        let device: Option<(Option<String>,)> = sqlx::query_as("SELECT firmware_version FROM network_devices WHERE id = $1")
            .bind(device_id).fetch_optional(self.pool).await?;
        let from_version = device.and_then(|d| d.0);

        let r = sqlx::query_as::<_, FirmwareUpdate>(
            "INSERT INTO firmware_updates (device_id, from_version, to_version, initiated_by) VALUES ($1,$2,$3,$4) RETURNING *"
        ).bind(device_id).bind(&from_version).bind(to_version).bind(initiated_by).fetch_one(self.pool).await?;
        Ok(r)
    }

    pub async fn update_firmware_status(&self, id: i64, status: &str, failure_reason: Option<&str>) -> Result<FirmwareUpdate, sqlx::Error> {
        sqlx::query_as::<_, FirmwareUpdate>(
            "UPDATE firmware_updates SET status = $2, failure_reason = $3, completed_at = CASE WHEN $2 IN ('completed','failed') THEN NOW() ELSE completed_at END WHERE id = $1 RETURNING *"
        ).bind(id).bind(status).bind(failure_reason).fetch_one(self.pool).await
    }

    // ── Metrics ────────────────────────────────────────────

    pub async fn get_device_metrics(&self, device_id: i64, limit: i64) -> Result<Vec<(String, f64, Option<String>, DateTime<Utc>)>, sqlx::Error> {
        sqlx::query_as("SELECT metric_name, metric_value, unit, recorded_at FROM device_metrics WHERE device_id = $1 ORDER BY recorded_at DESC LIMIT $2")
            .bind(device_id).bind(limit).fetch_all(self.pool).await
    }

    pub async fn insert_metric(&self, device_id: i64, name: &str, value: f64, unit: Option<&str>) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO device_metrics (device_id, metric_name, metric_value, unit) VALUES ($1,$2,$3,$4)")
            .bind(device_id).bind(name).bind(value).bind(unit).execute(self.pool).await?;
        Ok(())
    }

    // ── Device Logs ────────────────────────────────────────

    pub async fn list_logs(&self, device_id: i64, level: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<DeviceLog>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM device_logs WHERE device_id = $1 AND ($2::text IS NULL OR level = $2)")
            .bind(device_id).bind(level).fetch_one(self.pool).await?;
        let logs: Vec<DeviceLog> = sqlx::query_as("SELECT * FROM device_logs WHERE device_id = $1 AND ($2::text IS NULL OR level = $2) ORDER BY created_at DESC LIMIT $3 OFFSET $4")
            .bind(device_id).bind(level).bind(per_page).bind(offset).fetch_all(self.pool).await?;
        Ok((logs, count_row.0))
    }

    pub async fn insert_log(&self, device_id: i64, level: &str, message: &str, source: Option<&str>, metadata: Option<serde_json::Value>) -> Result<DeviceLog, sqlx::Error> {
        sqlx::query_as::<_, DeviceLog>("INSERT INTO device_logs (device_id, level, message, source, metadata) VALUES ($1,$2,$3,$4,$5) RETURNING *")
            .bind(device_id).bind(level).bind(message).bind(source).bind(metadata).fetch_one(self.pool).await
    }
}
