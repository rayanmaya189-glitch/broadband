use axum::{routing::get, Router};
use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::network::controller::network_controller;

pub fn network_routes() -> Router<SharedState> {
    rls_setup::branch_scoped(
        Router::new()
            .route("/vlans", get(network_controller::list_vlans).post(network_controller::create_vlan))
            .route("/ip-pools", get(network_controller::list_ip_pools).post(network_controller::create_ip_pool))
            .route("/pppoe/sessions", get(network_controller::list_pppoe_sessions).post(network_controller::create_pppoe_session))
            .route("/mac-bindings", get(network_controller::list_mac_bindings).post(network_controller::create_mac_binding))
    )
}
