//! OpenAPI documentation for AeroXe Broadband API.
//! Uses utoipa for automatic Swagger UI generation.
//!
//! Stub handler functions are used here solely for OpenAPI spec generation.
//! The actual request/response types mirror the real handler module types.

use utoipa::OpenApi;

pub mod auth;
pub mod billing;
pub mod customers;
pub mod devices;
pub mod network;
pub mod subscriptions;
pub mod tickets;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "AeroXe Broadband API",
        description = "ISP Platform Backend API — Customer management, billing, network, and device management",
        version = "0.1.0",
        contact(
            name = "AeroXe Engineering",
            email = "engineering@aeroxe.com"
        ),
        license(
            name = "Proprietary",
            url = "https://aeroxe.com/license"
        )
    ),
    paths(
        // Auth
        auth::register,
        auth::login,
        auth::login_2fa,
        auth::refresh_token,
        auth::setup_2fa,
        auth::confirm_2fa,
        auth::verify_2fa,
        auth::verify_backup_code,
        auth::disable_2fa,
        // Customers
        customers::list_customers,
        customers::create_customer,
        customers::get_customer,
        customers::update_customer_status,
        customers::delete_customer,
        // Subscriptions
        subscriptions::list_subscriptions,
        subscriptions::create_subscription,
        subscriptions::cancel_subscription,
        subscriptions::suspend_subscription,
        subscriptions::reactivate_subscription,
        subscriptions::upgrade_subscription,
        subscriptions::downgrade_subscription,
        // Billing
        billing::list_invoices,
        billing::create_invoice,
        billing::list_payments,
        billing::record_payment,
        billing::list_overdue_invoices,
        billing::auto_generate_invoices,
        // Tickets
        tickets::list_tickets,
        tickets::create_ticket,
        tickets::get_ticket,
        tickets::assign_ticket,
        tickets::resolve_ticket,
        // Network
        network::list_vlans,
        network::create_vlan,
        network::delete_vlan,
        network::list_ip_pools,
        network::create_ip_pool,
        network::list_pppoe_sessions,
        network::create_pppoe_session,
        network::terminate_pppoe_session,
        // Devices
        devices::list_devices,
        devices::register_device,
        devices::get_device,
        devices::update_device_status,
    ),
    components(schemas(
        // Auth
        auth::RegisterRequest,
        auth::LoginRequest,
        auth::Login2FARequest,
        auth::AuthResponse,
        auth::RefreshTokenRequest,
        auth::Setup2FARequest,
        auth::Setup2FAResponse,
        auth::Confirm2FARequest,
        auth::Verify2FARequest,
        auth::VerifyBackupCodeRequest,
        // Customers
        customers::CustomerResponse,
        customers::CreateCustomerRequest,
        customers::UpdateStatusRequest,
        customers::AddressResponse,
        customers::AddAddressRequest,
        // Subscriptions
        subscriptions::SubscriptionResponse,
        subscriptions::CreateSubscriptionRequest,
        subscriptions::UpgradeSubscriptionRequest,
        subscriptions::DowngradeSubscriptionRequest,
        // Billing
        billing::InvoiceResponse,
        billing::CreateInvoiceRequest,
        billing::PaymentResponse,
        billing::RecordPaymentRequest,
        // Tickets
        tickets::TicketResponse,
        tickets::CreateTicketRequest,
        tickets::AssignTicketRequest,
        // Network
        network::VlanResponse,
        network::CreateVlanRequest,
        network::IpPoolResponse,
        network::CreateIpPoolRequest,
        network::PppoeSessionResponse,
        network::CreatePppoeRequest,
        // Devices
        devices::DeviceResponse,
        devices::RegisterDeviceRequest,
        // Common
        common::ErrorResponse,
    )),
    modifiers(&SecurityAddon),
    tags(
        (name = "Auth", description = "Authentication & 2FA"),
        (name = "Customers", description = "Customer lifecycle"),
        (name = "Subscriptions", description = "Subscription management"),
        (name = "Billing", description = "Invoices & payments"),
        (name = "Tickets", description = "Support tickets & SLA"),
        (name = "Network", description = "VLANs, IP pools, PPPoE"),
        (name = "Devices", description = "Network device management"),
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

        let security = SecurityScheme::Http(
            HttpBuilder::new()
                .scheme(HttpAuthScheme::Bearer)
                .bearer_format("JWT")
                .build(),
        );

        openapi
            .components
            .as_mut()
            .unwrap()
            .security_schemes
            .insert("bearer_auth".to_string(), security);
    }
}

/// Common API response types
pub mod common {
    use serde::Serialize;
    use utoipa::ToSchema;

    #[derive(Debug, Serialize, ToSchema)]
    pub struct ErrorResponse {
        /// Human-readable error message
        pub error: String,
        /// Machine-readable error code
        pub status: u16,
    }
}
