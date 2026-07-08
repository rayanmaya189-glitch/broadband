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
}
