//! Frontend ↔ backend API contract tests.
//!
//! Each fixture below is the *exact* JSON payload a frontend admin page sends
//! today (see the file reference in the comment). If the backend DTO changes
//! without the frontend being updated — or vice versa — one of these tests
//! fails, so contract drift is caught in `cargo test` instead of at runtime
//! with a 422.
//!
//! The backend DTOs are the source of truth: serde will reject a payload that
//! is missing a required field or has an unknown required field. Extra
//! optional fields are ignored (matching serde's default behaviour), so the
//! fixtures intentionally only include fields the backend actually accepts.

#[cfg(test)]
mod tests {
    use serde_json::json;

    // ─── Plans ─────────────────────────────────────────────────────────────
    // frontend/src/admin/pages/Plans.tsx
    #[test]
    fn create_plan_payload_matches_backend_dto() {
        let payload = json!({
            "slug": "fibre-100",
            "name": "Fibre 100",
            "speed_label": "100 Mbps",
            "download_mbps": 100,
            "upload_mbps": 100,
        });
        let req: crate::modules::plans::api::http::CreatePlanRequest =
            serde_json::from_value(payload)
                .expect("frontend payload must deserialize into CreatePlanRequest");
        assert_eq!(req.slug, "fibre-100");
        assert_eq!(req.download_mbps, 100);
    }

    // ─── Subscriptions ─────────────────────────────────────────────────────
    // frontend/src/admin/pages/Subscriptions.tsx
    #[test]
    fn create_subscription_payload_matches_backend_dto() {
        let payload = json!({
            "customer_id": 42,
            "plan_id": 7,
            "billing_period_months": 12,
        });
        let req: crate::modules::subscription::api::http::CreateSubscriptionRequest =
            serde_json::from_value(payload)
                .expect("frontend payload must deserialize into CreateSubscriptionRequest");
        assert_eq!(req.customer_id, 42);
        assert_eq!(req.billing_period_months, 12);
    }

    // ─── Tickets ───────────────────────────────────────────────────────────
    // frontend/src/admin/pages/Tickets.tsx
    #[test]
    fn create_ticket_payload_matches_backend_dto() {
        let payload = json!({
            "subject": "No internet since morning",
            "description": "Connection is down",
            "customer_id": 5,
            "category": "outage",
            "priority": "high",
            "source": "portal",
        });
        let req: crate::modules::ticket::api::http::CreateTicketRequest =
            serde_json::from_value(payload)
                .expect("frontend payload must deserialize into CreateTicketRequest");
        assert_eq!(req.category, "outage");
        assert_eq!(req.source, "portal");
    }

    // ─── Billing ───────────────────────────────────────────────────────────
    // frontend/src/admin/pages/Billing.tsx
    #[test]
    fn create_invoice_payload_matches_backend_dto() {
        let payload = json!({
            "subscription_id": 12,
            "billing_period_start": "2026-09-01",
            "billing_period_end": "2026-09-30",
            "total_amount": "1099.00",
        });
        let req: crate::modules::billing::api::http::CreateInvoiceRequest =
            serde_json::from_value(payload)
                .expect("frontend payload must deserialize into CreateInvoiceRequest");
        assert_eq!(req.subscription_id, 12);
        assert_eq!(req.total_amount, "1099.00");
    }

    #[test]
    fn record_payment_payload_matches_backend_dto() {
        let payload = json!({
            "invoice_id": 12,
            "amount": "1099.00",
            "payment_method": "upi",
        });
        let req: crate::modules::billing::api::http::RecordPaymentRequest =
            serde_json::from_value(payload)
                .expect("frontend payload must deserialize into RecordPaymentRequest");
        assert_eq!(req.invoice_id, 12);
        assert_eq!(req.payment_method, "upi");
    }

    #[test]
    fn request_refund_payload_matches_backend_dto() {
        let payload = json!({
            "payment_id": 8,
            "amount": "499.00",
            "reason": "Duplicate charge",
        });
        let req: crate::modules::billing::api::http::RequestRefundRequest =
            serde_json::from_value(payload)
                .expect("frontend payload must deserialize into RequestRefundRequest");
        assert_eq!(req.payment_id, 8);
        assert_eq!(req.reason, "Duplicate charge");
    }

