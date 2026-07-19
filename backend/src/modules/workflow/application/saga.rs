use crate::modules::workflow::domain::entities::{
    workflow_instance, workflow_step, WorkflowInstanceActiveModel, WorkflowStepActiveModel,
};
use crate::shared::errors::AppError;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};
use std::collections::HashMap;
use tracing::{error, info, warn};

// ── Step Definition ──

/// Definition of a single step in a saga workflow.
#[derive(Debug, Clone)]
pub struct StepDefinition {
    /// Human-readable step name
    pub name: String,
    /// Target module to invoke (e.g. "billing", "network", "subscription")
    pub target_module: String,
    /// Action to perform (e.g. "create_account", "provision_bandwidth")
    pub action: String,
    /// Maximum retry attempts before failing
    pub max_retries: i32,
    /// Optional compensation action name (for rollback)
    pub compensation_action: Option<String>,
    /// Input payload template (JSONB)
    pub input_payload: serde_json::Value,
}

// ── Step Handler Trait ──

/// Trait that modules implement to handle saga steps.
/// Each module registers handlers for its actions.
#[async_trait::async_trait]
pub trait StepHandler: Send + Sync {
    /// Execute the forward action for this step.
    /// Returns output payload on success.
    async fn execute(
        &self,
        db: &DatabaseConnection,
        instance_id: i64,
        input: &serde_json::Value,
    ) -> Result<serde_json::Value, AppError>;

    /// Execute the compensation (rollback) action for this step.
    async fn compensate(
        &self,
        db: &DatabaseConnection,
        instance_id: i64,
        input: &serde_json::Value,
        output: &serde_json::Value,
    ) -> Result<(), AppError> {
        let _ = (db, instance_id, input, output);
        // Default: no-op compensation
        Ok(())
    }
}

// ── Saga Coordinator ──

/// Orchestrates multi-step saga workflows with compensation support.
///
/// The saga coordinator:
/// 1. Creates a workflow instance with ordered steps
/// 2. Executes steps sequentially
/// 3. On step failure, retries up to `max_retries` times
/// 4. If all retries exhausted, triggers compensation (rollback) for completed steps
/// 5. Updates workflow status throughout
pub struct SagaCoordinator {
    handlers: HashMap<String, Box<dyn StepHandler>>,
}

impl SagaCoordinator {
    /// Create a new saga coordinator with an empty handler registry
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Register a step handler for a given action key (format: "module.action")
    pub fn register_handler(&mut self, action_key: String, handler: Box<dyn StepHandler>) {
        self.handlers.insert(action_key, handler);
    }

