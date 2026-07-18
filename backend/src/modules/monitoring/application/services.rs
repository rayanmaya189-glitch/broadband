use std::sync::Arc;

use chrono::{Duration, Utc};
use tracing::{error, info};

use crate::modules::monitoring::application::traits::{
    AlertRuleRepository, MetricRecordRepository, MonitoringAlertRepository, MonitoringService,
};
use crate::modules::monitoring::domain::entities::metric_record;
use crate::modules::monitoring::domain::entities::monitoring_alert;
use crate::modules::monitoring::domain::rules::monitoring_rules;
use crate::modules::monitoring::domain::value_objects::{AlertSeverity, AlertStatus};
use crate::shared::errors::AppError;

/// Monitoring application service implementation
pub struct MonitoringServiceImpl {
    metric_repo: Arc<dyn MetricRecordRepository>,
    alert_rule_repo: Arc<dyn AlertRuleRepository>,
    alert_repo: Arc<dyn MonitoringAlertRepository>,
}

impl MonitoringServiceImpl {
    pub fn new(
        metric_repo: Arc<dyn MetricRecordRepository>,
        alert_rule_repo: Arc<dyn AlertRuleRepository>,
        alert_repo: Arc<dyn MonitoringAlertRepository>,
    ) -> Self {
        Self {
            metric_repo,
            alert_rule_repo,
            alert_repo,
        }
    }
}

#[async_trait::async_trait]
impl MonitoringService for MonitoringServiceImpl {
    async fn record_metrics(
        &self,
        device_id: i64,
        branch_id: i64,
        metrics: serde_json::Value,
    ) -> Result<i64, AppError> {
        let mut last_id = 0;

        // Record each metric
        if let Some(cpu) = metrics.get("cpu_usage").and_then(|v| v.as_f64()) {
            let record = metric_record::Model {
                id: 0,
                device_id,
                branch_id,
                metric_name: "cpu_usage".to_string(),
                metric_value: cpu,
                unit: Some("%".to_string()),
                tags: None,
                recorded_at: Utc::now(),
                created_at: Utc::now(),
            };
            last_id = self.metric_repo.save(&record).await?;
            info!(
                device_id,
                metric = "cpu_usage",
                value = cpu,
                "Recorded metric"
            );
        }

        if let Some(memory) = metrics.get("memory_usage").and_then(|v| v.as_f64()) {
            let record = metric_record::Model {
                id: 0,
                device_id,
                branch_id,
                metric_name: "memory_usage".to_string(),
                metric_value: memory,
                unit: Some("%".to_string()),
                tags: None,
                recorded_at: Utc::now(),
                created_at: Utc::now(),
            };
            last_id = self.metric_repo.save(&record).await?;
            info!(
                device_id,
                metric = "memory_usage",
                value = memory,
                "Recorded metric"
            );
        }

        if let Some(temp) = metrics.get("temperature").and_then(|v| v.as_f64()) {
            let record = metric_record::Model {
                id: 0,
                device_id,
                branch_id,
                metric_name: "temperature".to_string(),
                metric_value: temp,
                unit: Some("°C".to_string()),
                tags: None,
                recorded_at: Utc::now(),
                created_at: Utc::now(),
            };
            last_id = self.metric_repo.save(&record).await?;
        }

        if let Some(rx_power) = metrics.get("rx_power").and_then(|v| v.as_f64()) {
            let record = metric_record::Model {
                id: 0,
                device_id,
                branch_id,
                metric_name: "rx_power".to_string(),
                metric_value: rx_power,
                unit: Some("dBm".to_string()),
                tags: None,
                recorded_at: Utc::now(),
                created_at: Utc::now(),
            };
            last_id = self.metric_repo.save(&record).await?;
        }

        if let Some(health_score) = metrics.get("health_score").and_then(|v| v.as_i64()) {
            let record = metric_record::Model {
                id: 0,
                device_id,
                branch_id,
                metric_name: "device_health".to_string(),
                metric_value: health_score as f64,
                unit: Some("score".to_string()),
                tags: None,
                recorded_at: Utc::now(),
                created_at: Utc::now(),
            };
            last_id = self.metric_repo.save(&record).await?;
        }

        // Evaluate alert rules
        let _alerts = self
            .evaluate_alert_rules(device_id, branch_id, &metrics)
            .await?;

        Ok(last_id)
    }

    async fn get_device_metrics(
        &self,
        device_id: i64,
        metric_name: Option<&str>,
        limit: i64,
    ) -> Result<Vec<metric_record::Model>, AppError> {
        match metric_name {
            Some(name) => {
                self.metric_repo
                    .find_by_device_and_metric(device_id, name, limit)
                    .await
            }
            None => self.metric_repo.find_by_device_id(device_id, limit).await,
        }
    }

