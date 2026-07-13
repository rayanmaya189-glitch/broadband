use utoipa::{Modify, OpenApi, openapi::security::{Http, HttpAuthScheme, SecurityScheme}};

use crate::modules::accounting::controller::accounting_controller;
use crate::modules::accounting::request::accounting_request::*;
use crate::modules::accounting::response::accounting_response::{
    AccountResponse, BalanceSheetResponse, CashFlowResponse, GstReturnResponse,
    JournalEntryDetailResponse, JournalEntryLineResponse, JournalEntryResponse,
    ProfitLossResponse, TrialBalanceResponse,
};
use crate::modules::audit::controller::audit_controller;
use crate::modules::audit::controller::entity_history_controller;
use crate::modules::audit::request::audit_request::*;
use crate::modules::audit::request::entity_history_request::*;
use crate::modules::audit::response::audit_response::{
    AuditListResponse, AuditLogResponse, AuditStatsResponse,
};
use crate::modules::audit::response::entity_history_response::{
    EntityHistoryListResponse, EntityHistoryResponse, EntityHistoryStatsResponse,
};
use crate::modules::bandwidth::controller::bandwidth_controller;
use crate::modules::bandwidth::request::bandwidth_request::*;
use crate::modules::bandwidth::response::bandwidth_response::{
    BandwidthApplicationResponse, BandwidthProfileListResponse, BandwidthProfileResponse,
    BandwidthUsageRecord, BandwidthUsageResponse,
};
use crate::modules::billing::controller::billing_controller;
use crate::modules::billing::request::billing_request::*;
use crate::modules::billing::response::billing_response::{
    DiscountResponse, InvoiceLineItemResponse, InvoiceListResponse, InvoiceResponse,
    PaymentListResponse, PaymentResponse, RefundResponse,
};
use crate::modules::branch::controller::branch_controller;
use crate::modules::branch::request::branch_request::*;
use crate::modules::branch::response::branch_response::{
    BranchResponse, BranchStatsResponse, BranchUserResponse, WorkingHourResponse,
};
use crate::modules::coverage::controller::coverage_controller;
use crate::modules::coverage::request::coverage_request::*;
use crate::modules::coverage::response::coverage_response::{
    AvailabilityCheckResponse, CoverageAreaResponse, CoveragePincodeResponse,
    CoverageStatsResponse,
};
use crate::modules::customer::controller::customer_controller;
use crate::modules::customer::request::customer_request::*;
use crate::modules::customer::response::customer_response::{
    AddressResponse, CustomerFullResponse, CustomerProfileResponse, CustomerResponse,
    KycDocumentResponse,
};
use crate::modules::device::controller::device_controller;
use crate::modules::device::request::device_request::*;
use crate::modules::device::response::device_response::{
    DeviceListResponse, DeviceLogListResponse, DeviceLogResponse, DeviceMetricResponse,
    DeviceModelResponse, DevicePortResponse, DeviceResponse, FirmwareUpdateResponse,
};
use crate::modules::discovery::controller::discovery_controller;
use crate::modules::discovery::request::discovery_request::*;
use crate::modules::discovery::response::discovery_response::{
    DashboardResponse, ResultResponse, ScanResponse, VendorCount,
};
use crate::modules::document::controller::document_controller;
use crate::modules::document::request::document_request::*;
use crate::modules::document::response::document_response::{
    DocumentAccessLogResponse, DocumentResponse, UploadResponse,
};
use crate::modules::event::controller::event_controller;
use crate::modules::event::request::event_request::{EventQuery, PublishEventRequest, CreateSubscriptionRequest as EventCreateSubscriptionRequest};
use crate::modules::event::response::event_response::{
    EventListResponse, EventResponse, EventStatsResponse, EventSubscriptionResponse,
};
use crate::modules::installation::controller::installation_controller;
use crate::modules::installation::request::installation_request::*;
use crate::modules::installation::response::installation_response::{
    InstallationListResponse, InstallationResponse,
};
use crate::modules::inventory::controller::inventory_controller;
use crate::modules::inventory::request::inventory_request::*;
use crate::modules::inventory::response::inventory_response::{
    InventoryItemResponse, InventoryListResponse, InventoryMovementResponse,
    InventoryReportResponse, WarrantyAlertResponse,
};
use crate::modules::lead::controller::lead_controller;
use crate::modules::lead::request::lead_request::*;
use crate::modules::lead::response::lead_response::{
    LeadActivityResponse, LeadListResponse, LeadPipelineResponse, LeadResponse, LeadStatsResponse,
    SourceCount,
};
use crate::modules::network::controller::network_controller;
use crate::modules::network::request::network_request::*;
use crate::modules::network::response::network_response::{
    CustomerSessionResponse, DhcpLeaseResponse, IpAddressResponse, IpPoolResponse,
    MacBindingResponse, NetworkTopologyResponse, PppoeSessionResponse, VlanResponse,
    PaginatedResponse as NetworkPaginatedResponse,
};
use crate::modules::notification::controller::notification_controller;
use crate::modules::notification::request::notification_request::*;
use crate::modules::notification::response::notification_response::{
    ChannelResponse, HistoryListResponse, HistoryResponse, NotificationDetailResponse,
    NotificationListResponse, NotificationResponse, TemplateResponse,
};
use crate::modules::payment_gateway::controller::payment_gateway_controller;
use crate::modules::payment_gateway::request::payment_gateway_request::*;
use crate::modules::payment_gateway::response::payment_gateway_response::{
    GatewayConfigResponse, PaymentLinkResponse, TransactionListResponse,
    WebhookProcessResponse,
};
use crate::modules::permission::controller::permission_controller;
use crate::modules::permission::request::permission_request::*;
use crate::modules::permission::response::permission_response::PermissionResponse;
use crate::modules::plan::controller::plan_controller;
use crate::modules::plan::request::plan_request::*;
use crate::modules::plan::response::plan_response::{
    PlanCloneResponse, PlanPricingResponse, PlanResponse, SpeedProfileResponse,
};
use crate::modules::realtime::controller::realtime_controller;
use crate::modules::realtime::request::realtime_request::*;
use crate::modules::realtime::response::realtime_response::{
    ChannelInfo, ConnectionStats, HealthResponse, WsMessageResponse,
};
use crate::modules::referral::controller::referral_controller;
use crate::modules::referral::request::referral_request::*;
use crate::modules::referral::response::referral_response::{
    ReferralProgramResponse, ReferralStatsResponse, ReferralTrackingListResponse,
    ReferralTrackingResponse, StatusCount as ReferralStatusCount, WalletResponse,
    WalletTransactionListResponse, WalletTransactionResponse,
};
use crate::modules::role::controller::role_controller;
use crate::modules::role::request::role_request::*;
use crate::modules::role::response::role_response::RoleResponse;
use crate::modules::subscription::controller::subscription_controller;
use crate::modules::subscription::request::subscription_request::*;
use crate::modules::subscription::response::subscription_response::{
    SubscriptionHistoryEntry, SubscriptionResponse, UpgradeDowngradeResponse,
};
use crate::modules::ticket::controller::ticket_controller;
use crate::modules::ticket::request::ticket_request::*;
use crate::modules::ticket::response::ticket_response::{
    MessageResponse, TicketCommentResponse, TicketDashboardResponse, TicketEscalationResponse,
    TicketListResponse, TicketResponse, TicketStatusHistoryResponse,
};
use crate::modules::user::controller::user_controller;
use crate::modules::user::request::user_request::*;
use crate::modules::user::response::user_response::{
    AuthUserResponse, LoginResponse, OtpSentResponse, PasswordResetResponse, RegisterResponse,
    Requires2FaResponse, SessionResponse, TokenRefreshResponse, TwoFaEnabledResponse,
    TwoFaSetupResponse, UserResponse,
};
use crate::common::utils::helpers::PaginatedResponse;

