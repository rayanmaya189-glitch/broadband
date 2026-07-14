use sea_orm::DatabaseConnection;
use crate::common::errors::app_error::AppError;
use crate::modules::traffic::repository::traffic_sample_repository::TrafficSampleRepository;
use crate::modules::traffic::repository::traffic_policy_repository::TrafficPolicyRepository;
use crate::modules::traffic::repository::traffic_aggregate_repository::TrafficAggregateRepository;
use crate::modules::traffic::request::traffic_request::*;
use crate::modules::traffic::response::traffic_response::*;

pub struct TrafficService<'a> { db: &'a DatabaseConnection }
impl<'a> TrafficService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }
    pub async fn record_sample(&self, req: RecordSampleRequest) -> Result<SampleResponse, AppError> {
        let repo = TrafficSampleRepository::new(self.db);
        let s = repo.record(req.customer_id, req.subscription_id, req.branch_id, req.interface_name.as_deref(), req.bytes_in, req.bytes_out, req.packets_in, req.packets_out, req.sample_duration_seconds).await?;
        Ok(SampleResponse { id: s.id, customer_id: s.customer_id, subscription_id: s.subscription_id, branch_id: s.branch_id, interface_name: s.interface_name, bytes_in: s.bytes_in, bytes_out: s.bytes_out, packets_in: s.packets_in, packets_out: s.packets_out, sample_duration_seconds: s.sample_duration_seconds, recorded_at: s.recorded_at.into() })
    }
    pub async fn list_samples(&self, customer_id: Option<i64>, page: i64, per_page: i64) -> Result<(Vec<SampleResponse>, i64), AppError> {
        let repo = TrafficSampleRepository::new(self.db);
        let (items, total) = repo.list(customer_id, page, per_page).await?;
        Ok((items.into_iter().map(|s| SampleResponse { id: s.id, customer_id: s.customer_id, subscription_id: s.subscription_id, branch_id: s.branch_id, interface_name: s.interface_name, bytes_in: s.bytes_in, bytes_out: s.bytes_out, packets_in: s.packets_in, packets_out: s.packets_out, sample_duration_seconds: s.sample_duration_seconds, recorded_at: s.recorded_at.into() }).collect(), total))
    }
    pub async fn list_policies(&self, branch_id: Option<i64>) -> Result<Vec<PolicyResponse>, AppError> {
        let repo = TrafficPolicyRepository::new(self.db);
        let items = repo.list(branch_id).await?;
        Ok(items.into_iter().map(|p| PolicyResponse { id: p.id, name: p.name, priority: p.priority, criteria: p.criteria, action: p.action, is_active: p.is_active, created_at: p.created_at.into(), updated_at: p.updated_at.into() }).collect())
    }
    pub async fn create_policy(&self, branch_id: Option<i64>, req: CreatePolicyRequest) -> Result<PolicyResponse, AppError> {
        let repo = TrafficPolicyRepository::new(self.db);
        let p = repo.create(branch_id, &req.name, req.priority, req.criteria, req.action).await?;
        Ok(PolicyResponse { id: p.id, name: p.name, priority: p.priority, criteria: p.criteria, action: p.action, is_active: p.is_active, created_at: p.created_at.into(), updated_at: p.updated_at.into() })
    }
    pub async fn list_aggregates(&self, customer_id: Option<i64>, period: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<AggregateResponse>, i64), AppError> {
        let repo = TrafficAggregateRepository::new(self.db);
        let (items, total) = repo.list(customer_id, period, page, per_page).await?;
        Ok((items.into_iter().map(|a| AggregateResponse { id: a.id, customer_id: a.customer_id, subscription_id: a.subscription_id, branch_id: a.branch_id, period: a.period, total_bytes_in: a.total_bytes_in, total_bytes_out: a.total_bytes_out, peak_bytes_in: a.peak_bytes_in, peak_bytes_out: a.peak_bytes_out, sample_count: a.sample_count, period_start: a.period_start.into(), period_end: a.period_end.into() }).collect(), total))
    }
}