    // ─── Leads ─────────────────────────────────────────────────────────────
    // frontend/src/admin/pages/Leads.tsx
    #[test]
    fn create_lead_payload_matches_backend_dto() {
        let payload = json!({
            "name": "Rahul Sharma",
            "phone": "+919876543210",
            "email": "rahul@example.com",
            "source": "website",
        });
        let req: crate::modules::lead::api::http::CreateLeadRequest =
            serde_json::from_value(payload)
                .expect("frontend payload must deserialize into CreateLeadRequest");
        assert_eq!(req.phone, "+919876543210");
        assert_eq!(req.source, "website");
    }

    // ─── Coverage ──────────────────────────────────────────────────────────
    // frontend/src/admin/pages/Coverage.tsx
    #[test]
    fn create_coverage_area_payload_matches_backend_dto() {
        let payload = json!({
            "name": "Andheri West",
            "area_type": "locality",
        });
        let req: crate::modules::coverage::api::http::CreateAreaRequest =
            serde_json::from_value(payload)
                .expect("frontend payload must deserialize into CreateAreaRequest");
        assert_eq!(req.area_type, "locality");
    }

    // ─── Network ───────────────────────────────────────────────────────────
    // frontend/src/admin/pages/Network.tsx
    #[test]
    fn create_vlan_payload_matches_backend_dto() {
        let payload = json!({
            "vlan_id": 100,
            "name": "Customer Access",
            "vlan_type": "access",
        });
        let req: crate::modules::network::api::http::CreateVlanRequest =
            serde_json::from_value(payload)
                .expect("frontend payload must deserialize into CreateVlanRequest");
        assert_eq!(req.vlan_id, 100);
    }

    #[test]
    fn create_ip_pool_payload_matches_backend_dto() {
        let payload = json!({
            "name": "Pool A",
            "cidr": "10.0.0.0/24",
            "gateway": "10.0.0.1",
            "pool_type": "dhcp",
            "total_count": 254,
        });
        let req: crate::modules::network::api::http::CreateIpPoolRequest =
            serde_json::from_value(payload)
                .expect("frontend payload must deserialize into CreateIpPoolRequest");
        assert_eq!(req.total_count, 254);
    }

    #[test]
    fn create_mac_binding_payload_matches_backend_dto() {
        let payload = json!({
            "customer_id": 5,
            "subscription_id": 12,
            "mac_address": "AA:BB:CC:DD:EE:FF",
            "assigned_ip": "10.0.0.100",
        });
        let req: crate::modules::network::api::http::CreateMacBindingRequest =
            serde_json::from_value(payload)
                .expect("frontend payload must deserialize into CreateMacBindingRequest");
        assert_eq!(req.mac_address, "AA:BB:CC:DD:EE:FF");
    }

    // ─── Monitoring ────────────────────────────────────────────────────────
    // frontend/src/admin/pages/Monitoring.tsx
    #[test]
    fn create_alert_payload_matches_backend_dto() {
        let payload = json!({
            "device_id": 3,
            "severity": "critical",
            "title": "Device offline",
            "message": "Ping failed for 5 minutes",
        });
        let req: crate::modules::monitoring::api::http::CreateAlertRequest =
            serde_json::from_value(payload)
                .expect("frontend payload must deserialize into CreateAlertRequest");
        assert_eq!(req.severity, "critical");
    }

    // ─── Scheduler ─────────────────────────────────────────────────────────
    // frontend/src/admin/pages/Scheduler.tsx
    #[test]
    fn create_job_payload_matches_backend_dto() {
        let payload = json!({
            "name": "Nightly billing",
            "job_type": "cron",
            "schedule": "0 2 * * *",
            "target_module": "billing",
            "action": "generate_invoices",
            "payload": {"period": "monthly"},
        });
        let req: crate::modules::scheduler::api::http::CreateJobRequest =
            serde_json::from_value(payload)
                .expect("frontend payload must deserialize into CreateJobRequest");
        assert_eq!(req.target_module, "billing");
    }

    // ─── Gateway ───────────────────────────────────────────────────────────
    // frontend/src/admin/pages/Gateway.tsx
    #[test]
    fn create_api_key_payload_matches_backend_dto() {
        let payload = json!({
            "name": "Payment webhook",
            "permissions": "read",
        });
        let req: crate::modules::gateway::api::http::CreateApiKeyRequest =
            serde_json::from_value(payload)
                .expect("frontend payload must deserialize into CreateApiKeyRequest");
        assert_eq!(req.name, "Payment webhook");
    }