    /// Create a new workflow instance with the given steps
    pub async fn start_workflow(
        &self,
        db: &DatabaseConnection,
        workflow_type: &str,
        reference_type: &str,
        reference_id: i64,
        steps: Vec<StepDefinition>,
        input_data: serde_json::Value,
        initiated_by: Option<i64>,
        branch_id: Option<i64>,
    ) -> Result<i64, AppError> {
        let now = Utc::now();
        let total_steps = steps.len() as i32;

        // Create workflow instance
        let instance = WorkflowInstanceActiveModel {
            workflow_type: Set(workflow_type.to_string()),
            reference_type: Set(reference_type.to_string()),
            reference_id: Set(reference_id),
            status: Set("pending".to_string()),
            current_step: Set(0),
            total_steps: Set(total_steps),
            input_data: Set(input_data),
            output_data: Set(None),
            error_message: Set(None),
            initiated_by: Set(initiated_by),
            branch_id: Set(branch_id),
            started_at: Set(now),
            completed_at: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        let saved_instance = instance.insert(db).await.map_err(|e| {
            error!(error = %e, "Failed to create workflow instance");
            AppError::Internal(anyhow::anyhow!("Failed to create workflow instance: {}", e))
        })?;

        // Create step records
        for (idx, step_def) in steps.into_iter().enumerate() {
            let step = WorkflowStepActiveModel {
                workflow_instance_id: Set(saved_instance.id),
                step_name: Set(step_def.name),
                step_order: Set(idx as i32),
                target_module: Set(step_def.target_module),
                action: Set(step_def.action),
                input_payload: Set(step_def.input_payload),
                output_payload: Set(None),
                status: Set("pending".to_string()),
                error_message: Set(None),
                retry_count: Set(0),
                max_retries: Set(step_def.max_retries),
                compensation_action: Set(step_def.compensation_action),
                compensation_executed: Set(false),
                started_at: Set(None),
                completed_at: Set(None),
                created_at: Set(now),
                ..Default::default()
            };

            step.insert(db).await.map_err(|e| {
                error!(error = %e, workflow_id = saved_instance.id, "Failed to create workflow step");
                AppError::Internal(anyhow::anyhow!("Failed to create workflow step: {}", e))
            })?;
        }

        info!(
            workflow_id = saved_instance.id,
            workflow_type = workflow_type,
            total_steps = total_steps,
            "Workflow instance created"
        );

        Ok(saved_instance.id)
    }

    /// Execute a workflow instance from its current step
    pub async fn execute_workflow(
        &self,
        db: &DatabaseConnection,
        instance_id: i64,
    ) -> Result<serde_json::Value, AppError> {
        // Load instance
        let instance = workflow_instance::Entity::find_by_id(instance_id)
            .one(db)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to load workflow: {}", e)))?
            .ok_or_else(|| AppError::NotFound("Workflow instance not found".to_string()))?;

        if instance.status == "completed" {
            return Ok(instance.output_data.unwrap_or(serde_json::json!(null)));
        }
        if instance.status == "failed" || instance.status == "cancelled" {
            return Err(AppError::BadRequest(format!(
                "Workflow is in '{}' state and cannot be executed",
                instance.status
            )));
        }

        // Update status to running
        let now = Utc::now();
        let mut inst_active: WorkflowInstanceActiveModel = instance.into();
        inst_active.status = Set("running".to_string());
        inst_active.updated_at = Set(now);
        let instance = inst_active.update(db).await.map_err(|e| {
            AppError::Internal(anyhow::anyhow!("Failed to update workflow status: {}", e))
        })?;

        // Load all steps ordered by step_order
        let steps = workflow_step::Entity::find()
            .filter(workflow_step::Column::WorkflowInstanceId.eq(instance.id))
            .order_by_asc(workflow_step::Column::StepOrder)
            .all(db)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to load steps: {}", e)))?;

        let mut last_output: Option<serde_json::Value> = None;

        // Execute steps from current_step index
        for step in &steps {
            if step.step_order < instance.current_step {
                // Skip already completed steps
                if let Some(ref out) = step.output_payload {
                    last_output = Some(out.clone());
                }
                continue;
            }

            if step.status == "completed" {
                last_output = step.output_payload.clone();
                continue;
            }

            // Find handler
            let action_key = format!("{}.{}", step.target_module, step.action);
            let handler = self.handlers.get(&action_key).ok_or_else(|| {
                error!(
                    action_key = %action_key,
                    step_name = %step.step_name,
                    "No handler registered for action"
                );
                AppError::Internal(anyhow::anyhow!(
                    "No handler registered for action: {}",
                    action_key
                ))
            })?;

            // Execute with retries
            let result = self
                .execute_step_with_retry(db, handler, step, &last_output)
                .await;

            match result {
                Ok(output) => {
                    last_output = Some(output.clone());

                    // Mark step completed
                    let now = Utc::now();
                    let mut step_active: WorkflowStepActiveModel = step.clone().into();
                    step_active.status = Set("completed".to_string());
                    step_active.output_payload = Set(Some(output));
                    step_active.completed_at = Set(Some(now));
                    step_active.update(db).await.map_err(|e| {
                        AppError::Internal(anyhow::anyhow!("Failed to mark step completed: {}", e))
                    })?;

                    // Update instance current_step
                    let mut inst: WorkflowInstanceActiveModel = instance.clone().into();
                    inst.current_step = Set(step.step_order + 1);
                    inst.updated_at = Set(Utc::now());
                    inst.update(db).await.map_err(|e| {
                        AppError::Internal(anyhow::anyhow!("Failed to update current_step: {}", e))
                    })?;

                    info!(
                        workflow_id = instance.id,
                        step_name = %step.step_name,
                        step_order = step.step_order,
                        "Step completed successfully"
                    );
                }
                Err(e) => {
                    error!(
                        workflow_id = instance.id,
                        step_name = %step.step_name,
                        error = %e,
                        "Step failed after all retries"
                    );

                    // Mark step failed
                    let now = Utc::now();
                    let mut step_active: WorkflowStepActiveModel = step.clone().into();
                    step_active.status = Set("failed".to_string());
                    step_active.error_message = Set(Some(e.to_string()));
                    step_active.completed_at = Set(Some(now));
                    step_active.update(db).await.ok();

                    // Trigger compensation for completed steps
                    self.compensate(db, &steps, step.step_order).await;

                    // Mark workflow as failed
                    let mut inst: WorkflowInstanceActiveModel = instance.clone().into();
                    inst.status = Set("failed".to_string());
                    inst.error_message =
                        Set(Some(format!("Step '{}' failed: {}", step.step_name, e)));
                    inst.completed_at = Set(Some(Utc::now()));
                    inst.updated_at = Set(Utc::now());
                    inst.update(db).await.ok();

                    return Err(AppError::Internal(anyhow::anyhow!(
                        "Workflow failed at step '{}': {}",
                        step.step_name,
                        e
                    )));
                }
            }
        }

        // All steps completed
        let now = Utc::now();
        let mut inst: WorkflowInstanceActiveModel = instance.clone().into();
        inst.status = Set("completed".to_string());
        inst.output_data = Set(last_output.clone());
        inst.completed_at = Set(Some(now));
        inst.updated_at = Set(now);
        inst.update(db).await.map_err(|e| {
            AppError::Internal(anyhow::anyhow!("Failed to mark workflow completed: {}", e))
        })?;

        info!(workflow_id = instance.id, "Workflow completed successfully");

        Ok(last_output.unwrap_or(serde_json::json!(null)))
    }

    /// Execute a single step with retry logic
    async fn execute_step_with_retry(
        &self,
        db: &DatabaseConnection,
        handler: &Box<dyn StepHandler>,
        step: &workflow_step::Model,
        previous_output: &Option<serde_json::Value>,
    ) -> Result<serde_json::Value, AppError> {
        let max_retries = step.max_retries.max(1);
        let mut last_error = None;

        for attempt in 0..max_retries {
            // Merge previous output into step input for chained data passing
            let mut input = step.input_payload.clone();
            if let Some(ref prev) = previous_output {
                if let Some(obj) = input.as_object_mut() {
                    // Add previous output under "_prev" key
                    obj.insert("_prev".to_string(), prev.clone());
                }
            }

            match handler.execute(db, step.workflow_instance_id, &input).await {
                Ok(output) => {
                    // Update retry count
                    let mut step_active: WorkflowStepActiveModel = step.clone().into();
                    step_active.retry_count = Set(attempt + 1);
                    step_active.started_at = Set(Some(Utc::now()));
                    step_active.update(db).await.ok();

                    return Ok(output);
                }
                Err(e) => {
                    warn!(
                        step_name = %step.step_name,
                        attempt = attempt + 1,
                        max_retries = max_retries,
                        error = %e,
                        "Step execution failed, retrying"
                    );

                    let mut step_active: WorkflowStepActiveModel = step.clone().into();
                    step_active.retry_count = Set(attempt + 1);
                    step_active.error_message = Set(Some(e.to_string()));
                    step_active.started_at = Set(Some(Utc::now()));
                    step_active.update(db).await.ok();

                    last_error = Some(e);

                    // Brief backoff before retry (exponential: 100ms, 200ms, 400ms...)
                    let backoff_ms = 100 * (1 << attempt.min(4));
                    tokio::time::sleep(tokio::time::Duration::from_millis(backoff_ms)).await;
                }
            }
        }

        Err(last_error.unwrap_or_else(|| {
            AppError::Internal(anyhow::anyhow!(
                "Step '{}' failed after {} retries",
                step.step_name,
                max_retries
            ))
        }))
    }

    /// Execute compensation (rollback) for completed steps in reverse order
    async fn compensate(
        &self,
        db: &DatabaseConnection,
        steps: &[workflow_step::Model],
        failed_step_order: i32,
    ) {
        info!(
            failed_step_order = failed_step_order,
            "Starting compensation for completed steps"
        );

        // Collect completed steps with compensation actions, in reverse order
        let steps_to_compensate: Vec<_> = steps
            .iter()
            .filter(|s| {
                s.step_order < failed_step_order
                    && s.status == "completed"
                    && s.compensation_action.is_some()
                    && !s.compensation_executed
            })
            .rev() // Reverse order for compensation
            .collect();

        if steps_to_compensate.is_empty() {
            return;
        }

        let instance_id = steps_to_compensate.first().map(|s| s.workflow_instance_id);

        for step in steps_to_compensate {
            let comp_action = match &step.compensation_action {
                Some(a) => a.clone(),
                None => continue,
            };

            let action_key = format!("{}.{}", step.target_module, comp_action);
            if let Some(handler) = self.handlers.get(&action_key) {
                let input = step.input_payload.clone();
                let output = step.output_payload.clone().unwrap_or_default();

                match handler
                    .compensate(db, step.workflow_instance_id, &input, &output)
                    .await
                {
                    Ok(()) => {
                        info!(
                            step_name = %step.step_name,
                            compensation_action = %comp_action,
                            "Compensation executed successfully"
                        );
                        let mut step_active: WorkflowStepActiveModel = step.clone().into();
                        step_active.compensation_executed = Set(true);
                        step_active.update(db).await.ok();
                    }
                    Err(e) => {
                        error!(
                            step_name = %step.step_name,
                            compensation_action = %comp_action,
                            error = %e,
                            "Compensation failed — manual intervention required"
                        );
                    }
                }
            } else {
                warn!(
                    action_key = %action_key,
                    "No handler registered for compensation action"
                );
            }
        }

        // Update workflow status to 'compensated'
        if let Some(id) = instance_id {
            // Fetch the real instance and update its status
            if let Ok(Some(inst_model)) = workflow_instance::Entity::find_by_id(id).one(db).await {
                let mut inst: WorkflowInstanceActiveModel = inst_model.into();
                inst.status = Set("compensated".to_string());
                inst.updated_at = Set(Utc::now());
                inst.update(db).await.ok();
            }
        }
    }

    /// Cancel a running workflow
    pub async fn cancel_workflow(
        &self,
        db: &DatabaseConnection,
        instance_id: i64,
    ) -> Result<(), AppError> {
        let instance = workflow_instance::Entity::find_by_id(instance_id)
            .one(db)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to load workflow: {}", e)))?
            .ok_or_else(|| AppError::NotFound("Workflow instance not found".to_string()))?;

        if instance.status != "pending" && instance.status != "running" {
            return Err(AppError::BadRequest(format!(
                "Cannot cancel workflow in '{}' state",
                instance.status
            )));
        }

        // Load steps and compensate completed ones
        let steps = workflow_step::Entity::find()
            .filter(workflow_step::Column::WorkflowInstanceId.eq(instance_id))
            .order_by_asc(workflow_step::Column::StepOrder)
            .all(db)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to load steps: {}", e)))?;

        self.compensate(db, &steps, instance.total_steps).await;

        // Mark as cancelled
        let mut inst: WorkflowInstanceActiveModel = instance.into();
        inst.status = Set("cancelled".to_string());
        inst.completed_at = Set(Some(Utc::now()));
        inst.updated_at = Set(Utc::now());
        inst.update(db)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to cancel workflow: {}", e)))?;

        info!(workflow_id = instance_id, "Workflow cancelled");
        Ok(())
    }

    /// Get status of a workflow instance
    pub async fn get_status(
        &self,
        db: &DatabaseConnection,
        instance_id: i64,
    ) -> Result<WorkflowStatus, AppError> {
        let instance = workflow_instance::Entity::find_by_id(instance_id)
            .one(db)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to load workflow: {}", e)))?
            .ok_or_else(|| AppError::NotFound("Workflow instance not found".to_string()))?;

        let steps = workflow_step::Entity::find()
            .filter(workflow_step::Column::WorkflowInstanceId.eq(instance_id))
            .order_by_asc(workflow_step::Column::StepOrder)
            .all(db)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to load steps: {}", e)))?;

        Ok(WorkflowStatus {
            id: instance.id,
            workflow_type: instance.workflow_type,
            status: instance.status,
            current_step: instance.current_step,
            total_steps: instance.total_steps,
            error_message: instance.error_message,
            steps: steps
                .into_iter()
                .map(|s| StepStatus {
                    step_name: s.step_name,
                    step_order: s.step_order,
                    status: s.status,
                    retry_count: s.retry_count,
                    error_message: s.error_message,
                    compensation_executed: s.compensation_executed,
                })
                .collect(),
        })
    }
}

impl Default for SagaCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

// ── Response Types ──

#[derive(Debug, Clone, serde::Serialize)]
pub struct WorkflowStatus {
    pub id: i64,
    pub workflow_type: String,
    pub status: String,
    pub current_step: i32,
    pub total_steps: i32,
    pub error_message: Option<String>,
    pub steps: Vec<StepStatus>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct StepStatus {
    pub step_name: String,
    pub step_order: i32,
    pub status: String,
    pub retry_count: i32,
    pub error_message: Option<String>,
    pub compensation_executed: bool,
}

// ── Workflow Definitions ──

/// Pre-defined workflow templates for common ISP operations
pub struct WorkflowDefinitions;

impl WorkflowDefinitions {
    /// Customer onboarding workflow:
    /// 1. KYC verification
    /// 2. Create billing account
    /// 3. Assign plan
    /// 4. Provision network (VLAN/IP)
    /// 5. Schedule installation
    pub fn customer_onboarding(customer_id: i64, plan_id: i64) -> Vec<StepDefinition> {
        vec![
            StepDefinition {
                name: "KYC Verification".to_string(),
                target_module: "compliance".to_string(),
                action: "verify_kyc".to_string(),
                max_retries: 2,
                compensation_action: None,
                input_payload: serde_json::json!({"customer_id": customer_id}),
            },
            StepDefinition {
                name: "Create Billing Account".to_string(),
                target_module: "billing".to_string(),
                action: "create_account".to_string(),
                max_retries: 3,
                compensation_action: Some("deactivate_account".to_string()),
                input_payload: serde_json::json!({"customer_id": customer_id, "plan_id": plan_id}),
            },
            StepDefinition {
                name: "Assign Subscription Plan".to_string(),
                target_module: "subscription".to_string(),
                action: "create_subscription".to_string(),
                max_retries: 3,
                compensation_action: Some("cancel_subscription".to_string()),
                input_payload: serde_json::json!({"customer_id": customer_id, "plan_id": plan_id}),
            },
            StepDefinition {
                name: "Provision Network".to_string(),
                target_module: "network".to_string(),
                action: "provision_vlan".to_string(),
                max_retries: 2,
                compensation_action: Some("deprovision_vlan".to_string()),
                input_payload: serde_json::json!({"customer_id": customer_id}),
            },
            StepDefinition {
                name: "Schedule Installation".to_string(),
                target_module: "installation".to_string(),
                action: "schedule".to_string(),
                max_retries: 2,
                compensation_action: Some("cancel_installation".to_string()),
                input_payload: serde_json::json!({"customer_id": customer_id}),
            },
        ]
    }

