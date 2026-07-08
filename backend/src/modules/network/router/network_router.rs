use axum::{middleware, routing::{delete, get, post}, Router};
use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::network::controller::network_controller;

pub fn network_routes() -> Router<SharedState> {
    Router::new()
        .route("/vlans", get(network_controller::list_vlans).post(network_controller::create_vlan))
        .route("/vlans/:id", delete(network_controller::delete_vlan))
        .route("/ip-pools", get(network_controller::list_ip_pools).post(network_controller::create_ip_pool))
        .route("/pppoe/sessions", get(network_controller::list_pppoe_sessions))
        .route("/pppoe/sessions/:id/terminate", post(network_controller::terminate_session))
        .layer(middleware::from_fn(jwt_middleware))
}
