//! SeaORM-based repository for the Device domain.
//! Zero plain SQL — all queries use EntityTrait, ActiveModelTrait, and Select.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};

use crate::common::errors::app_error::AppError;
use crate::modules::device::model::device_model_entity::{self, Model as DeviceModelModel};
use crate::modules::device::model::network_device_entity::{self, Model as NetworkDeviceModel};
use crate::modules::device::model::device_port_entity::{self, Model as DevicePortModel};
use crate::modules::device::model::firmware_update_entity::{self, Model as FirmwareUpdateModel};
use crate::modules::device::model::device_log_entity::{self, Model as DeviceLogModel};
use crate::modules::device::model::device_metric_entity;

pub struct DeviceRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> DeviceRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    // ── Network Devices ─────────────────────────────────────

    pub async fn list_devices(&self, branch_id: Option<i64>, status: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<NetworkDeviceModel>, i64), AppError> {
        let page_size = (per_page.max(1)) as u64;
        let page_num = ((page.max(1) - 1) as u64);
        let mut select = network_device_entity::Entity::find();
        if let Some(bid) = branch_id {
            select = select.filter(network_device_entity::Column::BranchId.eq(bid));
        }
        if let Some(s) = status {
            select = select.filter(network_device_entity::Column::Status.eq(s));
        }
        let total = select.clone().count(self.db).await?;
        let models = select
            .order_by_desc(network_device_entity::Column::CreatedAt)
            .paginate(self.db, page_size)
            .fetch_page(page_num).await?;
        Ok((models, total as i64))
    }

    pub async fn get_device(&self, id: i64) -> Result<Option<NetworkDeviceModel>, AppError> {
        Ok(network_device_entity::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn create_device(&self, branch_id: i64, name: &str, model_id: i64, serial: &str, ip: &str, port: Option<i32>, firmware: Option<&str>, city: Option<&str>, area: Option<&str>) -> Result<NetworkDeviceModel, AppError> {
        let now = chrono::Utc::now();
        let active = network_device_entity::ActiveModel {
            branch_id: Set(branch_id),
            name: Set(name.to_owned()),
            device_model_id: Set(model_id),
            serial_number: Set(serial.to_owned()),
            management_ip: Set(ip.to_owned()),
            management_port: Set(port),
            firmware_version: Set(firmware.map(|s| s.to_owned())),
            status: Set("active".to_owned()),
            health_score: Set(None),
            location_city: Set(city.map(|s| s.to_owned())),
            location_area: Set(area.map(|s| s.to_owned())),
            created_by: Set(None),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn update_device(&self, id: i64, name: Option<&str>, firmware: Option<&str>, status: Option<&str>, city: Option<&str>, area: Option<&str>) -> Result<NetworkDeviceModel, AppError> {
        let existing = network_device_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Device not found".into()))?;
        let mut active = existing.into_active_model();
        if let Some(v) = name { active.name = Set(v.to_owned()); }
        if let Some(v) = firmware { active.firmware_version = Set(Some(v.to_owned())); }
        if let Some(v) = status { active.status = Set(v.to_owned()); }
        if let Some(v) = city { active.location_city = Set(Some(v.to_owned())); }
        if let Some(v) = area { active.location_area = Set(Some(v.to_owned())); }
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn delete_device(&self, id: i64) -> Result<bool, AppError> {
        let existing = network_device_entity::Entity::find_by_id(id).one(self.db).await?;
        match existing {
            Some(e) => { e.into_active_model().delete(self.db).await?; Ok(true) }
            None => Ok(false),
        }
    }

    // ── Device Models ───────────────────────────────────────

    pub async fn list_models(&self) -> Result<Vec<DeviceModelModel>, AppError> {
        Ok(device_model_entity::Entity::find()
            .order_by_asc(device_model_entity::Column::Vendor)
            .order_by_asc(device_model_entity::Column::Model)
            .all(self.db).await?)
    }

    pub async fn create_model(&self, vendor: &str, model: &str, device_type: &str, protocol: &str, port: Option<i32>) -> Result<DeviceModelModel, AppError> {
        let now = chrono::Utc::now();
        let active = device_model_entity::ActiveModel {
            vendor: Set(vendor.to_owned()),
            model: Set(model.to_owned()),
            device_type: Set(device_type.to_owned()),
            management_protocol: Set(protocol.to_owned()),
            default_port: Set(port),
            created_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    // ── Ports ──────────────────────────────────────────────

    pub async fn list_ports(&self, device_id: i64) -> Result<Vec<DevicePortModel>, AppError> {
        Ok(device_port_entity::Entity::find()
            .filter(device_port_entity::Column::DeviceId.eq(device_id))
            .order_by_asc(device_port_entity::Column::PortNumber)
            .all(self.db).await?)
    }

    pub async fn update_port_status(&self, device_id: i64, port_id: i64, status: &str) -> Result<DevicePortModel, AppError> {
        let existing = device_port_entity::Entity::find_by_id(port_id)
            .filter(device_port_entity::Column::DeviceId.eq(device_id))
            .one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Port not found".into()))?;
        let mut active = existing.into_active_model();
        active.status = Set(status.to_owned());
        Ok(active.update(self.db).await?)
    }

    // ── Device Control ─────────────────────────────────────

    pub async fn restart_device(&self, id: i64) -> Result<bool, AppError> {
        let existing = network_device_entity::Entity::find_by_id(id).one(self.db).await?;
        match existing {
            Some(e) => {
                let mut active = e.into_active_model();
                active.status = Set("maintenance".to_owned());
                active.updated_at = Set(chrono::Utc::now().into());
                active.update(self.db).await?;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    pub async fn shutdown_device(&self, id: i64) -> Result<bool, AppError> {
        let existing = network_device_entity::Entity::find_by_id(id).one(self.db).await?;
        match existing {
            Some(e) => {
                let mut active = e.into_active_model();
                active.status = Set("offline".to_owned());
                active.updated_at = Set(chrono::Utc::now().into());
                active.update(self.db).await?;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    // ── Firmware ───────────────────────────────────────────

    pub async fn list_firmware_updates(&self, device_id: i64) -> Result<Vec<FirmwareUpdateModel>, AppError> {
        Ok(firmware_update_entity::Entity::find()
            .filter(firmware_update_entity::Column::DeviceId.eq(device_id))
            .order_by_desc(firmware_update_entity::Column::CreatedAt)
            .all(self.db).await?)
    }

    pub async fn get_firmware_version(&self, device_id: i64) -> Result<Option<String>, AppError> {
        let device = network_device_entity::Entity::find_by_id(device_id).one(self.db).await?;
        Ok(device.and_then(|d| d.firmware_version))
    }

    pub async fn create_firmware_update(&self, device_id: i64, from_version: Option<&str>, to_version: &str, initiated_by: Option<i64>) -> Result<FirmwareUpdateModel, AppError> {
        let now = chrono::Utc::now();
        let active = firmware_update_entity::ActiveModel {
            device_id: Set(device_id),
            from_version: Set(from_version.map(|s| s.to_owned())),
            to_version: Set(to_version.to_owned()),
            status: Set("pending".to_owned()),
            initiated_by: Set(initiated_by),
            started_at: Set(None),
            completed_at: Set(None),
            failure_reason: Set(None),
            created_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn update_firmware_status(&self, id: i64, status: &str, failure_reason: Option<&str>) -> Result<FirmwareUpdateModel, AppError> {
        let existing = firmware_update_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Firmware update not found".into()))?;
        let mut active = existing.into_active_model();
        active.status = Set(status.to_owned());
        active.failure_reason = Set(failure_reason.map(|s| s.to_owned()));
        if status == "completed" || status == "failed" {
            active.completed_at = Set(Some(chrono::Utc::now().into()));
        }
        Ok(active.update(self.db).await?)
    }

    // ── Metrics ────────────────────────────────────────────

    pub async fn get_device_metrics(&self, device_id: i64, limit: i64) -> Result<Vec<device_metric_entity::Model>, AppError> {
        Ok(device_metric_entity::Entity::find()
            .filter(device_metric_entity::Column::DeviceId.eq(device_id))
            .order_by_desc(device_metric_entity::Column::RecordedAt)
            .paginate(self.db, limit.max(1) as u64)
            .fetch_page(0).await?)
    }

    pub async fn insert_metric(&self, device_id: i64, name: &str, value: f64, unit: Option<&str>) -> Result<(), AppError> {
        let now = chrono::Utc::now();
        let active = device_metric_entity::ActiveModel {
            device_id: Set(device_id),
            metric_name: Set(name.to_owned()),
            metric_value: Set(value),
            unit: Set(unit.map(|s| s.to_owned())),
            recorded_at: Set(now.into()),
            ..Default::default()
        };
        active.insert(self.db).await?;
        Ok(())
    }

    // ── Device Logs ────────────────────────────────────────

    pub async fn list_logs(&self, device_id: i64, level: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<DeviceLogModel>, i64), AppError> {
        let page_size = (per_page.max(1)) as u64;
        let page_num = ((page.max(1) - 1) as u64);
        let mut select = device_log_entity::Entity::find()
            .filter(device_log_entity::Column::DeviceId.eq(device_id));
        if let Some(l) = level {
            select = select.filter(device_log_entity::Column::Level.eq(l));
        }
        let total = select.clone().count(self.db).await?;
        let models = select
            .order_by_desc(device_log_entity::Column::CreatedAt)
            .paginate(self.db, page_size)
            .fetch_page(page_num).await?;
        Ok((models, total as i64))
    }

    pub async fn insert_log(&self, device_id: i64, level: &str, message: &str, source: Option<&str>, metadata: Option<serde_json::Value>) -> Result<DeviceLogModel, AppError> {
        let now = chrono::Utc::now();
        let active = device_log_entity::ActiveModel {
            device_id: Set(device_id),
            level: Set(level.to_owned()),
            message: Set(message.to_owned()),
            source: Set(source.map(|s| s.to_owned())),
            metadata: Set(metadata),
            created_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }
}
