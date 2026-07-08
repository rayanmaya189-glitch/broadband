use axum::middleware;
use axum::routing::{get, put};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::customer::controller::customer_controller;

pub fn customers_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(customer_controller::list_customers).post(customer_controller::create_customer))
        .route("/:id", get(customer_controller::get_customer).put(customer_controller::update_customer).delete(customer_controller::delete_customer))
        .route("/:id/status", put(customer_controller::update_status))
        .layer(middleware::from_fn(jwt_middleware))
}