    /// Plan upgrade workflow:
    /// 1. Validate eligibility
    /// 2. Prorate current billing
    /// 3. Update subscription
    /// 4. Update bandwidth profile
    pub fn plan_upgrade(
        customer_id: i64,
        subscription_id: i64,
        new_plan_id: i64,
    ) -> Vec<StepDefinition> {
        vec![
            StepDefinition {
                name: "Validate Upgrade Eligibility".to_string(),
                target_module: "subscription".to_string(),
                action: "validate_upgrade".to_string(),
                max_retries: 1,
                compensation_action: None,
                input_payload: serde_json::json!({
                    "customer_id": customer_id,
                    "subscription_id": subscription_id,
                    "new_plan_id": new_plan_id,
                }),
            },
            StepDefinition {
                name: "Prorate Current Billing".to_string(),
                target_module: "billing".to_string(),
                action: "prorate_invoice".to_string(),
                max_retries: 2,
                compensation_action: Some("reverse_proration".to_string()),
                input_payload: serde_json::json!({
                    "customer_id": customer_id,
                    "subscription_id": subscription_id,
                }),
            },
            StepDefinition {
                name: "Update Subscription Plan".to_string(),
                target_module: "subscription".to_string(),
                action: "change_plan".to_string(),
                max_retries: 3,
                compensation_action: Some("revert_plan".to_string()),
                input_payload: serde_json::json!({
                    "subscription_id": subscription_id,
                    "new_plan_id": new_plan_id,
                }),
            },
            StepDefinition {
                name: "Update Bandwidth Profile".to_string(),
                target_module: "bandwidth".to_string(),
                action: "update_profile".to_string(),
                max_retries: 2,
                compensation_action: Some("revert_profile".to_string()),
                input_payload: serde_json::json!({
                    "customer_id": customer_id,
                    "new_plan_id": new_plan_id,
                }),
            },
        ]
    }

