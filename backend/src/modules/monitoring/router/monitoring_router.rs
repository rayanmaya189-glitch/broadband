use axum::{routing::{get, post}, Router};
use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::monitoring::controller::monitoring_controller;

pub fn monitoring_routes() -> Router<SharedState> {
    rls_setup::admin_branch_scoped(
        Router::new()
            .route("/health-checks", get(monitoring_controller::list_health_checks).post(monitoring_controller::record_health_check))
            .route("/metrics", post(monitoring_controller::record_metric))
            .route("/alerts", get(monitoring_controller::list_alerts))
            .route("/alerts/{id}/acknowledge", post(monitoring_controller::acknowledge_alert))
            .route("/alerts/{id}/resolve", post(monitoring_controller::resolve_alert))
            .route("/alert-rules", get(monitoring_controller::list_alert_rules).post(monitoring_controller::create_alert_rule))
    )
}
