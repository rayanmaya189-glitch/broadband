use axum::routing::{delete, get, post, put};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::network::controller::network_controller;

pub fn admin_routes() -> Router<SharedState> {
    rls_setup::admin_branch_scoped(
        Router::new()
            .route("/vlans", get(network_controller::list_vlans).post(network_controller::create_vlan))
            .route("/vlans/{id}", put(network_controller::update_vlan).delete(network_controller::delete_vlan))
            .route("/ip-pools", get(network_controller::list_ip_pools).post(network_controller::create_ip_pool))
            .route("/ip-pools/{pool_id}/addresses", get(network_controller::list_ip_addresses))
            .route("/ip-pools/allocate", post(network_controller::allocate_ip))
            .route("/ip-pools/release", post(network_controller::release_ip))
            .route("/pppoe/sessions", get(network_controller::list_pppoe_sessions).post(network_controller::create_pppoe_session))
            .route("/pppoe/sessions/{id}/terminate", post(network_controller::terminate_session))
            .route("/mac-bindings", get(network_controller::list_mac_bindings).post(network_controller::create_mac_binding))
            .route("/mac-bindings/{id}", delete(network_controller::delete_mac_binding))
            .route("/dhcp/leases", get(network_controller::list_dhcp_leases))
            .route("/sessions", get(network_controller::list_customer_sessions))
            .route("/topology", get(network_controller::get_topology))
    )
}
