use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};

use crate::modules::monitoring::application::traits::{
    AlertRuleRepository, MetricRecordRepository, MonitoringAlertRepository,
};
use crate::modules::monitoring::domain::entities::alert_rule;
use crate::modules::monitoring::domain::entities::metric_record;
use crate::modules::monitoring::domain::entities::monitoring_alert;
use crate::modules::monitoring::domain::value_objects::{AlertSeverity, AlertStatus};
use crate::shared::errors::AppError;

/// Metric record repository implementation
pub struct MetricRecordRepositoryImpl {
    db: DatabaseConnection,
}

impl MetricRecordRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl MetricRecordRepository for MetricRecordRepositoryImpl {
    async fn save(&self, record: &metric_record::Model) -> Result<i64, AppError> {
        let active_model = metric_record::ActiveModel {
            device_id: Set(record.device_id),
            branch_id: Set(record.branch_id),
            metric_name: Set(record.metric_name.clone()),
            metric_value: Set(record.metric_value),
            unit: Set(record.unit.clone()),
            tags: Set(record.tags.clone()),
            recorded_at: Set(record.recorded_at),
            created_at: Set(record.created_at),
            ..Default::default()
        };

        let result = active_model.insert(&self.db).await.map_err(|e| {
            AppError::Internal(anyhow::anyhow!("Failed to save metric record: {}", e))
        })?;

        Ok(result.id)
    }

    async fn find_by_device_id(
        &self,
        device_id: i64,
        limit: i64,
    ) -> Result<Vec<metric_record::Model>, AppError> {
        let records = metric_record::Entity::find()
            .filter(metric_record::Column::DeviceId.eq(device_id))
            .order_by_desc(metric_record::Column::RecordedAt)
            .limit(limit as u64)
            .all(&self.db)
            .await
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to query metric records: {}", e))
            })?;

        Ok(records)
    }

    async fn find_by_device_and_metric(
        &self,
        device_id: i64,
        metric_name: &str,
        limit: i64,
    ) -> Result<Vec<metric_record::Model>, AppError> {
        let records = metric_record::Entity::find()
            .filter(metric_record::Column::DeviceId.eq(device_id))
            .filter(metric_record::Column::MetricName.eq(metric_name))
            .order_by_desc(metric_record::Column::RecordedAt)
            .limit(limit as u64)
            .all(&self.db)
            .await
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to query metric records: {}", e))
            })?;

        Ok(records)
    }

    async fn get_latest_by_device(
        &self,
        device_id: i64,
        metric_name: &str,
    ) -> Result<Option<metric_record::Model>, AppError> {
        let record = metric_record::Entity::find()
            .filter(metric_record::Column::DeviceId.eq(device_id))
            .filter(metric_record::Column::MetricName.eq(metric_name))
            .order_by_desc(metric_record::Column::RecordedAt)
            .one(&self.db)
            .await
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to query metric record: {}", e))
            })?;

        Ok(record)
    }

    async fn get_average(
        &self,
        device_id: i64,
        metric_name: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Option<f64>, AppError> {
        let records = metric_record::Entity::find()
            .filter(metric_record::Column::DeviceId.eq(device_id))
            .filter(metric_record::Column::MetricName.eq(metric_name))
            .filter(metric_record::Column::RecordedAt.gte(start))
            .filter(metric_record::Column::RecordedAt.lte(end))
            .all(&self.db)
            .await
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to query metric records: {}", e))
            })?;

        if records.is_empty() {
            return Ok(None);
        }

        let sum: f64 = records.iter().map(|r| r.metric_value).sum();
        let avg = sum / records.len() as f64;
        Ok(Some(avg))
    }

    async fn delete_before(&self, cutoff: DateTime<Utc>) -> Result<u64, AppError> {
        let result = metric_record::Entity::delete_many()
            .filter(metric_record::Column::RecordedAt.lt(cutoff))
            .exec(&self.db)
            .await
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to delete metric records: {}", e))
            })?;

        Ok(result.rows_affected)
    }
}

