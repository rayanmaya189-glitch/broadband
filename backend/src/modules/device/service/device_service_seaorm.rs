//! SeaORM-based service for the Device domain.

use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::modules::device::repository::device_repository_seaorm::DeviceRepositorySeaorm;
use crate::modules::device::request::device_request::*;
use crate::modules::device::response::device_response::*;

pub struct DeviceServiceSeaorm<'a> {
    repo: DeviceRepositorySeaorm<'a>,
}

impl<'a> DeviceServiceSeaorm<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { repo: DeviceRepositorySeaorm::new(db) }
    }

    pub async fn list_devices(&self, query: DeviceQuery) -> Result<DeviceListResponse, AppError> {
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(20);
        let (devices, total) = self.repo.list_devices(query.branch_id, query.status.as_deref(), page, per_page).await?;
        let responses: Vec<DeviceResponse> = devices.into_iter().map(DeviceResponse::from_model).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok(DeviceListResponse { devices: responses, total, page, per_page, total_pages })
    }

    pub async fn get_device(&self, id: i64) -> Result<DeviceResponse, AppError> {
        let d = self.repo.get_device(id).await?.ok_or_else(|| AppError::NotFound("Device not found".into()))?;
        Ok(DeviceResponse::from_model(d))
    }

    pub async fn create_device(&self, req: CreateDeviceRequest) -> Result<DeviceResponse, AppError> {
        let d = self.repo.create_device(req.branch_id, &req.name, req.device_model_id, &req.serial_number, &req.management_ip, req.management_port, req.firmware_version.as_deref(), req.location_city.as_deref(), req.location_area.as_deref()).await?;
        Ok(DeviceResponse::from_model(d))
    }

    pub async fn update_device(&self, id: i64, req: UpdateDeviceRequest) -> Result<DeviceResponse, AppError> {
        let d = self.repo.update_device(id, req.name.as_deref(), req.firmware_version.as_deref(), req.status.as_deref(), req.location_city.as_deref(), req.location_area.as_deref()).await?;
        Ok(DeviceResponse::from_model(d))
    }

    pub async fn delete_device(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.delete_device(id).await? {
            return Err(AppError::NotFound("Device not found".into()));
        }
        Ok(MessageResponse { message: "Device deleted".into() })
    }

    pub async fn list_models(&self) -> Result<Vec<DeviceModelResponse>, AppError> {
        let models = self.repo.list_models().await?;
        Ok(models.into_iter().map(DeviceModelResponse::from_model).collect())
    }

    pub async fn create_model(&self, req: CreateDeviceModelRequest) -> Result<DeviceModelResponse, AppError> {
        let m = self.repo.create_model(&req.vendor, &req.model, &req.device_type, &req.management_protocol, req.default_port).await?;
        Ok(DeviceModelResponse::from_model(m))
    }

    pub async fn list_ports(&self, device_id: i64) -> Result<Vec<DevicePortResponse>, AppError> {
        self.repo.get_device(device_id).await?.ok_or_else(|| AppError::NotFound("Device not found".into()))?;
        let ports = self.repo.list_ports(device_id).await?;
        Ok(ports.into_iter().map(DevicePortResponse::from_model).collect())
    }

    pub async fn update_port_status(&self, device_id: i64, port_id: i64, req: PortStatusRequest) -> Result<DevicePortResponse, AppError> {
        let p = self.repo.update_port_status(device_id, port_id, &req.status).await?;
        Ok(DevicePortResponse::from_model(p))
    }

    pub async fn restart_device(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.restart_device(id).await? {
            return Err(AppError::NotFound("Device not found".into()));
        }
        Ok(MessageResponse { message: "Device restart initiated".into() })
    }

    pub async fn shutdown_device(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.shutdown_device(id).await? {
            return Err(AppError::NotFound("Device not found".into()));
        }
        Ok(MessageResponse { message: "Device shutdown initiated".into() })
    }

    pub async fn list_firmware_updates(&self, device_id: i64) -> Result<Vec<FirmwareUpdateResponse>, AppError> {
        self.repo.get_device(device_id).await?.ok_or_else(|| AppError::NotFound("Device not found".into()))?;
        let updates = self.repo.list_firmware_updates(device_id).await?;
        Ok(updates.into_iter().map(FirmwareUpdateResponse::from_model).collect())
    }

    pub async fn create_firmware_update(&self, device_id: i64, req: FirmwareUpdateRequest) -> Result<FirmwareUpdateResponse, AppError> {
        self.repo.get_device(device_id).await?.ok_or_else(|| AppError::NotFound("Device not found".into()))?;
        let from_version = self.repo.get_firmware_version(device_id).await?;
        let u = self.repo.create_firmware_update(device_id, from_version.as_deref(), &req.to_version, None).await?;
        Ok(FirmwareUpdateResponse::from_model(u))
    }

    pub async fn update_firmware_status(&self, update_id: i64, req: FirmwareStatusRequest) -> Result<FirmwareUpdateResponse, AppError> {
        let u = self.repo.update_firmware_status(update_id, &req.status, req.failure_reason.as_deref()).await?;
        Ok(FirmwareUpdateResponse::from_model(u))
    }

    pub async fn get_device_metrics(&self, device_id: i64) -> Result<Vec<DeviceMetricResponse>, AppError> {
        self.repo.get_device(device_id).await?.ok_or_else(|| AppError::NotFound("Device not found".into()))?;
        let metrics = self.repo.get_device_metrics(device_id, 100).await?;
        Ok(metrics.into_iter().map(|m| DeviceMetricResponse {
            metric_name: m.metric_name, metric_value: m.metric_value,
            unit: m.unit, recorded_at: m.recorded_at.into(),
        }).collect())
    }

    pub async fn list_logs(&self, device_id: i64, query: DeviceLogQuery) -> Result<DeviceLogListResponse, AppError> {
        self.repo.get_device(device_id).await?.ok_or_else(|| AppError::NotFound("Device not found".into()))?;
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(50);
        let (logs, total) = self.repo.list_logs(device_id, query.level.as_deref(), page, per_page).await?;
        let responses: Vec<DeviceLogResponse> = logs.into_iter().map(DeviceLogResponse::from_model).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok(DeviceLogListResponse { logs: responses, total, page, per_page, total_pages })
    }

    pub async fn insert_log(&self, device_id: i64, level: &str, message: &str, source: Option<&str>, metadata: Option<serde_json::Value>) -> Result<DeviceLogResponse, AppError> {
        self.repo.get_device(device_id).await?.ok_or_else(|| AppError::NotFound("Device not found".into()))?;
        let l = self.repo.insert_log(device_id, level, message, source, metadata).await?;
        Ok(DeviceLogResponse::from_model(l))
    }
}
