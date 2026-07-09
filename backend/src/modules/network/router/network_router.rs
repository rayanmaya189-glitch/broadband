use axum::{middleware, routing::{delete, get, post, put}, Router};
use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::network::controller::network_controller;

pub fn network_routes() -> Router<SharedState> {
    Router::new()
        // ── VLANs ──────────────────────────────────────────
        .route("/vlans", get(network_controller::list_vlans).post(network_controller::create_vlan))
        .route("/vlans/:id", put(network_controller::update_vlan).delete(network_controller::delete_vlan))
        // ── IP Pools ───────────────────────────────────────
        .route("/ip-pools", get(network_controller::list_ip_pools).post(network_controller::create_ip_pool))
        .route("/ip-pools/:pool_id/addresses", get(network_controller::list_ip_addresses))
        .route("/ip-pools/allocate", post(network_controller::allocate_ip))
        .route("/ip-pools/release", post(network_controller::release_ip))
        // ── PPPoE Sessions ─────────────────────────────────
        .route("/pppoe/sessions", get(network_controller::list_pppoe_sessions).post(network_controller::create_pppoe_session))
        .route("/pppoe/sessions/:id/terminate", post(network_controller::terminate_session))
        // ── MAC Bindings ───────────────────────────────────
        .route("/mac-bindings", get(network_controller::list_mac_bindings).post(network_controller::create_mac_binding))
        .route("/mac-bindings/:id", delete(network_controller::delete_mac_binding))
        // ── DHCP Leases ────────────────────────────────────
        .route("/dhcp/leases", get(network_controller::list_dhcp_leases))
        // ── Customer Sessions ──────────────────────────────
        .route("/sessions", get(network_controller::list_customer_sessions))
        // ── Network Topology ───────────────────────────────
        .route("/topology", get(network_controller::get_topology))
        .layer(middleware::from_fn(jwt_middleware))
}
