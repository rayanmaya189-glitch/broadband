//! OpenAPI documentation for AeroXe Broadband API.
//! Uses utoipa for automatic Swagger UI generation.
//!
//! Stub handler functions are used here solely for OpenAPI spec generation.
//! The actual request/response types mirror the real handler module types.

use utoipa::OpenApi;

pub mod accounting;
pub mod admin;
pub mod audit;
pub mod auth;
pub mod bandwidth;
pub mod billing;
pub mod branches;
pub mod compliance;
pub mod coverage;
pub mod customers;
pub mod devices;
pub mod discovery;
pub mod documents;
pub mod gateway;
pub mod installations;
pub mod inventory;
pub mod leads;
pub mod monitoring;
pub mod network;
pub mod notifications;
pub mod payments;
pub mod plans;
pub mod referrals;
pub mod scheduler;
pub mod security;
pub mod subscriptions;
pub mod tickets;
pub mod workflow;

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
        // Coverage
        coverage::list_coverage_areas,
        coverage::check_availability,
        coverage::create_coverage_area,
        coverage::get_coverage_stats,
        // Discovery
        discovery::list_scans,
        discovery::create_scan,
        discovery::list_results,
        discovery::approve_result,
        // Documents
        documents::list_documents,
        documents::confirm_upload,
        documents::presign_upload,
        documents::get_document,
        documents::get_download_url,
        documents::list_entity_documents,
        documents::delete_document,
        // Gateway
        gateway::list_rate_limit_rules,
        gateway::create_rate_limit_rule,
        gateway::delete_rate_limit_rule,
        gateway::list_api_keys,
        gateway::create_api_key,
        gateway::revoke_api_key,
        gateway::list_request_logs,
        gateway::get_request_stats,
        // Installations
        installations::list_installations,
        installations::create_installation,
        installations::get_installation,
        installations::reschedule_installation,
        installations::start_installation,
        installations::complete_installation,
        installations::cancel_installation,
        installations::add_installation_photo,
        installations::list_my_installation_assignments,
        installations::list_equipment,
        installations::add_equipment,
        installations::update_equipment_status,
        // Inventory
        inventory::list_inventory,
        inventory::create_inventory_item,
        inventory::assign_inventory_item,
        // Referrals
        referrals::list_referrals,
        referrals::create_referral,
        referrals::get_wallet,
        referrals::get_my_referral_code,
        referrals::list_my_referrals,
        referrals::share_referral,
        referrals::get_referral_stats,
        referrals::list_programs,
        referrals::create_program,
        referrals::update_program,
        referrals::delete_program,
        referrals::get_analytics,
        referrals::list_wallets,
        referrals::adjust_wallet,
        // Scheduler
        scheduler::list_jobs,
        scheduler::get_job,
        scheduler::create_job,
        scheduler::update_job,
        scheduler::delete_job,
        scheduler::trigger_job,
        scheduler::list_executions,
        scheduler::scheduler_stats,
        // Security
        security::list_roles,
        security::create_role,
        security::get_role,
        security::update_role,
        security::delete_role,
        security::list_permissions,
        security::assign_permission,
        security::revoke_permission,
        security::assign_role,
        security::revoke_role,
        // Workflow
        workflow::create_approval_request,
        workflow::list_pending_approvals,
        workflow::get_approval_request,
        workflow::approve_request,
        workflow::reject_request,
        // Accounting
        accounting::list_accounts,
        accounting::create_account,
        accounting::update_account,
        accounting::list_journal_entries,
        accounting::create_journal_entry,
        accounting::post_journal_entry,
        accounting::void_journal_entry,
        accounting::generate_trial_balance,
        accounting::profit_and_loss,
        accounting::balance_sheet,
        accounting::gst_return,
        accounting::reconcile_account,
        // Admin
        admin::seed_data,
        // Audit
        audit::search_history,
        audit::get_history_entry,
        audit::rollback_entity,
        audit::list_entity_types,
        audit::compare_history,
        audit::export_history,
        audit::search_audit_logs,
        audit::get_audit_log,
        audit::get_user_activity,
        audit::export_audit_logs,
        audit::list_events,
        audit::export_events,
        audit::replay_event,
        // Bandwidth
        bandwidth::list_profiles,
        bandwidth::create_profile,
        bandwidth::update_profile,
        bandwidth::delete_profile,
        bandwidth::get_profile,
        bandwidth::list_policies,
        bandwidth::create_policy,
        bandwidth::update_policy,
        bandwidth::delete_policy,
        bandwidth::apply_profile_to_all,
        bandwidth::apply_to_subscription,
        bandwidth::list_bandwidth_applications,
        bandwidth::get_bandwidth_usage,
        // Branches
        branches::list_branches,
        branches::create_branch,
        branches::get_branch,
        branches::update_branch,
        branches::delete_branch,
        branches::get_branch_hierarchy,
        branches::get_working_hours,
        branches::update_working_hours,
        branches::get_branch_stats,
        branches::assign_branch_user,
        branches::remove_branch_user,
        // Compliance
        compliance::list_compliance_items,
        compliance::create_compliance_item,
        compliance::update_compliance_item,
        compliance::get_compliance_report,
        compliance::list_audits,
        compliance::create_audit,
        compliance::update_audit,
        compliance::get_audit_report,
        compliance::list_violations,
        compliance::report_violation,
        compliance::resolve_violation,
        // Leads
        leads::list_leads,
        leads::create_lead,
        leads::get_lead,
        leads::update_lead,
        leads::update_lead_status,
        leads::assign_lead,
        leads::log_activity,
        leads::list_activities,
        leads::convert_lead,
        leads::get_pipeline,
        leads::get_stats,
        leads::list_sources,
        // Monitoring
        monitoring::list_metrics,
        monitoring::get_device_metrics,
        monitoring::list_alerts,
        monitoring::get_alert_stats,
        monitoring::create_alert,
        monitoring::acknowledge_alert,
        monitoring::resolve_alert,
        monitoring::get_health_summary,
        monitoring::list_incidents,
        // Notifications
        notifications::list_templates,
        notifications::create_template,
        notifications::update_template,
        notifications::delete_template,
        notifications::send_notification,
        notifications::list_notifications,
        notifications::retry_notification,
        notifications::retry_all_notifications,
        notifications::list_channels,
        notifications::update_channel,
        notifications::list_delivery_history,
        // Payments
        payments::create_payment_link,
        payments::record_manual_payment,
        payments::retry_payment,
        payments::handle_razorpay_webhook,
        payments::handle_payu_webhook,
        payments::list_gateways,
        payments::get_gateway_config,
        // Plans
        plans::list_plans,
        plans::get_plan,
        plans::create_plan,
        plans::update_plan,
        plans::delete_plan,
        plans::update_pricing,
        plans::list_plan_pricing,
        plans::approve_plan,
        plans::publish_plan,
        plans::unpublish_plan,
        plans::clone_plan,
        plans::get_speed_profile,
        plans::set_speed_profile,
        plans::get_plan_history,
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
        // Coverage
        coverage::CoverageAreaResponse,
        coverage::CreateCoverageAreaRequest,
        coverage::CheckPincodeRequest,
        coverage::AvailabilityResponse,
        coverage::CoverageStatsResponse,
        // Discovery
        discovery::ScanResponse,
        discovery::CreateScanRequest,
        discovery::ResultResponse,
        // Documents
        documents::DocumentResponse,
        documents::UploadRequest,
        documents::PresignUploadRequest,
        documents::PresignUploadResponse,
        documents::DownloadUrlResponse,
        // Gateway
        gateway::RateLimitRuleResponse,
        gateway::CreateRateLimitRuleRequest,
        gateway::ApiKeyResponse,
        gateway::CreateApiKeyRequest,
        gateway::RequestLogResponse,
        // Installations
        installations::InstallationResponse,
        installations::CreateOrderRequest,
        installations::ScheduleRequest,
        installations::RescheduleRequest,
        installations::PhotoResponse,
        installations::AddPhotoRequest,
        installations::EquipmentResponse,
        installations::AddEquipmentRequest,
        installations::UpdateEquipmentStatusRequest,
        // Inventory
        inventory::InventoryItemResponse,
        inventory::CreateInventoryItemRequest,
        inventory::UpdateStockRequest,
        inventory::AssignItemRequest,
        // Referrals
        referrals::CreateReferralRequest,
        referrals::ReferralResponse,
        referrals::WalletResponse,
        referrals::ReferralStatsResponse,
        referrals::CreateProgramRequest,
        referrals::UpdateProgramRequest,
        referrals::ReferralProgramResponse,
        referrals::ShareReferralRequest,
        referrals::AdjustWalletRequest,
        referrals::WalletTransactionResponse,
        referrals::ReferralAnalyticsResponse,
        // Scheduler
        scheduler::CreateJobRequest,
        scheduler::UpdateJobRequest,
        scheduler::JobDefinitionResponse,
        scheduler::JobExecutionResponse,
        scheduler::SchedulerStatsResponse,
        scheduler::ExecutionsQuery,
        // Security
        security::RoleResponse,
        security::PermissionResponse,
        security::CreateRoleRequest,
        security::UpdateRoleRequest,
        security::AssignPermissionRequest,
        security::AssignRoleRequest,
        security::RolePermissionsResponse,
        // Workflow
        workflow::CreateApprovalRequest,
        workflow::ReviewApprovalRequest,
        workflow::ApprovalRequestResponse,
        workflow::WorkflowResponse,
        // Accounting
        accounting::CreateAccountRequest,
        accounting::UpdateAccountRequest,
        accounting::CreateJournalEntryRequest,
        accounting::JournalLineRequest,
        accounting::AccountResponse,
        accounting::JournalEntryResponse,
        accounting::JournalLineResponse,
        accounting::TrialBalanceResponse,
        accounting::TrialBalanceLine,
        accounting::ProfitLossResponse,
        accounting::ProfitLossLine,
        accounting::BalanceSheetResponse,
        accounting::BalanceSheetLine,
        accounting::ReconciliationResponse,
        // Admin
        admin::SeedDataResponse,
        // Audit
        audit::HistoryEntryResponse,
        audit::HistoryComparisonResponse,
        audit::AuditLogResponse,
        audit::EventResponse,
        audit::EventReplayResult,
        audit::RollbackResult,
        // Bandwidth
        bandwidth::CreateProfileRequest,
        bandwidth::UpdateProfileRequest,
        bandwidth::CreatePolicyRequest,
        bandwidth::UpdatePolicyRequest,
        bandwidth::ApplyProfileRequest,
        bandwidth::BandwidthProfileResponse,
        bandwidth::BandwidthPolicyResponse,
        bandwidth::BandwidthApplicationResponse,
        // Branches
        branches::CreateBranchRequest,
        branches::UpdateBranchRequest,
        branches::WorkingHoursEntry,
        branches::AssignBranchUserRequest,
        branches::BranchResponse,
        branches::WorkingHoursResponse,
        branches::BranchUserResponse,
        branches::BranchStatsResponse,
        // Compliance
        compliance::CreateComplianceItemRequest,
        compliance::UpdateComplianceItemRequest,
        compliance::CreateAuditRequest,
        compliance::UpdateAuditRequest,
        compliance::ReportViolationRequest,
        compliance::ComplianceItemResponse,
        compliance::AuditResponse,
        compliance::ViolationResponse,
        compliance::ComplianceReport,
        compliance::KycResponse,
        compliance::ConsentResponse,
        compliance::RetentionPolicyResponse,
        // Leads
        leads::LeadResponse,
        leads::CreateLeadRequest,
        leads::UpdateLeadRequest,
        leads::UpdateLeadStatusRequest,
        leads::AssignLeadRequest,
        leads::LogActivityRequest,
        leads::LeadActivityResponse,
        leads::ConvertLeadRequest,
        leads::ConvertLeadResponse,
        leads::PipelineResponse,
        leads::LeadStatsResponse,
        leads::LeadSourceResponse,
        // Monitoring
        monitoring::MetricRecordResponse,
        monitoring::MetricsResponse,
        monitoring::AlertResponse,
        monitoring::AlertsResponse,
        monitoring::AlertStatsResponse,
        monitoring::CreateAlertRequest,
        monitoring::AcknowledgeAlertRequest,
        monitoring::ResolveAlertRequest,
        monitoring::HealthSummaryResponse,
        monitoring::IncidentResponse,
        // Notifications
        notifications::NotificationTemplateResponse,
        notifications::CreateTemplateRequest,
        notifications::UpdateTemplateRequest,
        notifications::SendNotificationRequest,
        notifications::NotificationResponse,
        notifications::NotificationChannelResponse,
        notifications::UpdateChannelRequest,
        notifications::DeliveryHistoryResponse,
        // Payments
        payments::PaymentLinkResponse,
        payments::GatewayConfigResponse,
        payments::CreatePaymentLinkRequest,
        payments::ManualPaymentRequest,
        payments::RetryPaymentRequest,
        payments::RazorpayOrderResponse,
        payments::PaymentVerificationResponse,
        payments::PaymentGatewayResponse,
        // Plans
        plans::PlanResponse,
        plans::PlanPricingResponse,
        plans::CreatePlanRequest,
        plans::UpdatePlanRequest,
        plans::UpdatePricingRequest,
        plans::ClonePlanRequest,
        plans::SetSpeedProfileRequest,
        plans::SpeedProfileResponse,
        plans::PlanHistoryEntry,
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
        (name = "Accounting", description = "Chart of accounts, journals, financial reports"),
        (name = "Admin", description = "System administration & seeding"),
        (name = "Audit", description = "Entity history, audit logs & event sourcing"),
        (name = "Bandwidth", description = "Bandwidth profiles, policies & usage"),
        (name = "Branches", description = "Branch management & hierarchy"),
        (name = "Compliance", description = "KYC, consent, data retention & audits"),
        (name = "Leads", description = "Sales leads, pipeline & conversion"),
        (name = "Monitoring", description = "Metrics, alerts, health & incidents"),
        (name = "Notifications", description = "Templates, channels & delivery"),
        (name = "Payments", description = "Payment links, gateways & webhooks"),
        (name = "Plans", description = "Broadband plans, pricing & speed profiles"),
        (name = "Referrals", description = "Referral programs, wallets & rewards"),
        (name = "Scheduler", description = "Scheduled jobs & execution history"),
        (name = "Security", description = "RBAC roles & permissions"),
        (name = "Workflow", description = "Approval workflows & reviews"),
        (name = "Coverage", description = "Service coverage areas & availability"),
        (name = "Discovery", description = "Network device discovery & scanning"),
        (name = "Documents", description = "Document upload, download & management"),
        (name = "Gateway", description = "Rate limiting, API keys & request logs"),
        (name = "Installations", description = "Installation orders, scheduling & equipment"),
        (name = "Inventory", description = "Inventory items & stock management"),
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
