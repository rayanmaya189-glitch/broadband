use sea_orm::DatabaseConnection;
use crate::common::errors::app_error::AppError;
use crate::modules::reporting::repository::report_repository::ReportRepository;
use crate::modules::reporting::repository::report_schedule_repository::ReportScheduleRepository;
use crate::modules::reporting::request::report_request::*;
use crate::modules::reporting::response::report_response::*;

pub struct ReportingService<'a> { db: &'a DatabaseConnection }
impl<'a> ReportingService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    pub async fn list_reports(&self, branch_id: Option<i64>, report_type: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<ReportResponse>, i64), AppError> {
        let repo = ReportRepository::new(self.db);
        let (items, total) = repo.list(branch_id, report_type, page, per_page).await?;
        Ok((items.into_iter().map(|r| ReportResponse {
            id: r.id, branch_id: r.branch_id, user_id: r.user_id, report_type: r.report_type,
            name: r.name, parameters: r.parameters, result: r.result, status: r.status,
            file_url: r.file_url, created_at: r.created_at.into(),
            completed_at: r.completed_at.map(|v| v.into()),
        }).collect(), total))
    }

    pub async fn generate_report(&self, branch_id: Option<i64>, user_id: i64, req: GenerateReportRequest) -> Result<ReportResponse, AppError> {
        let repo = ReportRepository::new(self.db);
        let r = repo.create(branch_id, user_id, &req.report_type, &req.name, req.parameters).await?;
        Ok(ReportResponse {
            id: r.id, branch_id: r.branch_id, user_id: r.user_id, report_type: r.report_type,
            name: r.name, parameters: r.parameters, result: None, status: r.status,
            file_url: None, created_at: r.created_at.into(), completed_at: None,
        })
    }

    pub async fn list_schedules(&self, branch_id: Option<i64>) -> Result<Vec<ScheduleResponse>, AppError> {
        let repo = ReportScheduleRepository::new(self.db);
        let items = repo.list(branch_id).await?;
        Ok(items.into_iter().map(|s| ScheduleResponse {
            id: s.id, branch_id: s.branch_id, user_id: s.user_id, report_type: s.report_type,
            name: s.name, parameters: s.parameters, frequency: s.frequency,
            next_run_at: s.next_run_at.map(|v| v.into()),
            last_run_at: s.last_run_at.map(|v| v.into()),
            is_active: s.is_active, created_at: s.created_at.into(),
        }).collect())
    }

    pub async fn create_schedule(&self, branch_id: Option<i64>, user_id: i64, req: CreateScheduleRequest) -> Result<ScheduleResponse, AppError> {
        let repo = ReportScheduleRepository::new(self.db);
        let s = repo.create(branch_id, user_id, &req.report_type, &req.name, req.parameters, &req.frequency).await?;
        Ok(ScheduleResponse {
            id: s.id, branch_id: s.branch_id, user_id: s.user_id, report_type: s.report_type,
            name: s.name, parameters: s.parameters, frequency: s.frequency,
            next_run_at: s.next_run_at.map(|v| v.into()),
            last_run_at: s.last_run_at.map(|v| v.into()),
            is_active: s.is_active, created_at: s.created_at.into(),
        })
    }
}
