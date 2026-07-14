use axum::{routing::{get, post, delete}, Router};
use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::scheduler::controller::scheduler_controller;

pub fn scheduler_routes() -> Router<SharedState> {
    rls_setup::admin_branch_scoped(
        Router::new()
            .route("/tasks", get(scheduler_controller::list_tasks).post(scheduler_controller::create_task))
            .route("/tasks/{id}/toggle", post(scheduler_controller::toggle_task))
            .route("/tasks/{id}", delete(scheduler_controller::delete_task))
            .route("/executions", get(scheduler_controller::list_executions))
    )
}