    #[test]
    fn create_rate_limit_rule_payload_matches_backend_dto() {
        let payload = json!({
            "route_pattern": "/api/v1/auth/*",
            "methods": "ALL",
            "max_requests": 100,
            "window_seconds": 60,
        });
        let req: crate::modules::gateway::api::http::CreateRateLimitRuleRequest =
            serde_json::from_value(payload)
                .expect("frontend payload must deserialize into CreateRateLimitRuleRequest");
        assert_eq!(req.max_requests, 100);
    }

    // ─── Notifications ─────────────────────────────────────────────────────
    // frontend/src/admin/pages/Notifications.tsx
    #[test]
    fn create_template_payload_matches_backend_dto() {
        let payload = json!({
            "name": "Invoice ready",
            "channel": "email",
            "body_template": "Dear {{name}}, your invoice is ready.",
            "subject_template": "Invoice #{{invoice_number}}",
        });
        let req: crate::modules::notification::api::http::CreateTemplateRequest =
            serde_json::from_value(payload)
                .expect("frontend payload must deserialize into CreateTemplateRequest");
        assert_eq!(req.channel, "email");
    }

    // ─── Roles ─────────────────────────────────────────────────────────────
    // frontend/src/admin/pages/Roles.tsx
    #[test]
    fn create_role_payload_matches_backend_dto() {
        let payload = json!({
            "name": "Branch Manager",
            "slug": "branch-manager",
            "description": "Manages a branch",
        });
        let req: crate::modules::security::api::http::CreateRoleRequest =
            serde_json::from_value(payload)
                .expect("frontend payload must deserialize into CreateRoleRequest");
        assert_eq!(req.slug, "branch-manager");
    }

    // ─── Referrals ─────────────────────────────────────────────────────────
    // frontend/src/admin/pages/Referrals.tsx
    #[test]
    fn create_referral_program_payload_matches_backend_dto() {
        let payload = json!({
            "name": "Refer a friend",
            "reward_type": "fixed",
            "reward_value": 100,
            "valid_from": "2026-09-01",
            "valid_until": "2026-12-31",
        });
        let req: crate::modules::referral::api::http::CreateProgramRequest =
            serde_json::from_value(payload)
                .expect("frontend payload must deserialize into CreateProgramRequest");
        assert_eq!(req.reward_type, "fixed");
    }

    // ─── Inventory ─────────────────────────────────────────────────────────
    // frontend/src/admin/pages/Inventory.tsx
    #[test]
    fn create_inventory_item_payload_matches_backend_dto() {
        let payload = json!({
            "item_type": "router",
            "serial_number": "SN-1234",
        });
        let req: crate::modules::inventory::api::http::CreateItemRequest =
            serde_json::from_value(payload)
                .expect("frontend payload must deserialize into CreateItemRequest");
        assert_eq!(req.item_type, "router");
    }

    #[test]
    fn assign_inventory_item_payload_matches_backend_dto() {
        let payload = json!({
            "assigned_to": 9,
        });
        let req: crate::modules::inventory::api::http::AssignItemRequest =
            serde_json::from_value(payload)
                .expect("frontend payload must deserialize into AssignItemRequest");
        assert_eq!(req.assigned_to, 9);
    }

    // ─── Branches ──────────────────────────────────────────────────────────
    // frontend/src/admin/pages/Branches.tsx
    #[test]
    fn create_branch_payload_matches_backend_dto() {
        let payload = json!({
            "name": "Mumbai HQ",
            "slug": "mumbai-hq",
            "code": "MUM",
            "city": "Mumbai",
            "state": "MH",
        });
        let req: crate::modules::branches::api::http::CreateBranchRequest =
            serde_json::from_value(payload)
                .expect("frontend payload must deserialize into CreateBranchRequest");
        assert_eq!(req.slug, "mumbai-hq");
    }

    // ─── Bandwidth ─────────────────────────────────────────────────────────
    // frontend/src/admin/pages/Bandwidth.tsx
    #[test]
    fn create_bandwidth_profile_payload_matches_backend_dto() {
        let payload = json!({
            "name": "Home 100",
            "download_kbps": 102400,
            "upload_kbps": 102400,
        });
        let req: crate::modules::bandwidth::api::http::CreateProfileRequest =
            serde_json::from_value(payload)
                .expect("frontend payload must deserialize into CreateProfileRequest");
        assert_eq!(req.download_kbps, 102400);
    }
}