/// Central OpenAPI documentation for AeroXe Broadband ISP Platform
///
/// All endpoints are documented with `#[utoipa::path]` annotations in their
/// respective controller modules.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "AeroXe Broadband ISP Platform API",
        version = "0.1.0",
        description = "Enterprise-grade ISP management platform backend API"
    ),
    paths(
        // Auth
        user_controller::login,
        user_controller::register,
        user_controller::refresh_token,
        user_controller::logout,
        user_controller::logout_all,
        user_controller::change_password,
        user_controller::me,
        user_controller::list_sessions,
        user_controller::send_otp,
        user_controller::verify_otp,
        user_controller::request_password_reset,
        user_controller::confirm_password_reset,
        user_controller::enable_2fa,
        user_controller::confirm_2fa,
        user_controller::verify_2fa_login,
        user_controller::disable_2fa,
        // Users
        user_controller::list_users,
        user_controller::create_user,
        user_controller::get_user,
        user_controller::update_user,
        user_controller::delete_user,
        user_controller::update_user_status,
        user_controller::get_me,
        user_controller::update_me,
        // Roles
        role_controller::list_roles,
        role_controller::create_role,
        role_controller::get_role,
        role_controller::update_role,
        role_controller::deactivate_role,
        role_controller::assign_permissions,
        role_controller::remove_permission,
        role_controller::list_user_roles,
        role_controller::assign_role_to_user,
        role_controller::revoke_role_from_user,
        // Permissions
        permission_controller::list_permissions,
        permission_controller::create_permission,
        permission_controller::delete_permission,
        // Branches
        branch_controller::list_branches,
        branch_controller::create_branch,
        branch_controller::get_branch,
        branch_controller::update_branch,
        branch_controller::deactivate_branch,
        branch_controller::get_working_hours,
        branch_controller::update_working_hours,
        branch_controller::assign_user,
        branch_controller::remove_user,
        branch_controller::list_branch_users,
        branch_controller::get_branch_stats,
        // Customers
        customer_controller::list_customers,
        customer_controller::create_customer,
        customer_controller::get_customer,
        customer_controller::update_customer,
        customer_controller::update_status,
        customer_controller::delete_customer,
        customer_controller::get_profile,
        customer_controller::update_profile,
        customer_controller::submit_kyc,
        customer_controller::verify_kyc,
        customer_controller::list_kyc_documents,
        customer_controller::upload_kyc_document,
        customer_controller::delete_kyc_document,
        customer_controller::list_addresses,
        customer_controller::create_address,
        customer_controller::update_address,
        customer_controller::delete_address,
        // Plans
        plan_controller::list_plans,
        plan_controller::create_plan,
        plan_controller::get_plan,
        plan_controller::update_plan,
        plan_controller::delete_plan,
        plan_controller::publish_plan,
        plan_controller::unpublish_plan,
        plan_controller::clone_plan,
        plan_controller::get_speed_profile,
        plan_controller::create_speed_profile,
        plan_controller::delete_speed_profile,
        plan_controller::list_plan_pricing,
        plan_controller::update_plan_pricing,
        // Subscriptions
        subscription_controller::list_subscriptions,
        subscription_controller::create_subscription,
        subscription_controller::get_subscription,
        subscription_controller::suspend_subscription,
        subscription_controller::reactivate_subscription,
        subscription_controller::cancel_subscription,
        subscription_controller::upgrade_subscription,
        subscription_controller::downgrade_subscription,
        subscription_controller::get_subscription_history,
        // Tickets
        ticket_controller::list_tickets,
        ticket_controller::create_ticket,
        ticket_controller::get_ticket,
        ticket_controller::update_ticket,
        ticket_controller::delete_ticket,
        ticket_controller::assign_ticket,
        ticket_controller::escalate_ticket,
        ticket_controller::resolve_ticket,
        ticket_controller::close_ticket,
        ticket_controller::reopen_ticket,
        ticket_controller::set_feedback,
        ticket_controller::get_comments,
        ticket_controller::add_comment,
        ticket_controller::get_escalations,
        ticket_controller::get_status_history,
        ticket_controller::get_dashboard,
        ticket_controller::get_my_assignments,
        // Leads
        lead_controller::list_leads,
        lead_controller::create_lead,
        lead_controller::get_lead,
        lead_controller::update_lead,
        lead_controller::update_status,
        lead_controller::assign_lead,
        lead_controller::add_activity,
        lead_controller::get_activities,
        lead_controller::convert_lead,
        lead_controller::delete_lead,
        lead_controller::get_pipeline,
        lead_controller::get_stats,
        // Billing
        billing_controller::list_invoices,
        billing_controller::get_invoice,
        billing_controller::create_invoice,
        billing_controller::send_invoice,
        billing_controller::void_invoice,
        billing_controller::get_line_items,
        billing_controller::review_invoice,
        billing_controller::record_payment,
        billing_controller::list_payments,
        billing_controller::request_refund,
        billing_controller::approve_refund,
        billing_controller::list_discounts,
        billing_controller::create_discount,
        billing_controller::get_dunning_config,
        billing_controller::update_dunning_config,
        billing_controller::get_tax_config,
        billing_controller::update_tax_config,
        // Devices
        device_controller::list_devices,
        device_controller::get_device,
        device_controller::create_device,
        device_controller::update_device,
        device_controller::delete_device,
        device_controller::list_models,
        device_controller::create_model,
        device_controller::list_ports,
        device_controller::update_port_status,
        device_controller::restart_device,
        device_controller::shutdown_device,
        device_controller::list_firmware_updates,
        device_controller::create_firmware_update,
        device_controller::update_firmware_status,
        device_controller::get_device_metrics,
        device_controller::list_device_logs,
        device_controller::create_device_log,
        // Bandwidth
        bandwidth_controller::list_profiles,
        bandwidth_controller::get_profile,
        bandwidth_controller::create_profile,
        bandwidth_controller::update_profile,
        bandwidth_controller::delete_profile,
        bandwidth_controller::apply_to_subscription,
        bandwidth_controller::list_applications,
        bandwidth_controller::get_usage,
        // Network
        network_controller::list_vlans,
        network_controller::create_vlan,
        network_controller::update_vlan,
        network_controller::delete_vlan,
        network_controller::list_ip_pools,
        network_controller::create_ip_pool,
        network_controller::list_ip_addresses,
        network_controller::allocate_ip,
        network_controller::release_ip,
        network_controller::list_pppoe_sessions,
        network_controller::create_pppoe_session,
        network_controller::terminate_session,
        network_controller::list_mac_bindings,
        network_controller::create_mac_binding,
        network_controller::delete_mac_binding,
        network_controller::list_dhcp_leases,
        network_controller::list_customer_sessions,
        network_controller::get_topology,
        // Coverage
        coverage_controller::list_areas,
        coverage_controller::get_area,
        coverage_controller::create_area,
        coverage_controller::update_area,
        coverage_controller::check_availability,
        coverage_controller::delete_area,
        coverage_controller::list_pincodes,
        coverage_controller::add_pincode,
        coverage_controller::remove_pincode,
        coverage_controller::get_stats,
        // Installations
        installation_controller::list_installations,
        installation_controller::get_installation,
        installation_controller::create_installation,
        installation_controller::schedule_installation,
        installation_controller::reschedule_installation,
        installation_controller::start_installation,
        installation_controller::complete_installation,
        installation_controller::cancel_installation,
        installation_controller::upload_photo,
        installation_controller::get_my_assignments,
        // Inventory
        inventory_controller::list_items,
        inventory_controller::get_item,
        inventory_controller::create_item,
        inventory_controller::assign_item,
        inventory_controller::install_item,
        inventory_controller::return_item,
        inventory_controller::transfer_item,
        inventory_controller::scrap_item,
        inventory_controller::delete_item,
        inventory_controller::list_movements,
        inventory_controller::get_report,
        inventory_controller::get_warranty_alerts,
        // Referrals
        referral_controller::list_programs,
        referral_controller::create_program,
        referral_controller::update_program,
        referral_controller::share_referral,
        referral_controller::list_tracking,
        referral_controller::get_stats,
        referral_controller::get_wallet,
        referral_controller::get_or_create_wallet,
        referral_controller::credit_wallet,
        referral_controller::debit_wallet,
        referral_controller::list_wallet_transactions,
        // Notifications
        notification_controller::list_templates,
        notification_controller::create_template,
        notification_controller::update_template,
        notification_controller::delete_template,
        notification_controller::list_channels,
        notification_controller::upsert_channel,
        notification_controller::send_notification,
        notification_controller::list_notifications,
        notification_controller::retry_notification,
        notification_controller::list_history,
        // Events
        event_controller::list_events,
        event_controller::get_event,
        event_controller::get_aggregate_events,
        event_controller::publish_event,
        event_controller::mark_processed,
        event_controller::list_subscriptions,
        event_controller::create_subscription,
        event_controller::delete_subscription,
        event_controller::get_stats,
        // Documents
        document_controller::list_documents,
        document_controller::get_document,
        document_controller::upload_url,
        document_controller::confirm_upload,
        document_controller::associate_entity,
        document_controller::delete_document,
        document_controller::get_access_logs,
        // Accounting
        accounting_controller::list_accounts,
        accounting_controller::create_account,
        accounting_controller::list_journal,
        accounting_controller::create_journal,
        accounting_controller::get_entry_lines,
        accounting_controller::post_journal,
        accounting_controller::void_journal,
        accounting_controller::trial_balance,
        accounting_controller::profit_loss,
        accounting_controller::balance_sheet,
        accounting_controller::cash_flow,
        accounting_controller::gst_return_data,
        // Payment Gateway
        payment_gateway_controller::list_gateways,
        payment_gateway_controller::create_gateway,
        payment_gateway_controller::update_gateway,
        payment_gateway_controller::create_payment_link,
        payment_gateway_controller::list_transactions,
        payment_gateway_controller::retry_payment,
        // Discovery
        discovery_controller::list_scans,
        discovery_controller::create_scan,
        discovery_controller::start_scan,
        discovery_controller::stop_scan,
        discovery_controller::list_results,
        discovery_controller::approve_result,
        discovery_controller::reject_result,
        discovery_controller::dashboard,
        // Realtime
        realtime_controller::health,
        realtime_controller::channels,
        realtime_controller::stats,
        // Audit
        audit_controller::list_logs,
        audit_controller::get_log,
        audit_controller::get_user_activity,
        audit_controller::get_resource_history,
        audit_controller::export_logs,
        // Entity History
        entity_history_controller::search_history,
        entity_history_controller::get_history_entry,
        entity_history_controller::get_entity_history,
        entity_history_controller::rollback,
        entity_history_controller::get_stats,
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
        ListUsersQuery,
        SendOtpRequest,
        VerifyOtpRequest,
        PasswordResetRequest,
        PasswordResetConfirmRequest,
        Verify2FaRequest,
        Enable2FaRequest,
        Confirm2FaRequest,
        UserResponse,
        AuthUserResponse,
        LoginResponse,
        RegisterResponse,
        TokenRefreshResponse,
        SessionResponse,
        OtpSentResponse,
        PasswordResetResponse,
        Requires2FaResponse,
        TwoFaSetupResponse,
        TwoFaEnabledResponse,
        // === Roles & Permissions ===
        CreateRoleRequest,
        UpdateRoleRequest,
        AssignPermissionsRequest,
        AssignUserRoleRequest,
        ListRolesQuery,
        RoleResponse,
        CreatePermissionRequest,
        ListPermissionsQuery,
        PermissionResponse,
        // === Branches ===
        CreateBranchRequest,
        UpdateBranchRequest,
        UpdateWorkingHoursRequest,
        AssignUserToBranchRequest,
        ListBranchesQuery,
        BranchResponse,
        WorkingHourResponse,
        BranchUserResponse,
        BranchStatsResponse,
        // === Customers ===
        CreateCustomerRequest,
        UpdateCustomerRequest,
        CustomerStatusTransition,
        UpdateCustomerProfileRequest,
        SubmitKycRequest,
        VerifyKycRequest,
        CreateAddressRequest,
        UpdateAddressRequest,
        ListCustomersQuery,
        ListKycDocumentsQuery,
        CustomerResponse,
        CustomerProfileResponse,
        KycDocumentResponse,
        AddressResponse,
        CustomerFullResponse,
        // === Plans ===
        CreatePlanRequest,
        UpdatePlanRequest,
        CreateSpeedProfileRequest,
        PublishPlanRequest,
        UpdatePlanPricingRequest,
        ListPlansQuery,
        ListPlanPricingQuery,
        PlanResponse,
        SpeedProfileResponse,
        PlanPricingResponse,
        PlanCloneResponse,
        // === Subscriptions ===
        CreateSubscriptionRequest,
        SubscriptionActionRequest,
        UpgradeDowngradeRequest,
        ListSubscriptionsQuery,
        SubscriptionHistoryQuery,
        SubscriptionResponse,
        UpgradeDowngradeResponse,
        SubscriptionHistoryEntry,
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
        TicketQuery,
        TicketResponse,
        TicketListResponse,
        TicketCommentResponse,
        TicketDashboardResponse,
        TicketEscalationResponse,
        TicketStatusHistoryResponse,
        MessageResponse,
        // === Leads ===
        CreateLeadRequest,
        UpdateLeadRequest,
        LeadStatusRequest,
        AssignLeadRequest,
        AddActivityRequest,
        ConvertLeadRequest,
        LeadQuery,
        LeadResponse,
        LeadListResponse,
        LeadActivityResponse,
        LeadPipelineResponse,
        LeadStatsResponse,
        SourceCount,
        // === Billing ===
        CreateInvoiceRequest,
        CreateLineItemRequest,
        RecordPaymentRequest,
        CreateRefundRequest,
        CreateDiscountRequest,
        ReviewInvoiceRequest,
        BillingConfigRequest,
        InvoiceQuery,
        PaymentQuery,
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
        PortStatusRequest,
        FirmwareUpdateRequest,
        FirmwareStatusRequest,
        DeviceQuery,
        DeviceLogQuery,
        DeviceResponse,
        DeviceListResponse,
        DeviceModelResponse,
        DevicePortResponse,
        FirmwareUpdateResponse,
        DeviceMetricResponse,
        DeviceLogResponse,
        DeviceLogListResponse,
        // === Bandwidth ===
        CreateBandwidthProfileRequest,
        UpdateBandwidthProfileRequest,
        ApplyProfileRequest,
        ApplicationQuery,
        UsageQuery,
        BandwidthProfileResponse,
        BandwidthProfileListResponse,
        BandwidthApplicationResponse,
        BandwidthUsageResponse,
        BandwidthUsageRecord,
        // === Network ===
        CreateVlanRequest,
        UpdateVlanRequest,
        CreateIpPoolRequest,
        CreatePppoeSessionRequest,
        AllocateIpRequest,
        CreateMacBindingRequest,
        ReleaseIpRequest,
        NetworkQuery,
        IpPoolQuery,
        VlanResponse,
        IpPoolResponse,
        IpAddressResponse,
        PppoeSessionResponse,
        MacBindingResponse,
        DhcpLeaseResponse,
        CustomerSessionResponse,
        NetworkTopologyResponse,
        // === Coverage ===
        CreateCoverageAreaRequest,
        UpdateCoverageAreaRequest,
        CheckAvailabilityRequest,
        AddPincodeRequest,
        CoverageQuery,
        CoverageAreaResponse,
        AvailabilityCheckResponse,
        CoveragePincodeResponse,
        CoverageStatsResponse,
        // === Installations ===
        CreateInstallationRequest,
        ScheduleInstallationRequest,
        RescheduleInstallationRequest,
        CompleteInstallationRequest,
        UploadPhotoRequest,
        InstallationQuery,
        InstallationResponse,
        InstallationListResponse,
        // === Inventory ===
        CreateInventoryItemRequest,
        AssignInventoryRequest,
        TransferInventoryRequest,
        InventoryQuery,
        InventoryItemResponse,
        InventoryListResponse,
        InventoryMovementResponse,
        InventoryReportResponse,
        WarrantyAlertResponse,
        // === Referrals ===
        CreateReferralProgramRequest,
        UpdateReferralProgramRequest,
        ShareReferralRequest,
        WalletCreditRequest,
        WalletDebitRequest,
        TrackingQuery,
        ReferralProgramResponse,
        ReferralTrackingResponse,
        ReferralTrackingListResponse,
        ReferralStatsResponse,
        ReferralStatusCount,
        WalletResponse,
        WalletTransactionResponse,
        WalletTransactionListResponse,
        // === Notifications ===
        CreateTemplateRequest,
        UpdateTemplateRequest,
        SendNotificationRequest,
        UpsertChannelRequest,
        NotificationQuery,
        HistoryQuery,
        TemplateResponse,
        NotificationResponse,
        NotificationDetailResponse,
        NotificationListResponse,
        ChannelResponse,
        HistoryResponse,
        HistoryListResponse,
        // === Events ===
        PublishEventRequest,
        EventCreateSubscriptionRequest,
        EventQuery,
        EventResponse,
        EventListResponse,
        EventSubscriptionResponse,
        EventStatsResponse,
        // === Documents ===
        UploadRequest,
        ConfirmUploadRequest,
        AssociateEntityRequest,
        DocumentQuery,
        DocumentResponse,
        UploadResponse,
        DocumentAccessLogResponse,
        // === Accounting ===
        CreateAccountRequest,
        CreateJournalEntryRequest,
        JournalLineRequest,
        AccountingQuery,
        TrialBalanceQuery,
        GstQuery,
        AccountResponse,
        JournalEntryResponse,
        JournalEntryLineResponse,
        JournalEntryDetailResponse,
        TrialBalanceResponse,
        ProfitLossResponse,
        BalanceSheetResponse,
        CashFlowResponse,
        GstReturnResponse,
        // === Payment Gateway ===
        CreateGatewayConfigRequest,
        UpdateGatewayRequest,
        CreatePaymentLinkRequest,
        RetryPaymentRequest,
        WebhookPayload,
        TransactionQuery,
        GatewayConfigResponse,
        PaymentLinkResponse,
        TransactionListResponse,
        WebhookProcessResponse,
        // === Discovery ===
        CreateScanRequest,
        RejectRequest,
        DiscoveryQuery,
        ScanResponse,
        ResultResponse,
        DashboardResponse,
        VendorCount,
        // === Realtime / WebSocket ===
        ChannelQuery,
        HealthResponse,
        ChannelInfo,
        ConnectionStats,
        WsMessageResponse,
        // === Audit ===
        AuditQuery,
        ExportAuditRequest,
        EntityHistoryQuery,
        RollbackRequest,
        AuditLogResponse,
        AuditListResponse,
        AuditStatsResponse,
        EntityHistoryResponse,
        EntityHistoryListResponse,
        EntityHistoryStatsResponse,
        // === Paginated Responses ===
        PaginatedResponse<UserResponse>,
        PaginatedResponse<BranchResponse>,
        PaginatedResponse<CustomerResponse>,
        PaginatedResponse<PlanResponse>,
        PaginatedResponse<SubscriptionResponse>,
        PaginatedResponse<RoleResponse>,
        PaginatedResponse<PermissionResponse>,
        NetworkPaginatedResponse<PppoeSessionResponse>,
        NetworkPaginatedResponse<MacBindingResponse>,
        NetworkPaginatedResponse<DhcpLeaseResponse>,
        NetworkPaginatedResponse<CustomerSessionResponse>,
    )),
    modifiers(&SecurityAddon),
    tags(
        (name = "Auth", description = "Authentication & Authorization"),
        (name = "Users", description = "User management"),
        (name = "Roles", description = "Role management"),
        (name = "Permissions", description = "Permission management"),
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

/// Adds the Bearer Auth (JWT) security scheme to the OpenAPI spec.
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                Http::new(HttpAuthScheme::Bearer)
            ),
        );
    }
}
