use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, QueryOrder, Set, ActiveModelTrait};
use crate::common::errors::app_error::AppError;
use crate::modules::workflow::model::workflow_definition_entity::{self, Model as DefinitionModel};
use crate::modules::workflow::model::workflow_step_entity;

pub struct WorkflowRepository<'a> { db: &'a DatabaseConnection }
impl<'a> WorkflowRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }
    pub async fn list(&self, branch_id: Option<i64>) -> Result<Vec<DefinitionModel>, AppError> {
        let mut select = workflow_definition_entity::Entity::find();
        if let Some(bid) = branch_id { select = select.filter(workflow_definition_entity::Column::BranchId.eq(bid)); }
        Ok(select.all(self.db).await?)
    }
    pub async fn get_by_id(&self, id: i64) -> Result<Option<DefinitionModel>, AppError> {
        Ok(workflow_definition_entity::Entity::find_by_id(id).one(self.db).await?)
    }
    pub async fn create(&self, branch_id: Option<i64>, name: &str, description: Option<&str>, entity_type: &str) -> Result<DefinitionModel, AppError> {
        let now = chrono::Utc::now();
        let active = workflow_definition_entity::ActiveModel {
            branch_id: Set(branch_id), name: Set(name.to_owned()),
            description: Set(description.map(|s| s.to_owned())),
            entity_type: Set(entity_type.to_owned()), is_active: Set(true), version: Set(1),
            created_at: Set(now.into()), updated_at: Set(now.into()), ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }
    pub async fn add_step(&self, definition_id: i64, name: &str, step_type: &str, step_order: i32, required_role: Option<&str>, config: Option<serde_json::Value>) -> Result<(), AppError> {
        let active = workflow_step_entity::ActiveModel {
            definition_id: Set(definition_id), name: Set(name.to_owned()),
            step_type: Set(step_type.to_owned()), step_order: Set(step_order),
            required_role: Set(required_role.map(|s| s.to_owned())), config: Set(config),
            created_at: Set(chrono::Utc::now().into()), ..Default::default()
        };
        active.insert(self.db).await?;
        Ok(())
    }
    pub async fn get_steps(&self, definition_id: i64) -> Result<Vec<workflow_step_entity::Model>, AppError> {
        Ok(workflow_step_entity::Entity::find().filter(workflow_step_entity::Column::DefinitionId.eq(definition_id)).order_by_asc(workflow_step_entity::Column::StepOrder).all(self.db).await?)
    }
    pub async fn delete(&self, id: i64) -> Result<bool, AppError> {
        let result = workflow_definition_entity::Entity::delete_by_id(id).exec(self.db).await?;
        Ok(result.rows_affected > 0)
    }
}