    /// Payment failure recovery workflow:
    /// 1. Retry payment (up to 3 times)
    /// 2. Send payment reminder
    /// 3. Suspend subscription
    /// 4. Schedule deprovisioning (after grace period)
    pub fn payment_failure_recovery(
        customer_id: i64,
        subscription_id: i64,
        invoice_id: i64,
    ) -> Vec<StepDefinition> {
        vec![
            StepDefinition {
                name: "Retry Payment".to_string(),
                target_module: "payment".to_string(),
                action: "retry_payment".to_string(),
                max_retries: 3,
                compensation_action: None,
                input_payload: serde_json::json!({
                    "customer_id": customer_id,
                    "invoice_id": invoice_id,
                }),
            },
            StepDefinition {
                name: "Send Payment Reminder".to_string(),
                target_module: "notification".to_string(),
                action: "send_reminder".to_string(),
                max_retries: 2,
                compensation_action: None,
                input_payload: serde_json::json!({
                    "customer_id": customer_id,
                    "invoice_id": invoice_id,
                    "template": "payment_reminder",
                }),
            },
            StepDefinition {
                name: "Suspend Subscription".to_string(),
                target_module: "subscription".to_string(),
                action: "suspend".to_string(),
                max_retries: 2,
                compensation_action: Some("reactivate".to_string()),
                input_payload: serde_json::json!({
                    "subscription_id": subscription_id,
                    "reason": "payment_failure",
                }),
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_definition_creation() {
        let step = StepDefinition {
            name: "Test Step".to_string(),
            target_module: "billing".to_string(),
            action: "create_account".to_string(),
            max_retries: 3,
            compensation_action: Some("deactivate_account".to_string()),
            input_payload: serde_json::json!({"key": "value"}),
        };

        assert_eq!(step.name, "Test Step");
        assert_eq!(step.max_retries, 3);
        assert!(step.compensation_action.is_some());
    }

    #[test]
    fn test_customer_onboarding_workflow_steps() {
        let steps = WorkflowDefinitions::customer_onboarding(1, 10);
        assert_eq!(steps.len(), 5);
        assert_eq!(steps[0].name, "KYC Verification");
        assert_eq!(steps[4].name, "Schedule Installation");

        // Verify compensation actions exist for non-trivial steps
        assert!(steps[1].compensation_action.is_some());
        assert!(steps[2].compensation_action.is_some());
        assert!(steps[3].compensation_action.is_some());
    }

    #[test]
    fn test_plan_upgrade_workflow_steps() {
        let steps = WorkflowDefinitions::plan_upgrade(1, 2, 20);
        assert_eq!(steps.len(), 4);
        assert_eq!(steps[0].name, "Validate Upgrade Eligibility");
        assert_eq!(steps[2].name, "Update Subscription Plan");
    }

    #[test]
    fn test_payment_failure_recovery_steps() {
        let steps = WorkflowDefinitions::payment_failure_recovery(1, 2, 30);
        assert_eq!(steps.len(), 3);
        assert_eq!(steps[0].action, "retry_payment");
        assert_eq!(steps[2].action, "suspend");
    }

    #[test]
    fn test_saga_coordinator_default() {
        let coordinator = SagaCoordinator::new();
        assert!(coordinator.handlers.is_empty());
    }

    #[test]
    fn test_workflow_status_serialization() {
        let status = WorkflowStatus {
            id: 1,
            workflow_type: "customer_onboarding".to_string(),
            status: "running".to_string(),
            current_step: 2,
            total_steps: 5,
            error_message: None,
            steps: vec![StepStatus {
                step_name: "KYC Verification".to_string(),
                step_order: 0,
                status: "completed".to_string(),
                retry_count: 1,
                error_message: None,
                compensation_executed: false,
            }],
        };

        let json = serde_json::to_value(&status).unwrap();
        assert_eq!(json["id"], 1);
        assert_eq!(json["status"], "running");
        assert_eq!(json["steps"][0]["step_name"], "KYC Verification");
    }
}
