use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::modules::monitoring::domain::value_objects::{AlertSeverity, AlertStatus};
use crate::shared::errors::AppError;

/// Metric record type
pub type MetricRecordModel = crate::modules::monitoring::domain::entities::metric_record::Model;
pub type AlertRuleModel = crate::modules::monitoring::domain::entities::alert_rule::Model;
pub type MonitoringAlertModel = crate::modules::monitoring::domain::entities::monitoring_alert::Model;

/// Repository trait for metric records
#[async_trait]
pub trait MetricRecordRepository: Send + Sync {
    /// Save a metric record
    async fn save(&self, record: &MetricRecordModel) -> Result<i64, AppError>;

    /// Find metric records by device ID
    async fn find_by_device_id(
        &self,
        device_id: i64,
        limit: i64,
    ) -> Result<Vec<MetricRecordModel>, AppError>;

    /// Find metric records by device ID and metric name
    async fn find_by_device_and_metric(
        &self,
        device_id: i64,
        metric_name: &str,
        limit: i64,
    ) -> Result<Vec<MetricRecordModel>, AppError>;

    /// Get latest metric for a device
    async fn get_latest_by_device(
        &self,
        device_id: i64,
        metric_name: &str,
    ) -> Result<Option<MetricRecordModel>, AppError>;

    /// Get average metric value over time period
    async fn get_average(
        &self,
        device_id: i64,
        metric_name: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Option<f64>, AppError>;

    /// Delete old metric records
    async fn delete_before(&self, cutoff: DateTime<Utc>) -> Result<u64, AppError>;
}

/// Repository trait for alert rules
#[async_trait]
pub trait AlertRuleRepository: Send + Sync {
    /// Save an alert rule
    async fn save(&self, rule: &AlertRuleModel) -> Result<i64, AppError>;

    /// Find all active alert rules
    async fn find_active(&self) -> Result<Vec<AlertRuleModel>, AppError>;

    /// Find alert rule by ID
    async fn find_by_id(&self, id: i64) -> Result<Option<AlertRuleModel>, AppError>;

    /// Update an alert rule
    async fn update(&self, rule: &AlertRuleModel) -> Result<(), AppError>;

    /// Delete an alert rule
    async fn delete(&self, id: i64) -> Result<(), AppError>;
}

/// Repository trait for monitoring alerts
#[async_trait]
pub trait MonitoringAlertRepository: Send + Sync {
    /// Save a monitoring alert
    async fn save(&self, alert: &MonitoringAlertModel) -> Result<i64, AppError>;

    /// Find alert by ID
    async fn find_by_id(&self, id: i64) -> Result<Option<MonitoringAlertModel>, AppError>;

    /// Find active alerts by device ID
    async fn find_active_by_device(
        &self,
        device_id: i64,
    ) -> Result<Vec<MonitoringAlertModel>, AppError>;

    /// Find all active alerts
    async fn find_active(&self) -> Result<Vec<MonitoringAlertModel>, AppError>;

    /// Find alerts by severity
    async fn find_by_severity(
        &self,
        severity: &AlertSeverity,
        status: Option<&AlertStatus>,
    ) -> Result<Vec<MonitoringAlertModel>, AppError>;

    /// Update alert status
    async fn update_status(
        &self,
        id: i64,
        status: &AlertStatus,
        user_id: Option<i64>,
        notes: Option<String>,
    ) -> Result<(), AppError>;

    /// Count active alerts by branch
    async fn count_active_by_branch(&self, branch_id: i64) -> Result<i64, AppError>;

    /// Delete expired alerts
    async fn delete_expired(&self, before: DateTime<Utc>) -> Result<u64, AppError>;
}

/// Application service trait for monitoring
#[async_trait]
pub trait MonitoringService: Send + Sync {
    /// Record device metrics
    async fn record_metrics(
        &self,
        device_id: i64,
        branch_id: i64,
        metrics: serde_json::Value,
    ) -> Result<i64, AppError>;

    /// Get device metrics history
    async fn get_device_metrics(
        &self,
        device_id: i64,
        metric_name: Option<&str>,
        limit: i64,
    ) -> Result<Vec<MetricRecordModel>, AppError>;

    /// Evaluate alert rules against metrics
    async fn evaluate_alert_rules(
        &self,
        device_id: i64,
        branch_id: i64,
        metrics: &serde_json::Value,
    ) -> Result<Vec<MonitoringAlertModel>, AppError>;

    /// Create a manual alert
    async fn create_alert(
        &self,
        device_id: i64,
        branch_id: i64,
        severity: &AlertSeverity,
        title: &str,
        message: &str,
    ) -> Result<i64, AppError>;

    /// Acknowledge an alert
    async fn acknowledge_alert(
        &self,
        alert_id: i64,
        user_id: i64,
    ) -> Result<(), AppError>;

    /// Resolve an alert
    async fn resolve_alert(
        &self,
        alert_id: i64,
        user_id: i64,
        notes: Option<String>,
    ) -> Result<(), AppError>;

    /// Get all active alerts
    async fn get_active_alerts(&self) -> Result<Vec<MonitoringAlertModel>, AppError>;

    /// Get alerts by branch
    async fn get_alerts_by_branch(
        &self,
        branch_id: i64,
    ) -> Result<Vec<MonitoringAlertModel>, AppError>;

    /// Cleanup old metrics and alerts
    async fn cleanup(&self) -> Result<(u64, u64), AppError>;
}
