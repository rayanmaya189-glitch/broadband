use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::modules::monitoring::domain::value_objects::{AlertSeverity, AlertStatus};

/// Monitoring alert aggregate root
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringAlert {
    pub id: i64,
    pub device_id: i64,
    pub branch_id: i64,
    pub alert_type: String,
    pub severity: AlertSeverity,
    pub status: AlertStatus,
    pub title: String,
    pub message: String,
    pub metric_name: Option<String>,
    pub metric_value: Option<f64>,
    pub threshold_value: Option<f64>,
    pub acknowledged_by: Option<i64>,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<i64>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolution_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl MonitoringAlert {
    /// Create a new monitoring alert
    pub fn new(
        device_id: i64,
        branch_id: i64,
        alert_type: String,
        severity: AlertSeverity,
        title: String,
        message: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: 0,
            device_id,
            branch_id,
            alert_type,
            severity,
            status: AlertStatus::Active,
            title,
            message,
            metric_name: None,
            metric_value: None,
            threshold_value: None,
            acknowledged_by: None,
            acknowledged_at: None,
            resolved_by: None,
            resolved_at: None,
            resolution_notes: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Acknowledge the alert
    pub fn acknowledge(&mut self, user_id: i64) {
        self.status = AlertStatus::Acknowledged;
        self.acknowledged_by = Some(user_id);
        self.acknowledged_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Resolve the alert
    pub fn resolve(&mut self, user_id: i64, notes: Option<String>) {
        self.status = AlertStatus::Resolved;
        self.resolved_by = Some(user_id);
        self.resolved_at = Some(Utc::now());
        self.resolution_notes = notes;
        self.updated_at = Utc::now();
    }

    /// Auto-resolve the alert (when metric returns to normal)
    pub fn auto_resolve(&mut self) {
        self.status = AlertStatus::AutoResolved;
        self.resolved_at = Some(Utc::now());
        self.resolution_notes = Some("Metric returned to normal threshold".to_string());
        self.updated_at = Utc::now();
    }

    /// Check if the alert is still active
    pub fn is_active(&self) -> bool {
        matches!(self.status, AlertStatus::Active | AlertStatus::Acknowledged)
    }

    /// Check if the alert is critical
    pub fn is_critical(&self) -> bool {
        self.severity == AlertSeverity::Critical
    }
}
