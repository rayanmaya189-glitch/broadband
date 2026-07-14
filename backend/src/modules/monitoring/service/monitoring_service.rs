use sea_orm::DatabaseConnection;
use crate::common::errors::app_error::AppError;
use crate::modules::monitoring::repository::health_check_repository::HealthCheckRepository;
use crate::modules::monitoring::repository::metric_repository::MetricRepository;
use crate::modules::monitoring::repository::alert_repository::AlertRepository;
use crate::modules::monitoring::repository::alert_rule_repository::AlertRuleRepository;
use crate::modules::monitoring::request::monitoring_request::*;
use crate::modules::monitoring::response::monitoring_response::*;

pub struct MonitoringService<'a> { db: &'a DatabaseConnection }
impl<'a> MonitoringService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    pub async fn list_health_checks(&self, service_name: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<HealthCheckResponse>, i64), AppError> {
        let repo = HealthCheckRepository::new(self.db);
        let (items, total) = repo.list(service_name, page, per_page).await?;
        Ok((items.into_iter().map(|h| HealthCheckResponse {
            id: h.id, service_name: h.service_name, status: h.status,
            response_time_ms: h.response_time_ms, error_message: h.error_message,
            checked_at: h.checked_at.into(),
        }).collect(), total))
    }

    pub async fn record_health_check(&self, service_name: &str, status: &str, response_time_ms: Option<i32>, error_message: Option<&str>) -> Result<HealthCheckResponse, AppError> {
        let repo = HealthCheckRepository::new(self.db);
        let h = repo.record(service_name, status, response_time_ms, error_message).await?;
        Ok(HealthCheckResponse {
            id: h.id, service_name: h.service_name, status: h.status,
            response_time_ms: h.response_time_ms, error_message: h.error_message,
            checked_at: h.checked_at.into(),
        })
    }

    pub async fn record_metric(&self, metric_name: &str, metric_type: &str, value: f64, tags: Option<serde_json::Value>) -> Result<MetricResponse, AppError> {
        let repo = MetricRepository::new(self.db);
        let m = repo.record(metric_name, metric_type, value, tags).await?;
        Ok(MetricResponse {
            id: m.id, metric_name: m.metric_name, metric_type: m.metric_type,
            value: m.value, tags: m.tags, recorded_at: m.recorded_at.into(),
        })
    }

    pub async fn list_alerts(&self, status: Option<&str>, severity: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<AlertResponse>, i64), AppError> {
        let repo = AlertRepository::new(self.db);
        let (items, total) = repo.list(status, severity, page, per_page).await?;
        Ok((items.into_iter().map(|a| AlertResponse {
            id: a.id, rule_id: a.rule_id, service_name: a.service_name,
            severity: a.severity, message: a.message, status: a.status,
            acknowledged_by: a.acknowledged_by, acknowledged_at: a.acknowledged_at.map(|v| v.into()),
            resolved_at: a.resolved_at.map(|v| v.into()), created_at: a.created_at.into(),
        }).collect(), total))
    }

    pub async fn create_alert(&self, service_name: &str, severity: &str, message: &str) -> Result<AlertResponse, AppError> {
        let repo = AlertRepository::new(self.db);
        let a = repo.create(service_name, severity, message, None).await?;
        Ok(AlertResponse {
            id: a.id, rule_id: a.rule_id, service_name: a.service_name,
            severity: a.severity, message: a.message, status: a.status,
            acknowledged_by: a.acknowledged_by, acknowledged_at: a.acknowledged_at.map(|v| v.into()),
            resolved_at: a.resolved_at.map(|v| v.into()), created_at: a.created_at.into(),
        })
    }

    pub async fn acknowledge_alert(&self, id: i64, user_id: i64) -> Result<AlertResponse, AppError> {
        let repo = AlertRepository::new(self.db);
        let a = repo.acknowledge(id, user_id).await?;
        Ok(AlertResponse {
            id: a.id, rule_id: a.rule_id, service_name: a.service_name,
            severity: a.severity, message: a.message, status: a.status,
            acknowledged_by: a.acknowledged_by, acknowledged_at: a.acknowledged_at.map(|v| v.into()),
            resolved_at: a.resolved_at.map(|v| v.into()), created_at: a.created_at.into(),
        })
    }

    pub async fn resolve_alert(&self, id: i64) -> Result<AlertResponse, AppError> {
        let repo = AlertRepository::new(self.db);
        let a = repo.resolve(id).await?;
        Ok(AlertResponse {
            id: a.id, rule_id: a.rule_id, service_name: a.service_name,
            severity: a.severity, message: a.message, status: a.status,
            acknowledged_by: a.acknowledged_by, acknowledged_at: a.acknowledged_at.map(|v| v.into()),
            resolved_at: a.resolved_at.map(|v| v.into()), created_at: a.created_at.into(),
        })
    }

    pub async fn list_alert_rules(&self) -> Result<Vec<AlertRuleResponse>, AppError> {
        let repo = AlertRuleRepository::new(self.db);
        let items = repo.list().await?;
        Ok(items.into_iter().map(|r| AlertRuleResponse {
            id: r.id, name: r.name, service_name: r.service_name,
            metric_name: r.metric_name, operator: r.operator, threshold: r.threshold,
            severity: r.severity, is_active: r.is_active, created_at: r.created_at.into(),
        }).collect())
    }

    pub async fn create_alert_rule(&self, req: CreateAlertRuleRequest) -> Result<AlertRuleResponse, AppError> {
        let repo = AlertRuleRepository::new(self.db);
        let r = repo.create(&req.name, &req.service_name, &req.metric_name, &req.operator, req.threshold, &req.severity).await?;
        Ok(AlertRuleResponse {
            id: r.id, name: r.name, service_name: r.service_name,
            metric_name: r.metric_name, operator: r.operator, threshold: r.threshold,
            severity: r.severity, is_active: r.is_active, created_at: r.created_at.into(),
        })
    }
}
