use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, QueryOrder, PaginatorTrait, Set, ActiveModelTrait, IntoActiveModel};
use crate::common::errors::app_error::AppError;
use crate::modules::workflow::model::workflow_instance_entity::{self, Model as InstanceModel};
use crate::modules::workflow::model::workflow_step_instance_entity;

pub struct WorkflowInstanceRepository<'a> { db: &'a DatabaseConnection }
impl<'a> WorkflowInstanceRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }
    pub async fn list(&self, branch_id: Option<i64>, status: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<InstanceModel>, i64), AppError> {
        let page_size = per_page as u64; let page_num = if per_page > 0 { ((page - 1).max(0) as u64 * page_size) / page_size } else { 0 };
        let mut select = workflow_instance_entity::Entity::find();
        if let Some(bid) = branch_id { select = select.filter(workflow_instance_entity::Column::BranchId.eq(bid)); }
        if let Some(s) = status { select = select.filter(workflow_instance_entity::Column::Status.eq(s)); }
        let total = select.clone().count(self.db).await? as i64;
        let items = select.order_by_desc(workflow_instance_entity::Column::StartedAt).paginate(self.db, page_size).fetch_page(page_num).await?;
        Ok((items, total))
    }
    pub async fn create(&self, definition_id: i64, branch_id: Option<i64>, entity_id: i64, started_by: i64) -> Result<InstanceModel, AppError> {
        let active = workflow_instance_entity::ActiveModel {
            definition_id: Set(definition_id), branch_id: Set(branch_id), entity_id: Set(entity_id),
            status: Set("in_progress".to_owned()), current_step_index: Set(0),
            started_by: Set(started_by), started_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }
    pub async fn complete_step(&self, _instance_id: i64, step_instance_id: i64, decision: &str, decided_by: i64, comments: Option<&str>) -> Result<(), AppError> {
        let existing = workflow_step_instance_entity::Entity::find_by_id(step_instance_id).one(self.db).await?.ok_or_else(|| AppError::NotFound("Step instance not found".into()))?;
        let mut active = existing.into_active_model();
        active.status = Set("completed".to_owned());
        active.decision = Set(Some(decision.to_owned()));
        active.decided_by = Set(Some(decided_by));
        active.comments = Set(comments.map(|s| s.to_owned()));
        active.completed_at = Set(Some(chrono::Utc::now().into()));
        active.update(self.db).await?;
        Ok(())
    }
    pub async fn update_status(&self, id: i64, status: &str) -> Result<(), AppError> {
        let existing = workflow_instance_entity::Entity::find_by_id(id).one(self.db).await?.ok_or_else(|| AppError::NotFound("Instance not found".into()))?;
        let mut active = existing.into_active_model();
        active.status = Set(status.to_owned());
        if status == "completed" || status == "rejected" { active.completed_at = Set(Some(chrono::Utc::now().into())); }
        active.update(self.db).await?;
        Ok(())
    }
}
