use sqlx::PgPool;
use crate::common::errors::app_error::AppError;
use crate::modules::device::repository::device_repository::DeviceRepository;
use crate::modules::device::request::device_request::*;
use crate::modules::device::response::device_response::*;

pub struct DeviceService<'a> { repo: DeviceRepository<'a> }
impl<'a> DeviceService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: DeviceRepository::new(pool) } }

    pub async fn list_devices(&self, query: DeviceQuery) -> Result<DeviceListResponse, AppError> {
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(20);
        let (devices, total) = self.repo.list_devices(query.branch_id, query.status.as_deref(), page, per_page).await?;
        let responses: Vec<DeviceResponse> = devices.iter().map(|d| DeviceResponse { id: d.id, branch_id: d.branch_id, name: d.name.clone(), device_model_id: d.device_model_id, serial_number: d.serial_number.clone(), management_ip: d.management_ip.clone(), management_port: d.management_port, firmware_version: d.firmware_version.clone(), status: d.status.clone(), health_score: d.health_score, location_city: d.location_city.clone(), location_area: d.location_area.clone(), created_at: d.created_at }).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok(DeviceListResponse { devices: responses, total, page, per_page, total_pages })
    }

    pub async fn get_device(&self, id: i64) -> Result<DeviceResponse, AppError> {
        let d = self.repo.get_device(id).await?.ok_or_else(|| AppError::NotFound("Device not found".into()))?;
        Ok(DeviceResponse { id: d.id, branch_id: d.branch_id, name: d.name, device_model_id: d.device_model_id, serial_number: d.serial_number, management_ip: d.management_ip, management_port: d.management_port, firmware_version: d.firmware_version, status: d.status, health_score: d.health_score, location_city: d.location_city, location_area: d.location_area, created_at: d.created_at })
    }

    pub async fn create_device(&self, req: CreateDeviceRequest) -> Result<DeviceResponse, AppError> {
        let d = self.repo.create_device(req.branch_id, &req.name, req.device_model_id, &req.serial_number, &req.management_ip, req.management_port, req.firmware_version.as_deref(), req.location_city.as_deref(), req.location_area.as_deref()).await?;
        Ok(DeviceResponse { id: d.id, branch_id: d.branch_id, name: d.name, device_model_id: d.device_model_id, serial_number: d.serial_number, management_ip: d.management_ip, management_port: d.management_port, firmware_version: d.firmware_version, status: d.status, health_score: d.health_score, location_city: d.location_city, location_area: d.location_area, created_at: d.created_at })
    }

    pub async fn update_device(&self, id: i64, req: UpdateDeviceRequest) -> Result<DeviceResponse, AppError> {
        let d = self.repo.update_device(id, req.name.as_deref(), req.firmware_version.as_deref(), req.status.as_deref(), req.location_city.as_deref(), req.location_area.as_deref()).await.map_err(|_| AppError::NotFound("Device not found".into()))?;
        Ok(DeviceResponse { id: d.id, branch_id: d.branch_id, name: d.name, device_model_id: d.device_model_id, serial_number: d.serial_number, management_ip: d.management_ip, management_port: d.management_port, firmware_version: d.firmware_version, status: d.status, health_score: d.health_score, location_city: d.location_city, location_area: d.location_area, created_at: d.created_at })
    }

    pub async fn delete_device(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.delete_device(id).await? { return Err(AppError::NotFound("Device not found".into())); }
        Ok(MessageResponse { message: "Device deleted".into() })
    }

    pub async fn list_models(&self) -> Result<Vec<DeviceModelResponse>, AppError> {
        let models = self.repo.list_models().await?;
        Ok(models.iter().map(|m| DeviceModelResponse { id: m.id, vendor: m.vendor.clone(), model: m.model.clone(), device_type: m.device_type.clone(), management_protocol: m.management_protocol.clone(), default_port: m.default_port, created_at: m.created_at }).collect())
    }

    pub async fn create_model(&self, req: CreateDeviceModelRequest) -> Result<DeviceModelResponse, AppError> {
        let m = self.repo.create_model(&req.vendor, &req.model, &req.device_type, &req.management_protocol, req.default_port).await?;
        Ok(DeviceModelResponse { id: m.id, vendor: m.vendor, model: m.model, device_type: m.device_type, management_protocol: m.management_protocol, default_port: m.default_port, created_at: m.created_at })
    }

    // ── Ports ──────────────────────────────────────────────

    pub async fn list_ports(&self, device_id: i64) -> Result<Vec<DevicePortResponse>, AppError> {
        let _ = self.repo.get_device(device_id).await?.ok_or_else(|| AppError::NotFound("Device not found".into()))?;
        let ports = self.repo.list_ports(device_id).await?;
        Ok(ports.iter().map(|p| DevicePortResponse { id: p.id, device_id: p.device_id, port_number: p.port_number, port_name: p.port_name.clone(), port_type: p.port_type.clone(), speed_mbps: p.speed_mbps, status: p.status.clone(), connected_device_id: p.connected_device_id, customer_id: p.customer_id, created_at: p.created_at }).collect())
    }

    pub async fn update_port_status(&self, device_id: i64, port_id: i64, req: PortStatusRequest) -> Result<DevicePortResponse, AppError> {
        let p = self.repo.update_port_status(device_id, port_id, &req.status).await.map_err(|_| AppError::NotFound("Port not found".into()))?;
        Ok(DevicePortResponse { id: p.id, device_id: p.device_id, port_number: p.port_number, port_name: p.port_name, port_type: p.port_type, speed_mbps: p.speed_mbps, status: p.status, connected_device_id: p.connected_device_id, customer_id: p.customer_id, created_at: p.created_at })
    }

    // ── Device Control ─────────────────────────────────────

    pub async fn restart_device(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.restart_device(id).await? { return Err(AppError::NotFound("Device not found".into())); }
        Ok(MessageResponse { message: "Device restart initiated".into() })
    }

    pub async fn shutdown_device(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.shutdown_device(id).await? { return Err(AppError::NotFound("Device not found".into())); }
        Ok(MessageResponse { message: "Device shutdown initiated".into() })
    }

    // ── Firmware ───────────────────────────────────────────

    pub async fn list_firmware_updates(&self, device_id: i64) -> Result<Vec<FirmwareUpdateResponse>, AppError> {
        let _ = self.repo.get_device(device_id).await?.ok_or_else(|| AppError::NotFound("Device not found".into()))?;
        let updates = self.repo.list_firmware_updates(device_id).await?;
        Ok(updates.iter().map(|u| FirmwareUpdateResponse { id: u.id, device_id: u.device_id, from_version: u.from_version.clone(), to_version: u.to_version.clone(), status: u.status.clone(), started_at: u.started_at, completed_at: u.completed_at, failure_reason: u.failure_reason.clone(), created_at: u.created_at }).collect())
    }

    pub async fn create_firmware_update(&self, device_id: i64, req: FirmwareUpdateRequest) -> Result<FirmwareUpdateResponse, AppError> {
        let _ = self.repo.get_device(device_id).await?.ok_or_else(|| AppError::NotFound("Device not found".into()))?;
        let u = self.repo.create_firmware_update(device_id, &req.to_version, None).await?;
        Ok(FirmwareUpdateResponse { id: u.id, device_id: u.device_id, from_version: u.from_version, to_version: u.to_version, status: u.status, started_at: u.started_at, completed_at: u.completed_at, failure_reason: u.failure_reason, created_at: u.created_at })
    }

    pub async fn update_firmware_status(&self, update_id: i64, req: FirmwareStatusRequest) -> Result<FirmwareUpdateResponse, AppError> {
        let u = self.repo.update_firmware_status(update_id, &req.status, req.failure_reason.as_deref()).await.map_err(|_| AppError::NotFound("Firmware update not found".into()))?;
        Ok(FirmwareUpdateResponse { id: u.id, device_id: u.device_id, from_version: u.from_version, to_version: u.to_version, status: u.status, started_at: u.started_at, completed_at: u.completed_at, failure_reason: u.failure_reason, created_at: u.created_at })
    }

    // ── Metrics ────────────────────────────────────────────

    pub async fn get_device_metrics(&self, device_id: i64) -> Result<Vec<DeviceMetricResponse>, AppError> {
        let _ = self.repo.get_device(device_id).await?.ok_or_else(|| AppError::NotFound("Device not found".into()))?;
        let metrics: Vec<(String, f64, Option<String>, chrono::DateTime<chrono::Utc>)> = self.repo.get_device_metrics(device_id, 100).await?;
        Ok(metrics.into_iter().map(|(name, value, unit, recorded_at)| DeviceMetricResponse { metric_name: name, metric_value: value, unit, recorded_at }).collect())
    }

    // ── Device Logs ────────────────────────────────────────

    pub async fn list_logs(&self, device_id: i64, query: DeviceLogQuery) -> Result<DeviceLogListResponse, AppError> {
        let _ = self.repo.get_device(device_id).await?.ok_or_else(|| AppError::NotFound("Device not found".into()))?;
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(50);
        let (logs, total) = self.repo.list_logs(device_id, query.level.as_deref(), page, per_page).await?;
        let responses: Vec<DeviceLogResponse> = logs.iter().map(|l| DeviceLogResponse { id: l.id, device_id: l.device_id, level: l.level.clone(), message: l.message.clone(), source: l.source.clone(), metadata: l.metadata.clone(), created_at: l.created_at }).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok(DeviceLogListResponse { logs: responses, total, page, per_page, total_pages })
    }

    pub async fn insert_log(&self, device_id: i64, level: &str, message: &str, source: Option<&str>, metadata: Option<serde_json::Value>) -> Result<DeviceLogResponse, AppError> {
        let _ = self.repo.get_device(device_id).await?.ok_or_else(|| AppError::NotFound("Device not found".into()))?;
        let l = self.repo.insert_log(device_id, level, message, source, metadata).await?;
        Ok(DeviceLogResponse { id: l.id, device_id: l.device_id, level: l.level, message: l.message, source: l.source, metadata: l.metadata, created_at: l.created_at })
    }
}