    async fn evaluate_alert_rules(
        &self,
        device_id: i64,
        branch_id: i64,
        metrics: &serde_json::Value,
    ) -> Result<Vec<monitoring_alert::Model>, AppError> {
        let rules = self.alert_rule_repo.find_active().await?;
        let mut _alerts = Vec::new();

        for rule in rules {
            if let Some(value) = metrics.get(&rule.metric_name).and_then(|v| v.as_f64()) {
                let should_alert = match rule.condition.as_str() {
                    "gt" => value > rule.threshold_value,
                    "gte" => value >= rule.threshold_value,
                    "lt" => value < rule.threshold_value,
                    "lte" => value <= rule.threshold_value,
                    "eq" => (value - rule.threshold_value).abs() < f64::EPSILON,
                    _ => false,
                };

                if should_alert {
                    // Check if there's already an active alert for this device and metric
                    let existing_alerts = self.alert_repo.find_active_by_device(device_id).await?;
                    let has_existing = existing_alerts.iter().any(|a| {
                        a.metric_name.as_deref() == Some(&rule.metric_name)
                            && a.alert_rule_id == Some(rule.id)
                    });

                    if !has_existing {
                        let severity = AlertSeverity::from_str(&rule.severity)
                            .unwrap_or(AlertSeverity::Medium);

                        let alert_model = monitoring_alert::Model {
                            id: 0,
                            device_id,
                            branch_id,
                            alert_rule_id: Some(rule.id),
                            alert_type: rule.metric_name.clone(),
                            severity: severity.as_str().to_string(),
                            status: AlertStatus::Active.as_str().to_string(),
                            title: format!("{} threshold exceeded", rule.metric_name),
                            message: format!(
                                "Device {} {} value {} exceeds threshold {}",
                                device_id, rule.metric_name, value, rule.threshold_value
                            ),
                            metric_name: Some(rule.metric_name.clone()),
                            metric_value: Some(value),
                            threshold_value: Some(rule.threshold_value),
                            acknowledged_by: None,
                            acknowledged_at: None,
                            resolved_by: None,
                            resolved_at: None,
                            resolution_notes: None,
                            created_at: Utc::now(),
                            updated_at: Utc::now(),
                        };

                        match self.alert_repo.save(&alert_model).await {
                            Ok(_id) => {
                                info!(
                                    device_id,
                                    metric = %rule.metric_name,
                                    value,
                                    threshold = rule.threshold_value,
                                    severity = %severity.as_str(),
                                    "Created alert"
                                );
                            }
                            Err(e) => {
                                error!(
                                    device_id,
                                    metric = %rule.metric_name,
                                    error = %e,
                                    "Failed to create alert"
                                );
                            }
                        }
                    }
                } else {
                    // Auto-resolve alerts when metric returns to normal
                    let existing_alerts = self.alert_repo.find_active_by_device(device_id).await?;
                    for alert in existing_alerts {
                        if alert.metric_name.as_deref() == Some(&rule.metric_name)
                            && alert.alert_rule_id == Some(rule.id)
                        {
                            let _ = self
                                .alert_repo
                                .update_status(
                                    alert.id,
                                    &AlertStatus::AutoResolved,
                                    None,
                                    Some("Metric returned to normal threshold".to_string()),
                                )
                                .await;
                            info!(device_id, metric = %rule.metric_name, "Auto-resolved alert");
                        }
                    }
                }
            }
        }

        Ok(_alerts)
    }

    async fn create_alert(
        &self,
        device_id: i64,
        branch_id: i64,
        severity: &AlertSeverity,
        title: &str,
        message: &str,
    ) -> Result<i64, AppError> {
        let alert_model = monitoring_alert::Model {
            id: 0,
            device_id,
            branch_id,
            alert_rule_id: None,
            alert_type: "manual".to_string(),
            severity: severity.as_str().to_string(),
            status: AlertStatus::Active.as_str().to_string(),
            title: title.to_string(),
            message: message.to_string(),
            metric_name: None,
            metric_value: None,
            threshold_value: None,
            acknowledged_by: None,
            acknowledged_at: None,
            resolved_by: None,
            resolved_at: None,
            resolution_notes: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let id = self.alert_repo.save(&alert_model).await?;
        info!(device_id, severity = %severity.as_str(), "Created manual alert");
        Ok(id)
    }

    async fn acknowledge_alert(&self, alert_id: i64, user_id: i64) -> Result<(), AppError> {
        self.alert_repo
            .update_status(alert_id, &AlertStatus::Acknowledged, Some(user_id), None)
            .await
    }

    async fn resolve_alert(
        &self,
        alert_id: i64,
        user_id: i64,
        notes: Option<String>,
    ) -> Result<(), AppError> {
        self.alert_repo
            .update_status(alert_id, &AlertStatus::Resolved, Some(user_id), notes)
            .await
    }

    async fn get_active_alerts(&self) -> Result<Vec<monitoring_alert::Model>, AppError> {
        self.alert_repo.find_active().await
    }

    async fn get_alerts_by_branch(
        &self,
        branch_id: i64,
    ) -> Result<Vec<monitoring_alert::Model>, AppError> {
        let all_alerts = self.alert_repo.find_active().await?;
        Ok(all_alerts
            .into_iter()
            .filter(|a| a.branch_id == branch_id)
            .collect())
    }

    async fn cleanup(&self) -> Result<(u64, u64), AppError> {
        let metric_cutoff = Utc::now() - Duration::days(90);
        let alert_cutoff = Utc::now() - Duration::hours(monitoring_rules::ALERT_EXPIRY_HOURS);

        let metrics_deleted = self.metric_repo.delete_before(metric_cutoff).await?;
        let alerts_deleted = self.alert_repo.delete_expired(alert_cutoff).await?;

        info!(
            metrics_deleted,
            alerts_deleted, "Monitoring cleanup completed"
        );

        Ok((metrics_deleted, alerts_deleted))
    }
}
