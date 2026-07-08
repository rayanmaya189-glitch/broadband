use utoipa::OpenApi;

use crate::modules::accounting::request::accounting_request::*;
use crate::modules::accounting::response::accounting_response::*;
use crate::modules::audit::response::audit_response::*;
use crate::modules::bandwidth::request::bandwidth_request::*;
use crate::modules::bandwidth::response::bandwidth_response::*;
use crate::modules::billing::request::billing_request::*;
use crate::modules::billing::response::billing_response::*;
use crate::modules::branch::request::branch_request::*;
use crate::modules::branch::response::branch_response::*;
use crate::modules::coverage::request::coverage_request::*;
use crate::modules::coverage::response::coverage_response::*;
use crate::modules::customer::request::customer_request::*;
use crate::modules::customer::response::customer_response::*;
use crate::modules::device::request::device_request::*;
use crate::modules::device::response::device_response::*;
use crate::modules::discovery::request::discovery_request::*;
use crate::modules::discovery::response::discovery_response::*;
use crate::modules::document::request::document_request::*;
use crate::modules::document::response::document_response::*;
use crate::modules::event::response::event_response::*;
use crate::modules::installation::request::installation_request::*;
use crate::modules::installation::response::installation_response::*;
use crate::modules::inventory::request::inventory_request::*;
use crate::modules::inventory::response::inventory_response::*;
use crate::modules::lead::request::lead_request::*;
use crate::modules::lead::response::lead_response::*;
use crate::modules::network::request::network_request::*;
use crate::modules::network::response::network_response::*;
use crate::modules::notification::request::notification_request::*;
use crate::modules::notification::response::notification_response::*;
use crate::modules::payment_gateway::request::payment_gateway_request::*;
use crate::modules::payment_gateway::response::payment_gateway_response::*;
use crate::modules::permission::request::permission_request::*;
use crate::modules::permission::response::permission_response::*;
use crate::modules::plan::request::plan_request::*;
use crate::modules::plan::response::plan_response::*;
use crate::modules::realtime::response::realtime_response::*;
use crate::modules::referral::request::referral_request::*;
use crate::modules::referral::response::referral_response::*;
use crate::modules::role::request::role_request::*;
use crate::modules::role::response::role_response::*;
use crate::modules::subscription::request::subscription_request::*;
use crate::modules::subscription::response::subscription_response::*;
use crate::modules::ticket::request::ticket_request::*;
use crate::modules::ticket::response::ticket_response::*;
use crate::modules::user::request::user_request::*;
use crate::modules::user::response::user_response::*;

