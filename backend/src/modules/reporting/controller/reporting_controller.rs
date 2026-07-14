use axum::extract::{Json, Query, State};
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::reporting::request::report_request::*;
use crate::modules::reporting::response::report_response::*;
use crate::modules::reporting::service::reporting_service::ReportingService;

pub async fn list_reports(State(state): State<SharedState>, Query(q): Query<ReportQuery>) -> Result<Json<Vec<ReportResponse>>, AppError> {
    let svc = ReportingService::new(&state.db);
    let (items, _) = svc.list_reports(q.branch_id, q.report_type.as_deref(), q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?;
    Ok(Json(items))
}
pub async fn generate_report(State(state): State<SharedState>, Json(req): Json<GenerateReportRequest>) -> Result<Json<ReportResponse>, AppError> {
    let svc = ReportingService::new(&state.db);
    Ok(Json(svc.generate_report(None, 0, req).await?))
}
pub async fn list_schedules(State(state): State<SharedState>) -> Result<Json<Vec<ScheduleResponse>>, AppError> {
    let svc = ReportingService::new(&state.db);
    Ok(Json(svc.list_schedules(None).await?))
}
pub async fn create_schedule(State(state): State<SharedState>, Json(req): Json<CreateScheduleRequest>) -> Result<Json<ScheduleResponse>, AppError> {
    let svc = ReportingService::new(&state.db);
    Ok(Json(svc.create_schedule(None, 0, req).await?))
}
