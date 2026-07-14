use sea_orm::DatabaseConnection;
use crate::common::errors::app_error::AppError;
use crate::modules::automation::repository::automation_rule_repository::AutomationRuleRepository;
use crate::modules::automation::repository::automation_execution_repository::AutomationExecutionRepository;
use crate::modules::automation::request::automation_request::*;
use crate::modules::automation::response::automation_response::*;

pub struct AutomationService<'a> { db: &'a DatabaseConnection }
impl<'a> AutomationService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }
    pub async fn list_rules(&self, branch_id: Option<i64>) -> Result<Vec<RuleResponse>, AppError> {
        let repo = AutomationRuleRepository::new(self.db);
        let items = repo.list(branch_id).await?;
        Ok(items.into_iter().map(|r| RuleResponse { id: r.id, name: r.name, description: r.description, priority: r.priority, is_active: r.is_active, created_at: r.created_at.into(), updated_at: r.updated_at.into() }).collect())
    }
    pub async fn create_rule(&self, branch_id: Option<i64>, req: CreateRuleRequest) -> Result<RuleResponse, AppError> {
        let repo = AutomationRuleRepository::new(self.db);
        let r = repo.create(branch_id, &req.name, req.description.as_deref(), req.priority.unwrap_or(0)).await?;
        Ok(RuleResponse { id: r.id, name: r.name, description: r.description, priority: r.priority, is_active: r.is_active, created_at: r.created_at.into(), updated_at: r.updated_at.into() })
    }
    pub async fn add_trigger(&self, rule_id: i64, req: AddTriggerRequest) -> Result<MessageResponse, AppError> {
        let repo = AutomationRuleRepository::new(self.db);
        repo.add_trigger(rule_id, &req.trigger_type, req.config).await?;
        Ok(MessageResponse { message: "Trigger added".into() })
    }
    pub async fn add_action(&self, rule_id: i64, req: AddActionRequest) -> Result<MessageResponse, AppError> {
        let repo = AutomationRuleRepository::new(self.db);
        repo.add_action(rule_id, &req.action_type, req.config, req.order_index.unwrap_or(0)).await?;
        Ok(MessageResponse { message: "Action added".into() })
    }
    pub async fn list_executions(&self, rule_id: Option<i64>, page: i64, per_page: i64) -> Result<(Vec<ExecutionResponse>, i64), AppError> {
        let repo = AutomationExecutionRepository::new(self.db);
        let (items, total) = repo.list(rule_id, page, per_page).await?;
        Ok((items.into_iter().map(|e| ExecutionResponse { id: e.id, rule_id: e.rule_id, status: e.status, trigger_data: e.trigger_data, result: e.result, error_message: e.error_message, started_at: e.started_at.into(), completed_at: e.completed_at.map(|v| v.into()) }).collect(), total))
    }
    pub async fn delete_rule(&self, id: i64) -> Result<MessageResponse, AppError> {
        let repo = AutomationRuleRepository::new(self.db);
        if !repo.delete(id).await? { return Err(AppError::NotFound("Rule not found".into())); }
        Ok(MessageResponse { message: "Rule deleted".into() })
    }
}