/// Central OpenAPI documentation for AeroXe Broadband ISP Platform
///
/// Schemas are registered here. Paths (endpoints) will be added once
/// handler functions are annotated with `#[utoipa::path]`.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "AeroXe Broadband ISP Platform API",
        version = "0.1.0",
        description = "Enterprise-grade ISP management platform backend API"
    ),
    components(schemas(
        // === Auth & User ===
        LoginRequest,
        RegisterRequest,
        CreateUserRequest,
        UpdateUserRequest,
        UpdateUserStatusRequest,
        UpdateProfileRequest,
        RefreshTokenRequest,
        ChangePasswordRequest,
        LogoutRequest,
        UserResponse,
        AuthUserResponse,
        LoginResponse,
        RegisterResponse,
        TokenRefreshResponse,
        SessionResponse,
        // === Roles & Permissions ===
        CreateRoleRequest,
        UpdateRoleRequest,
        AssignPermissionsRequest,
        RoleResponse,
        CreatePermissionRequest,
        PermissionResponse,
        // === Branches ===
        CreateBranchRequest,
        UpdateBranchRequest,
        BranchResponse,
        // === Customers ===
        CreateCustomerRequest,
        UpdateCustomerRequest,
        CustomerResponse,
        // === Plans ===
        CreatePlanRequest,
        UpdatePlanRequest,
        PlanResponse,
        // === Subscriptions ===
        CreateSubscriptionRequest,
        SubscriptionActionRequest,
        SubscriptionResponse,
        // === Tickets ===
        CreateTicketRequest,
        UpdateTicketRequest,
        AssignTicketRequest,
        EscalateTicketRequest,
        ResolveTicketRequest,
        CloseTicketRequest,
        ReopenTicketRequest,
        AddCommentRequest,
        TicketFeedbackRequest,
        TicketResponse,
        TicketCommentResponse,
        TicketDashboardResponse,
        // === Leads ===
        CreateLeadRequest,
        UpdateLeadRequest,
        LeadStatusRequest,
        AssignLeadRequest,
        AddActivityRequest,
        ConvertLeadRequest,
        LeadResponse,
        LeadActivityResponse,
        LeadPipelineResponse,
        LeadStatsResponse,
        // === Billing ===
        CreateInvoiceRequest,
        CreateLineItemRequest,
        RecordPaymentRequest,
        CreateRefundRequest,
        CreateDiscountRequest,
        InvoiceResponse,
        InvoiceLineItemResponse,
        InvoiceListResponse,
        PaymentResponse,
        PaymentListResponse,
        RefundResponse,
        DiscountResponse,
        // === Devices ===
        CreateDeviceRequest,
        UpdateDeviceRequest,
        CreateDeviceModelRequest,
        DeviceResponse,
        DeviceListResponse,
        DeviceModelResponse,
        // === Bandwidth ===
        CreateBandwidthProfileRequest,
        UpdateBandwidthProfileRequest,
        BandwidthProfileResponse,
        BandwidthProfileListResponse,
        // === Network ===
        CreateVlanRequest,
        CreateIpPoolRequest,
        CreatePppoeSessionRequest,
        VlanResponse,
        IpPoolResponse,
        PppoeSessionResponse,
        // === Coverage ===
        CreateCoverageAreaRequest,
        CheckAvailabilityRequest,
        CoverageAreaResponse,
        AvailabilityCheckResponse,
        // === Installations ===
        CreateInstallationRequest,
        ScheduleInstallationRequest,
        CompleteInstallationRequest,
        InstallationResponse,
        InstallationListResponse,
        // === Inventory ===
        CreateInventoryItemRequest,
        InventoryItemResponse,
        InventoryListResponse,
        // === Referrals ===
        CreateReferralProgramRequest,
        ReferralProgramResponse,
        ReferralTrackingResponse,
        // === Notifications ===
        CreateTemplateRequest,
        SendNotificationRequest,
        TemplateResponse,
        NotificationResponse,
        // === Events ===
        EventResponse,
        EventListResponse,
        // === Documents ===
        UploadRequest,
        DocumentResponse,
        UploadResponse,
        // === Accounting ===
        CreateAccountRequest,
        CreateJournalEntryRequest,
        JournalLineRequest,
        AccountingQuery,
        AccountResponse,
        JournalEntryResponse,
        // === Payment Gateway ===
        CreateGatewayConfigRequest,
        CreatePaymentLinkRequest,
        GatewayConfigResponse,
        PaymentLinkResponse,
        // === Discovery ===
        CreateScanRequest,
        ScanResponse,
        ResultResponse,
        // === Realtime / WebSocket ===
        HealthResponse,
        ChannelInfo,
        ConnectionStats,
        WsMessageResponse,
        // === Audit ===
        AuditLogResponse,
        AuditListResponse,
    )),
    tags(
        (name = "Auth", description = "Authentication & Authorization"),
        (name = "Users", description = "User management"),
        (name = "Roles", description = "Role management"),
        (name = "Branches", description = "Branch/office management"),
        (name = "Customers", description = "Customer management"),
        (name = "Plans", description = "Service plan management"),
        (name = "Subscriptions", description = "Customer subscription management"),
        (name = "Tickets", description = "Support ticket management"),
        (name = "Leads", description = "Sales lead pipeline"),
        (name = "Billing", description = "Invoices, payments, refunds, discounts"),
        (name = "Devices", description = "CPE device management"),
        (name = "Bandwidth", description = "Bandwidth profile management"),
        (name = "Network", description = "VLAN, IP pool, PPPoE session management"),
        (name = "Coverage", description = "Service coverage areas"),
        (name = "Installations", description = "Installation scheduling & tracking"),
        (name = "Inventory", description = "Inventory item management"),
        (name = "Referrals", description = "Referral program management"),
        (name = "Notifications", description = "Notification templates & delivery"),
        (name = "Events", description = "System event log"),
        (name = "Documents", description = "Document upload & management"),
        (name = "Accounting", description = "Chart of accounts & journal entries"),
        (name = "Payment Gateway", description = "Payment gateway configuration"),
        (name = "Discovery", description = "Network device discovery"),
        (name = "Realtime", description = "WebSocket health & channel info"),
        (name = "Audit", description = "Audit trail logs"),
    )
)]
pub struct ApiDoc;
