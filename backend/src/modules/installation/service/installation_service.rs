use sqlx::PgPool;
use crate::common::errors::app_error::AppError;
use crate::modules::installation::repository::installation_repository::InstallationRepository;
use crate::modules::installation::request::installation_request::*;
use crate::modules::installation::response::installation_response::*;

pub struct InstallationService<'a> { repo: InstallationRepository<'a> }
impl<'a> InstallationService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: InstallationRepository::new(pool) } }

    pub async fn list_installations(&self, query: InstallationQuery) -> Result<InstallationListResponse, AppError> {
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(20);
        let (orders, total) = self.repo.list(query.branch_id, query.status.as_deref(), page, per_page).await?;
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        let responses: Vec<InstallationResponse> = orders.into_iter().map(|o| InstallationResponse {
            id: o.id, customer_id: o.customer_id, branch_id: o.branch_id, subscription_id: o.subscription_id,
            assigned_technician_id: o.assigned_technician_id, status: o.status, scheduled_date: o.scheduled_date,
            scheduled_time_slot: o.scheduled_time_slot, completed_at: o.completed_at, installation_type: o.installation_type,
            notes: o.notes, created_at: o.created_at, customer_name: None, technician_name: None,
        }).collect();
        Ok(InstallationListResponse { installations: responses, total, page, per_page, total_pages })
    }

    pub async fn get_installation(&self, id: i64) -> Result<InstallationResponse, AppError> {
        let o = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Installation not found".into()))?;
        Ok(InstallationResponse { id: o.id, customer_id: o.customer_id, branch_id: o.branch_id, subscription_id: o.subscription_id, assigned_technician_id: o.assigned_technician_id, status: o.status, scheduled_date: o.scheduled_date, scheduled_time_slot: o.scheduled_time_slot, completed_at: o.completed_at, installation_type: o.installation_type, notes: o.notes, created_at: o.created_at, customer_name: None, technician_name: None })
    }

    pub async fn create_installation(&self, req: CreateInstallationRequest) -> Result<InstallationResponse, AppError> {
        let o = self.repo.create(req.customer_id, req.branch_id, req.subscription_id, &req.installation_type.unwrap_or_else(|| "new".into())).await?;
        Ok(InstallationResponse { id: o.id, customer_id: o.customer_id, branch_id: o.branch_id, subscription_id: o.subscription_id, assigned_technician_id: o.assigned_technician_id, status: o.status, scheduled_date: o.scheduled_date, scheduled_time_slot: o.scheduled_time_slot, completed_at: o.completed_at, installation_type: o.installation_type, notes: o.notes, created_at: o.created_at, customer_name: None, technician_name: None })
    }

    pub async fn schedule_installation(&self, id: i64, req: ScheduleInstallationRequest) -> Result<InstallationResponse, AppError> {
        let o = self.repo.schedule(id, req.scheduled_date, &req.scheduled_time_slot, req.technician_id).await.map_err(|_| AppError::NotFound("Installation not found".into()))?;
        Ok(InstallationResponse { id: o.id, customer_id: o.customer_id, branch_id: o.branch_id, subscription_id: o.subscription_id, assigned_technician_id: o.assigned_technician_id, status: o.status, scheduled_date: o.scheduled_date, scheduled_time_slot: o.scheduled_time_slot, completed_at: o.completed_at, installation_type: o.installation_type, notes: o.notes, created_at: o.created_at, customer_name: None, technician_name: None })
    }

    pub async fn reschedule_installation(&self, id: i64, req: RescheduleInstallationRequest) -> Result<InstallationResponse, AppError> {
        let o = self.repo.reschedule(id, req.scheduled_date, &req.scheduled_time_slot, req.reason.as_deref()).await.map_err(|_| AppError::NotFound("Installation not found".into()))?;
        Ok(InstallationResponse { id: o.id, customer_id: o.customer_id, branch_id: o.branch_id, subscription_id: o.subscription_id, assigned_technician_id: o.assigned_technician_id, status: o.status, scheduled_date: o.scheduled_date, scheduled_time_slot: o.scheduled_time_slot, completed_at: o.completed_at, installation_type: o.installation_type, notes: o.notes, created_at: o.created_at, customer_name: None, technician_name: None })
    }

    pub async fn start_installation(&self, id: i64) -> Result<InstallationResponse, AppError> {
        let o = self.repo.start(id).await.map_err(|_| AppError::NotFound("Installation not found".into()))?;
        Ok(InstallationResponse { id: o.id, customer_id: o.customer_id, branch_id: o.branch_id, subscription_id: o.subscription_id, assigned_technician_id: o.assigned_technician_id, status: o.status, scheduled_date: o.scheduled_date, scheduled_time_slot: o.scheduled_time_slot, completed_at: o.completed_at, installation_type: o.installation_type, notes: o.notes, created_at: o.created_at, customer_name: None, technician_name: None })
    }

    pub async fn complete_installation(&self, id: i64, req: CompleteInstallationRequest) -> Result<InstallationResponse, AppError> {
        let o = self.repo.complete(id, req.fiber_drop_length_meters, req.onu_power_dbm, req.equipment_issued, req.notes.as_deref()).await.map_err(|_| AppError::NotFound("Installation not found".into()))?;
        Ok(InstallationResponse { id: o.id, customer_id: o.customer_id, branch_id: o.branch_id, subscription_id: o.subscription_id, assigned_technician_id: o.assigned_technician_id, status: o.status, scheduled_date: o.scheduled_date, scheduled_time_slot: o.scheduled_time_slot, completed_at: o.completed_at, installation_type: o.installation_type, notes: o.notes, created_at: o.created_at, customer_name: None, technician_name: None })
    }

    pub async fn cancel_installation(&self, id: i64) -> Result<MessageResponse, AppError> {
        self.repo.cancel(id).await.map_err(|_| AppError::NotFound("Installation not found".into()))?;
        Ok(MessageResponse { message: "Installation cancelled".into() })
    }

    pub async fn upload_photo(&self, id: i64, req: UploadPhotoRequest) -> Result<MessageResponse, AppError> {
        self.repo.add_photo(id, &req.photo_url).await.map_err(|_| AppError::NotFound("Installation not found".into()))?;
        Ok(MessageResponse { message: "Photo uploaded".into() })
    }

    pub async fn get_my_assignments(&self, technician_id: i64) -> Result<Vec<InstallationResponse>, AppError> {
        let orders = self.repo.get_my_assignments(technician_id).await?;
        Ok(orders.into_iter().map(|o| InstallationResponse { id: o.id, customer_id: o.customer_id, branch_id: o.branch_id, subscription_id: o.subscription_id, assigned_technician_id: o.assigned_technician_id, status: o.status, scheduled_date: o.scheduled_date, scheduled_time_slot: o.scheduled_time_slot, completed_at: o.completed_at, installation_type: o.installation_type, notes: o.notes, created_at: o.created_at, customer_name: None, technician_name: None }).collect())
    }
}
