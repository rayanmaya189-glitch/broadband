use sea_orm::DatabaseConnection;
use crate::common::errors::app_error::AppError;
use crate::modules::workflow::repository::workflow_repository::WorkflowRepository;
use crate::modules::workflow::repository::workflow_instance_repository::WorkflowInstanceRepository;
use crate::modules::workflow::request::workflow_request::*;
use crate::modules::workflow::response::workflow_response::*;

pub struct WorkflowService<'a> { db: &'a DatabaseConnection }
impl<'a> WorkflowService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }
    pub async fn list_definitions(&self, branch_id: Option<i64>) -> Result<Vec<DefinitionResponse>, AppError> {
        let repo = WorkflowRepository::new(self.db);
        let items = repo.list(branch_id).await?;
        Ok(items.into_iter().map(|d| DefinitionResponse { id: d.id, name: d.name, description: d.description, entity_type: d.entity_type, is_active: d.is_active, version: d.version, created_at: d.created_at.into(), updated_at: d.updated_at.into() }).collect())
    }
    pub async fn create_definition(&self, branch_id: Option<i64>, req: CreateDefinitionRequest) -> Result<DefinitionResponse, AppError> {
        let repo = WorkflowRepository::new(self.db);
        let d = repo.create(branch_id, &req.name, req.description.as_deref(), &req.entity_type).await?;
        Ok(DefinitionResponse { id: d.id, name: d.name, description: d.description, entity_type: d.entity_type, is_active: d.is_active, version: d.version, created_at: d.created_at.into(), updated_at: d.updated_at.into() })
    }
    pub async fn add_step(&self, definition_id: i64, req: AddStepRequest) -> Result<MessageResponse, AppError> {
        let repo = WorkflowRepository::new(self.db);
        repo.add_step(definition_id, &req.name, &req.step_type, req.step_order, req.required_role.as_deref(), req.config).await?;
        Ok(MessageResponse { message: "Step added".into() })
    }
    pub async fn list_instances(&self, branch_id: Option<i64>, status: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<InstanceResponse>, i64), AppError> {
        let repo = WorkflowInstanceRepository::new(self.db);
        let (items, total) = repo.list(branch_id, status, page, per_page).await?;
        Ok((items.into_iter().map(|i| InstanceResponse { id: i.id, definition_id: i.definition_id, entity_id: i.entity_id, status: i.status, current_step_index: i.current_step_index, started_by: i.started_by, started_at: i.started_at.into(), completed_at: i.completed_at.map(|v| v.into()), notes: i.notes }).collect(), total))
    }
    pub async fn start_instance(&self, definition_id: i64, branch_id: Option<i64>, entity_id: i64, started_by: i64) -> Result<InstanceResponse, AppError> {
        let repo = WorkflowInstanceRepository::new(self.db);
        let i = repo.create(definition_id, branch_id, entity_id, started_by).await?;
        Ok(InstanceResponse { id: i.id, definition_id: i.definition_id, entity_id: i.entity_id, status: i.status, current_step_index: i.current_step_index, started_by: i.started_by, started_at: i.started_at.into(), completed_at: None, notes: None })
    }
    pub async fn approve_step(&self, instance_id: i64, step_instance_id: i64, decided_by: i64, comments: Option<&str>) -> Result<MessageResponse, AppError> {
        let repo = WorkflowInstanceRepository::new(self.db);
        repo.complete_step(instance_id, step_instance_id, "approved", decided_by, comments).await?;
        Ok(MessageResponse { message: "Step approved".into() })
    }
    pub async fn reject_step(&self, instance_id: i64, step_instance_id: i64, decided_by: i64, comments: Option<&str>) -> Result<MessageResponse, AppError> {
        let repo = WorkflowInstanceRepository::new(self.db);
        repo.complete_step(instance_id, step_instance_id, "rejected", decided_by, comments).await?;
        repo.update_status(instance_id, "rejected").await?;
        Ok(MessageResponse { message: "Step rejected".into() })
    }
}
