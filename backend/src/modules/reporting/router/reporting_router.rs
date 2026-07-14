use axum::{routing::get, Router};
use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::reporting::controller::reporting_controller;

pub fn reporting_routes() -> Router<SharedState> {
    rls_setup::admin_branch_scoped(
        Router::new()
            .route("/reports", get(reporting_controller::list_reports).post(reporting_controller::generate_report))
            .route("/reports/schedules", get(reporting_controller::list_schedules).post(reporting_controller::create_schedule))
    )
}
