//! SeaORM-based service for the Installation domain.

use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::modules::installation::repository::installation_repository_seaorm::InstallationRepositorySeaorm;
use crate::modules::installation::request::installation_request::*;
use crate::modules::installation::response::installation_response::*;

pub struct InstallationServiceSeaorm<'a> {
    repo: InstallationRepositorySeaorm<'a>,
}

impl<'a> InstallationServiceSeaorm<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { repo: InstallationRepositorySeaorm::new(db) }
    }

    pub async fn list(&self, branch_id: Option<i64>, status: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<InstallationOrderResponse>, i64), AppError> {
        let (orders, total) = self.repo.list(branch_id, status, page, per_page).await?;
        let responses = orders.into_iter().map(|o| InstallationOrderResponse {
            id: o.id, customer_id: o.customer_id, branch_id: o.branch_id, subscription_id: o.subscription_id,
            assigned_technician_id: o.assigned_technician_id, status: o.status,
            scheduled_date: o.scheduled_date, scheduled_time_slot: o.scheduled_time_slot,
            completed_at: o.completed_at.map(|v| v.into()), installation_type: o.installation_type,
            equipment_issued: o.equipment_issued, fiber_drop_length_meters: o.fiber_drop_length_meters,
            onu_power_dbm: o.onu_power_dbm, notes: o.notes, photos: o.photos,
            created_at: o.created_at.into(), updated_at: o.updated_at.into(),
        }).collect();
        Ok((responses, total))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<InstallationOrderResponse, AppError> {
        let o = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Installation order not found".into()))?;
        Ok(InstallationOrderResponse {
            id: o.id, customer_id: o.customer_id, branch_id: o.branch_id, subscription_id: o.subscription_id,
            assigned_technician_id: o.assigned_technician_id, status: o.status,
            scheduled_date: o.scheduled_date, scheduled_time_slot: o.scheduled_time_slot,
            completed_at: o.completed_at.map(|v| v.into()), installation_type: o.installation_type,
            equipment_issued: o.equipment_issued, fiber_drop_length_meters: o.fiber_drop_length_meters,
            onu_power_dbm: o.onu_power_dbm, notes: o.notes, photos: o.photos,
            created_at: o.created_at.into(), updated_at: o.updated_at.into(),
        })
    }

    pub async fn create(&self, customer_id: i64, branch_id: i64, subscription_id: Option<i64>, installation_type: &str) -> Result<InstallationOrderResponse, AppError> {
        let o = self.repo.create(customer_id, branch_id, subscription_id, installation_type).await?;
        Ok(InstallationOrderResponse {
            id: o.id, customer_id: o.customer_id, branch_id: o.branch_id, subscription_id: o.subscription_id,
            assigned_technician_id: o.assigned_technician_id, status: o.status,
            scheduled_date: o.scheduled_date, scheduled_time_slot: o.scheduled_time_slot,
            completed_at: o.completed_at.map(|v| v.into()), installation_type: o.installation_type,
            equipment_issued: o.equipment_issued, fiber_drop_length_meters: o.fiber_drop_length_meters,
            onu_power_dbm: o.onu_power_dbm, notes: o.notes, photos: o.photos,
            created_at: o.created_at.into(), updated_at: o.updated_at.into(),
        })
    }

    pub async fn schedule(&self, id: i64, req: ScheduleInstallationRequest) -> Result<InstallationOrderResponse, AppError> {
        let o = self.repo.schedule(id, req.scheduled_date, &req.scheduled_time_slot, req.assigned_technician_id).await?;
        Ok(InstallationOrderResponse {
            id: o.id, customer_id: o.customer_id, branch_id: o.branch_id, subscription_id: o.subscription_id,
            assigned_technician_id: o.assigned_technician_id, status: o.status,
            scheduled_date: o.scheduled_date, scheduled_time_slot: o.scheduled_time_slot,
            completed_at: o.completed_at.map(|v| v.into()), installation_type: o.installation_type,
            equipment_issued: o.equipment_issued, fiber_drop_length_meters: o.fiber_drop_length_meters,
            onu_power_dbm: o.onu_power_dbm, notes: o.notes, photos: o.photos,
            created_at: o.created_at.into(), updated_at: o.updated_at.into(),
        })
    }

    pub async fn start(&self, id: i64) -> Result<InstallationOrderResponse, AppError> {
        let o = self.repo.start(id).await?;
        Ok(InstallationOrderResponse {
            id: o.id, customer_id: o.customer_id, branch_id: o.branch_id, subscription_id: o.subscription_id,
            assigned_technician_id: o.assigned_technician_id, status: o.status,
            scheduled_date: o.scheduled_date, scheduled_time_slot: o.scheduled_time_slot,
            completed_at: o.completed_at.map(|v| v.into()), installation_type: o.installation_type,
            equipment_issued: o.equipment_issued, fiber_drop_length_meters: o.fiber_drop_length_meters,
            onu_power_dbm: o.onu_power_dbm, notes: o.notes, photos: o.photos,
            created_at: o.created_at.into(), updated_at: o.updated_at.into(),
        })
    }

    pub async fn complete(&self, id: i64, req: CompleteInstallationRequest) -> Result<InstallationOrderResponse, AppError> {
        let o = self.repo.complete(id, req.fiber_drop_length_meters, req.onu_power_dbm, req.equipment_issued, req.notes.as_deref()).await?;
        Ok(InstallationOrderResponse {
            id: o.id, customer_id: o.customer_id, branch_id: o.branch_id, subscription_id: o.subscription_id,
            assigned_technician_id: o.assigned_technician_id, status: o.status,
            scheduled_date: o.scheduled_date, scheduled_time_slot: o.scheduled_time_slot,
            completed_at: o.completed_at.map(|v| v.into()), installation_type: o.installation_type,
            equipment_issued: o.equipment_issued, fiber_drop_length_meters: o.fiber_drop_length_meters,
            onu_power_dbm: o.onu_power_dbm, notes: o.notes, photos: o.photos,
            created_at: o.created_at.into(), updated_at: o.updated_at.into(),
        })
    }

    pub async fn cancel(&self, id: i64) -> Result<InstallationOrderResponse, AppError> {
        let o = self.repo.cancel(id).await?;
        Ok(InstallationOrderResponse {
            id: o.id, customer_id: o.customer_id, branch_id: o.branch_id, subscription_id: o.subscription_id,
            assigned_technician_id: o.assigned_technician_id, status: o.status,
            scheduled_date: o.scheduled_date, scheduled_time_slot: o.scheduled_time_slot,
            completed_at: o.completed_at.map(|v| v.into()), installation_type: o.installation_type,
            equipment_issued: o.equipment_issued, fiber_drop_length_meters: o.fiber_drop_length_meters,
            onu_power_dbm: o.onu_power_dbm, notes: o.notes, photos: o.photos,
            created_at: o.created_at.into(), updated_at: o.updated_at.into(),
        })
    }

    pub async fn get_my_assignments(&self, technician_id: i64) -> Result<Vec<InstallationOrderResponse>, AppError> {
        let orders = self.repo.get_my_assignments(technician_id).await?;
        Ok(orders.into_iter().map(|o| InstallationOrderResponse {
            id: o.id, customer_id: o.customer_id, branch_id: o.branch_id, subscription_id: o.subscription_id,
            assigned_technician_id: o.assigned_technician_id, status: o.status,
            scheduled_date: o.scheduled_date, scheduled_time_slot: o.scheduled_time_slot,
            completed_at: o.completed_at.map(|v| v.into()), installation_type: o.installation_type,
            equipment_issued: o.equipment_issued, fiber_drop_length_meters: o.fiber_drop_length_meters,
            onu_power_dbm: o.onu_power_dbm, notes: o.notes, photos: o.photos,
            created_at: o.created_at.into(), updated_at: o.updated_at.into(),
        }).collect())
    }
}