/// Alert rule repository implementation
pub struct AlertRuleRepositoryImpl {
    db: DatabaseConnection,
}

impl AlertRuleRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AlertRuleRepository for AlertRuleRepositoryImpl {
    async fn save(&self, rule: &alert_rule::Model) -> Result<i64, AppError> {
        let active_model = alert_rule::ActiveModel {
            name: Set(rule.name.clone()),
            description: Set(rule.description.clone()),
            metric_name: Set(rule.metric_name.clone()),
            condition: Set(rule.condition.clone()),
            threshold_value: Set(rule.threshold_value),
            severity: Set(rule.severity.clone()),
            duration_seconds: Set(rule.duration_seconds),
            cooldown_seconds: Set(rule.cooldown_seconds),
            notification_channels: Set(rule.notification_channels.clone()),
            is_active: Set(rule.is_active),
            created_at: Set(rule.created_at),
            updated_at: Set(rule.updated_at),
            ..Default::default()
        };

        let result = active_model
            .insert(&self.db)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to save alert rule: {}", e)))?;

        Ok(result.id)
    }

    async fn find_active(&self) -> Result<Vec<alert_rule::Model>, AppError> {
        let rules = alert_rule::Entity::find()
            .filter(alert_rule::Column::IsActive.eq(true))
            .all(&self.db)
            .await
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to query alert rules: {}", e))
            })?;

        Ok(rules)
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<alert_rule::Model>, AppError> {
        let rule = alert_rule::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to query alert rule: {}", e))
            })?;

        Ok(rule)
    }

    async fn update(&self, rule: &alert_rule::Model) -> Result<(), AppError> {
        let active_model = alert_rule::ActiveModel {
            id: Set(rule.id),
            name: Set(rule.name.clone()),
            description: Set(rule.description.clone()),
            metric_name: Set(rule.metric_name.clone()),
            condition: Set(rule.condition.clone()),
            threshold_value: Set(rule.threshold_value),
            severity: Set(rule.severity.clone()),
            duration_seconds: Set(rule.duration_seconds),
            cooldown_seconds: Set(rule.cooldown_seconds),
            notification_channels: Set(rule.notification_channels.clone()),
            is_active: Set(rule.is_active),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };

        active_model.update(&self.db).await.map_err(|e| {
            AppError::Internal(anyhow::anyhow!("Failed to update alert rule: {}", e))
        })?;

        Ok(())
    }

    async fn delete(&self, id: i64) -> Result<(), AppError> {
        alert_rule::Entity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to delete alert rule: {}", e))
            })?;

        Ok(())
    }
}

/// Monitoring alert repository implementation
pub struct MonitoringAlertRepositoryImpl {
    db: DatabaseConnection,
}

impl MonitoringAlertRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl MonitoringAlertRepository for MonitoringAlertRepositoryImpl {
    async fn save(&self, alert: &monitoring_alert::Model) -> Result<i64, AppError> {
        let active_model = monitoring_alert::ActiveModel {
            device_id: Set(alert.device_id),
            branch_id: Set(alert.branch_id),
            alert_rule_id: Set(alert.alert_rule_id),
            alert_type: Set(alert.alert_type.clone()),
            severity: Set(alert.severity.clone()),
            status: Set(alert.status.clone()),
            title: Set(alert.title.clone()),
            message: Set(alert.message.clone()),
            metric_name: Set(alert.metric_name.clone()),
            metric_value: Set(alert.metric_value),
            threshold_value: Set(alert.threshold_value),
            acknowledged_by: Set(alert.acknowledged_by),
            acknowledged_at: Set(alert.acknowledged_at),
            resolved_by: Set(alert.resolved_by),
            resolved_at: Set(alert.resolved_at),
            resolution_notes: Set(alert.resolution_notes.clone()),
            created_at: Set(alert.created_at),
            updated_at: Set(alert.updated_at),
            ..Default::default()
        };

        let result = active_model.insert(&self.db).await.map_err(|e| {
            AppError::Internal(anyhow::anyhow!("Failed to save monitoring alert: {}", e))
        })?;

        Ok(result.id)
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<monitoring_alert::Model>, AppError> {
        let alert = monitoring_alert::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to query monitoring alert: {}", e))
            })?;

        Ok(alert)
    }

    async fn find_active_by_device(
        &self,
        device_id: i64,
    ) -> Result<Vec<monitoring_alert::Model>, AppError> {
        let alerts = monitoring_alert::Entity::find()
            .filter(monitoring_alert::Column::DeviceId.eq(device_id))
            .filter(monitoring_alert::Column::Status.is_in(vec!["active", "acknowledged"]))
            .all(&self.db)
            .await
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to query monitoring alerts: {}", e))
            })?;

        Ok(alerts)
    }

    async fn find_active(&self) -> Result<Vec<monitoring_alert::Model>, AppError> {
        let alerts = monitoring_alert::Entity::find()
            .filter(monitoring_alert::Column::Status.is_in(vec!["active", "acknowledged"]))
            .order_by_desc(monitoring_alert::Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to query monitoring alerts: {}", e))
            })?;

        Ok(alerts)
    }

    async fn find_by_severity(
        &self,
        severity: &AlertSeverity,
        status: Option<&AlertStatus>,
    ) -> Result<Vec<monitoring_alert::Model>, AppError> {
        let mut query = monitoring_alert::Entity::find()
            .filter(monitoring_alert::Column::Severity.eq(severity.as_str()));

        if let Some(s) = status {
            query = query.filter(monitoring_alert::Column::Status.eq(s.as_str()));
        }

        let alerts = query
            .order_by_desc(monitoring_alert::Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to query monitoring alerts: {}", e))
            })?;

        Ok(alerts)
    }

    async fn update_status(
        &self,
        id: i64,
        status: &AlertStatus,
        user_id: Option<i64>,
        notes: Option<String>,
    ) -> Result<(), AppError> {
        let alert = monitoring_alert::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to query monitoring alert: {}", e))
            })?;

        if let Some(alert) = alert {
            let mut active_model: monitoring_alert::ActiveModel = alert.into();
            active_model.status = Set(status.as_str().to_string());
            active_model.updated_at = Set(Utc::now());

            match status {
                AlertStatus::Acknowledged => {
                    active_model.acknowledged_by = Set(user_id);
                    active_model.acknowledged_at = Set(Some(Utc::now()));
                }
                AlertStatus::Resolved | AlertStatus::AutoResolved => {
                    active_model.resolved_by = Set(user_id);
                    active_model.resolved_at = Set(Some(Utc::now()));
                    active_model.resolution_notes = Set(notes);
                }
                _ => {}
            }

            active_model.update(&self.db).await.map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to update monitoring alert: {}", e))
            })?;
        }

        Ok(())
    }

    async fn count_active_by_branch(&self, branch_id: i64) -> Result<i64, AppError> {
        let count = monitoring_alert::Entity::find()
            .filter(monitoring_alert::Column::BranchId.eq(branch_id))
            .filter(monitoring_alert::Column::Status.is_in(vec!["active", "acknowledged"]))
            .count(&self.db)
            .await
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to count monitoring alerts: {}", e))
            })?;

        Ok(count as i64)
    }

    async fn delete_expired(&self, before: DateTime<Utc>) -> Result<u64, AppError> {
        let result = monitoring_alert::Entity::delete_many()
            .filter(monitoring_alert::Column::Status.eq("expired"))
            .filter(monitoring_alert::Column::CreatedAt.lt(before))
            .exec(&self.db)
            .await
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to delete expired alerts: {}", e))
            })?;

        Ok(result.rows_affected)
    }
}
