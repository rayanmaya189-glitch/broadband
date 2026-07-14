use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, Set, ActiveModelTrait};

use crate::modules::permission::model::permission_entity;

/// Seed all API permissions into the database on server startup.
/// Uses ON CONFLICT DO NOTHING to ensure no duplicates are inserted.
pub async fn seed_permissions(db: &DatabaseConnection) -> Result<u64, sea_orm::DbErr> {
    let permissions = vec![
        // ── Auth ─────────────────────────────────────────────
        ("auth.login", "POST", "/api/v1/auth/login", "public", "auth"),
        ("auth.login.otp.send", "POST", "/api/v1/auth/login/otp/send", "public", "auth"),
        ("auth.login.otp.verify", "POST", "/api/v1/auth/login/otp/verify", "public", "auth"),
        ("auth.register", "POST", "/api/v1/auth/register", "public", "auth"),
        ("auth.refresh", "POST", "/api/v1/auth/refresh", "public", "auth"),
        ("auth.password.reset.request", "POST", "/api/v1/auth/password/reset/request", "public", "auth"),
        ("auth.password.reset.confirm", "POST", "/api/v1/auth/password/reset/confirm", "public", "auth"),
        ("auth.logout", "POST", "/api/v1/users/logout", "jwt", "auth"),
        ("auth.logout.all", "POST", "/api/v1/users/logout/all", "jwt", "auth"),
        ("auth.password.change", "POST", "/api/v1/users/password/change", "jwt", "auth"),

        // ── Users ────────────────────────────────────────────
        ("user.me", "GET", "/api/v1/users/me", "jwt", "user"),
        ("user.sessions", "GET", "/api/v1/users/sessions", "jwt", "user"),
        ("user.view", "GET", "/api/v1/users/", "jwt", "user"),
        ("user.create", "POST", "/api/v1/users/", "jwt", "user"),
        ("user.update", "PUT", "/api/v1/users/:id", "jwt", "user"),
        ("user.delete", "DELETE", "/api/v1/users/:id", "jwt", "user"),
        ("user.2fa.enable", "POST", "/api/v1/users/2fa/enable", "jwt", "user"),
        ("user.2fa.confirm", "POST", "/api/v1/users/2fa/confirm", "jwt", "user"),
        ("user.2fa.disable", "POST", "/api/v1/users/2fa/disable", "jwt", "user"),

        // ── Roles ────────────────────────────────────────────
        ("role.view", "GET", "/api/v1/roles/", "jwt", "role"),
        ("role.create", "POST", "/api/v1/roles/", "jwt", "role"),
        ("role.update", "PUT", "/api/v1/roles/:id", "jwt", "role"),
        ("role.delete", "DELETE", "/api/v1/roles/:id", "jwt", "role"),
        ("role.assign_permissions", "POST", "/api/v1/roles/:id/permissions", "jwt", "role"),
        ("role.remove_permission", "DELETE", "/api/v1/roles/:id/permissions", "jwt", "role"),
        ("role.list_user_roles", "GET", "/api/v1/roles/user/:uid/roles", "jwt", "role"),
        ("role.assign_role_to_user", "POST", "/api/v1/roles/user/:uid/roles", "jwt", "role"),
        ("role.revoke_role_from_user", "DELETE", "/api/v1/roles/user/:uid/roles", "jwt", "role"),

        // ── Permissions ──────────────────────────────────────
        ("permission.view", "GET", "/api/v1/permissions/", "jwt", "permission"),
        ("permission.create", "POST", "/api/v1/permissions/", "jwt", "permission"),
        ("permission.delete", "DELETE", "/api/v1/permissions/:id", "jwt", "permission"),

        // ── Branches ─────────────────────────────────────────
        ("branch.view", "GET", "/api/v1/branches/", "jwt", "branch"),
        ("branch.create", "POST", "/api/v1/branches/", "jwt", "branch"),
        ("branch.update", "PUT", "/api/v1/branches/:id", "jwt", "branch"),
        ("branch.delete", "DELETE", "/api/v1/branches/:id", "jwt", "branch"),
        ("branch.working_hours.view", "GET", "/api/v1/branches/:id/working-hours", "jwt", "branch"),
        ("branch.working_hours.update", "PUT", "/api/v1/branches/:id/working-hours", "jwt", "branch"),
        ("branch.users.view", "GET", "/api/v1/branches/:id/users", "jwt", "branch"),
        ("branch.users.assign", "POST", "/api/v1/branches/:id/users", "jwt", "branch"),
        ("branch.users.remove", "DELETE", "/api/v1/branches/:id/users/:uid", "jwt", "branch"),
        ("branch.stats", "GET", "/api/v1/branches/:id/stats", "jwt", "branch"),

        // ── Customers ────────────────────────────────────────
        ("customer.view", "GET", "/api/v1/customers/", "jwt", "customer"),
        ("customer.create", "POST", "/api/v1/customers/", "jwt", "customer"),
        ("customer.get", "GET", "/api/v1/customers/:id", "jwt", "customer"),
        ("customer.update", "PUT", "/api/v1/customers/:id", "jwt", "customer"),
        ("customer.delete", "DELETE", "/api/v1/customers/:id", "jwt", "customer"),
        ("customer.status.update", "PUT", "/api/v1/customers/:id/status", "jwt", "customer"),
        ("customer.profile.view", "GET", "/api/v1/customers/:id/profile", "jwt", "customer"),
        ("customer.profile.update", "PUT", "/api/v1/customers/:id/profile", "jwt", "customer"),
        ("customer.kyc.submit", "POST", "/api/v1/customers/:id/kyc/submit", "jwt", "customer"),
        ("customer.kyc.verify", "POST", "/api/v1/customers/:id/kyc/verify", "jwt", "customer"),
        ("customer.kyc.documents.view", "GET", "/api/v1/customers/:id/kyc/documents", "jwt", "customer"),
        ("customer.kyc.documents.upload", "POST", "/api/v1/customers/:id/kyc/documents", "jwt", "customer"),
        ("customer.kyc.documents.delete", "DELETE", "/api/v1/customers/:id/kyc/documents/:doc_id", "jwt", "customer"),
        ("customer.addresses.view", "GET", "/api/v1/customers/:id/addresses", "jwt", "customer"),
        ("customer.addresses.create", "POST", "/api/v1/customers/:id/addresses", "jwt", "customer"),
        ("customer.addresses.update", "PUT", "/api/v1/customers/:id/addresses/:addr_id", "jwt", "customer"),
        ("customer.addresses.delete", "DELETE", "/api/v1/customers/:id/addresses/:addr_id", "jwt", "customer"),

        // ── Plans ────────────────────────────────────────────
        ("plan.view", "GET", "/api/v1/plans/", "jwt", "plan"),
        ("plan.create", "POST", "/api/v1/plans/", "jwt", "plan"),
        ("plan.get", "GET", "/api/v1/plans/:id", "jwt", "plan"),
        ("plan.update", "PUT", "/api/v1/plans/:id", "jwt", "plan"),
        ("plan.delete", "DELETE", "/api/v1/plans/:id", "jwt", "plan"),
        ("plan.publish", "POST", "/api/v1/plans/:id/publish", "jwt", "plan"),
        ("plan.unpublish", "POST", "/api/v1/plans/:id/unpublish", "jwt", "plan"),
        ("plan.clone", "POST", "/api/v1/plans/:id/clone", "jwt", "plan"),
        ("plan.speed_profile.view", "GET", "/api/v1/plans/:id/speed-profile", "jwt", "plan"),
        ("plan.speed_profile.create", "POST", "/api/v1/plans/:id/speed-profile", "jwt", "plan"),
        ("plan.speed_profile.delete", "POST", "/api/v1/plans/:id/speed-profile/delete", "jwt", "plan"),
        ("plan.pricing.view", "GET", "/api/v1/plans/:id/pricing", "jwt", "plan"),
        ("plan.pricing.update", "PUT", "/api/v1/plans/:id/pricing", "jwt", "plan"),

        // ── Subscriptions ────────────────────────────────────
        ("subscription.view", "GET", "/api/v1/subscriptions/", "jwt", "subscription"),
        ("subscription.create", "POST", "/api/v1/subscriptions/", "jwt", "subscription"),
        ("subscription.get", "GET", "/api/v1/subscriptions/:id", "jwt", "subscription"),
        ("subscription.suspend", "POST", "/api/v1/subscriptions/:id/suspend", "jwt", "subscription"),
        ("subscription.reactivate", "POST", "/api/v1/subscriptions/:id/reactivate", "jwt", "subscription"),
        ("subscription.cancel", "POST", "/api/v1/subscriptions/:id/cancel", "jwt", "subscription"),
        ("subscription.upgrade", "POST", "/api/v1/subscriptions/:id/upgrade", "jwt", "subscription"),
        ("subscription.downgrade", "POST", "/api/v1/subscriptions/:id/downgrade", "jwt", "subscription"),
        ("subscription.history", "GET", "/api/v1/subscriptions/:id/history", "jwt", "subscription"),

        // ── Billing ──────────────────────────────────────────
        ("billing.invoice.view", "GET", "/api/v1/billing/invoices", "jwt", "billing"),
        ("billing.invoice.create", "POST", "/api/v1/billing/invoices", "jwt", "billing"),
        ("billing.invoice.get", "GET", "/api/v1/billing/invoices/:id", "jwt", "billing"),
        ("billing.invoice.line_items", "GET", "/api/v1/billing/invoices/:id/line-items", "jwt", "billing"),
        ("billing.invoice.send", "POST", "/api/v1/billing/invoices/:id/send", "jwt", "billing"),
        ("billing.invoice.void", "POST", "/api/v1/billing/invoices/:id/void", "jwt", "billing"),
        ("billing.invoice.review", "POST", "/api/v1/billing/invoices/:id/review", "jwt", "billing"),
        ("billing.payment.view", "GET", "/api/v1/billing/payments", "jwt", "billing"),
        ("billing.payment.record", "POST", "/api/v1/billing/payments", "jwt", "billing"),
        ("billing.refund.request", "POST", "/api/v1/billing/refunds", "jwt", "billing"),
        ("billing.refund.approve", "POST", "/api/v1/billing/refunds/:id/approve", "jwt", "billing"),
        ("billing.discount.view", "GET", "/api/v1/billing/discounts", "jwt", "billing"),
        ("billing.discount.create", "POST", "/api/v1/billing/discounts", "jwt", "billing"),
        ("billing.dunning_config.view", "GET", "/api/v1/billing/dunning/config", "jwt", "billing"),
        ("billing.dunning_config.update", "PUT", "/api/v1/billing/dunning/config", "jwt", "billing"),
        ("billing.tax_config.view", "GET", "/api/v1/billing/tax/config", "jwt", "billing"),
        ("billing.tax_config.update", "PUT", "/api/v1/billing/tax/config", "jwt", "billing"),

        // ── Tickets ──────────────────────────────────────────
        ("ticket.view", "GET", "/api/v1/tickets/", "jwt", "ticket"),
        ("ticket.create", "POST", "/api/v1/tickets/", "jwt", "ticket"),
        ("ticket.dashboard", "GET", "/api/v1/tickets/dashboard", "jwt", "ticket"),
        ("ticket.my_assignments", "GET", "/api/v1/tickets/my-assignments", "jwt", "ticket"),
        ("ticket.get", "GET", "/api/v1/tickets/:id", "jwt", "ticket"),
        ("ticket.update", "PUT", "/api/v1/tickets/:id", "jwt", "ticket"),
        ("ticket.delete", "DELETE", "/api/v1/tickets/:id", "jwt", "ticket"),
        ("ticket.assign", "POST", "/api/v1/tickets/:id/assign", "jwt", "ticket"),
        ("ticket.escalate", "POST", "/api/v1/tickets/:id/escalate", "jwt", "ticket"),
        ("ticket.resolve", "POST", "/api/v1/tickets/:id/resolve", "jwt", "ticket"),
        ("ticket.close", "POST", "/api/v1/tickets/:id/close", "jwt", "ticket"),
        ("ticket.reopen", "POST", "/api/v1/tickets/:id/reopen", "jwt", "ticket"),
        ("ticket.feedback", "POST", "/api/v1/tickets/:id/feedback", "jwt", "ticket"),
        ("ticket.comments.view", "GET", "/api/v1/tickets/:id/comments", "jwt", "ticket"),
        ("ticket.comments.create", "POST", "/api/v1/tickets/:id/comments", "jwt", "ticket"),
        ("ticket.escalations", "GET", "/api/v1/tickets/:id/escalations", "jwt", "ticket"),
        ("ticket.status_history", "GET", "/api/v1/tickets/:id/status-history", "jwt", "ticket"),

        // ── Devices ──────────────────────────────────────────
        ("device.view", "GET", "/api/v1/devices/", "jwt", "device"),
        ("device.create", "POST", "/api/v1/devices/", "jwt", "device"),
        ("device.models.view", "GET", "/api/v1/devices/models", "jwt", "device"),
        ("device.models.create", "POST", "/api/v1/devices/models", "jwt", "device"),
        ("device.get", "GET", "/api/v1/devices/:id", "jwt", "device"),
        ("device.update", "PUT", "/api/v1/devices/:id", "jwt", "device"),
        ("device.delete", "DELETE", "/api/v1/devices/:id", "jwt", "device"),
        ("device.restart", "POST", "/api/v1/devices/:id/restart", "jwt", "device"),
        ("device.shutdown", "POST", "/api/v1/devices/:id/shutdown", "jwt", "device"),
        ("device.ports.view", "GET", "/api/v1/devices/:id/ports", "jwt", "device"),
        ("device.ports.update", "POST", "/api/v1/devices/:id/ports/:port_id", "jwt", "device"),
        ("device.firmware.view", "GET", "/api/v1/devices/:id/firmware", "jwt", "device"),
        ("device.firmware.create", "POST", "/api/v1/devices/:id/firmware/update", "jwt", "device"),
        ("device.firmware.status", "POST", "/api/v1/devices/firmware/:update_id/status", "jwt", "device"),
        ("device.metrics", "GET", "/api/v1/devices/:id/metrics", "jwt", "device"),
        ("device.logs.view", "GET", "/api/v1/devices/:id/logs", "jwt", "device"),
        ("device.logs.create", "POST", "/api/v1/devices/:id/logs", "jwt", "device"),

        // ── Bandwidth ────────────────────────────────────────
        ("bandwidth.profile.view", "GET", "/api/v1/bandwidth/profiles", "jwt", "bandwidth"),
        ("bandwidth.profile.create", "POST", "/api/v1/bandwidth/profiles", "jwt", "bandwidth"),
        ("bandwidth.profile.get", "GET", "/api/v1/bandwidth/profiles/:id", "jwt", "bandwidth"),
        ("bandwidth.profile.update", "PUT", "/api/v1/bandwidth/profiles/:id", "jwt", "bandwidth"),
        ("bandwidth.profile.delete", "DELETE", "/api/v1/bandwidth/profiles/:id", "jwt", "bandwidth"),
        ("bandwidth.profile.apply", "POST", "/api/v1/bandwidth/profiles/:id/apply", "jwt", "bandwidth"),
        ("bandwidth.applications.view", "GET", "/api/v1/bandwidth/applications", "jwt", "bandwidth"),
        ("bandwidth.usage.view", "GET", "/api/v1/bandwidth/usage/:subscription_id", "jwt", "bandwidth"),

        // ── Network ──────────────────────────────────────────
        ("network.vlan.view", "GET", "/api/v1/network/vlans", "jwt", "network"),
        ("network.vlan.create", "POST", "/api/v1/network/vlans", "jwt", "network"),
        ("network.vlan.update", "PUT", "/api/v1/network/vlans/:id", "jwt", "network"),
        ("network.vlan.delete", "DELETE", "/api/v1/network/vlans/:id", "jwt", "network"),
        ("network.ip_pool.view", "GET", "/api/v1/network/ip-pools", "jwt", "network"),
        ("network.ip_pool.create", "POST", "/api/v1/network/ip-pools", "jwt", "network"),
        ("network.ip_pool.addresses", "GET", "/api/v1/network/ip-pools/:pool_id/addresses", "jwt", "network"),
        ("network.ip_pool.allocate", "POST", "/api/v1/network/ip-pools/allocate", "jwt", "network"),
        ("network.ip_pool.release", "POST", "/api/v1/network/ip-pools/release", "jwt", "network"),
        ("network.pppoe.sessions.view", "GET", "/api/v1/network/pppoe/sessions", "jwt", "network"),
        ("network.pppoe.sessions.create", "POST", "/api/v1/network/pppoe/sessions", "jwt", "network"),
        ("network.pppoe.sessions.terminate", "POST", "/api/v1/network/pppoe/sessions/:id/terminate", "jwt", "network"),
        ("network.mac_binding.view", "GET", "/api/v1/network/mac-bindings", "jwt", "network"),
        ("network.mac_binding.create", "POST", "/api/v1/network/mac-bindings", "jwt", "network"),
        ("network.mac_binding.delete", "DELETE", "/api/v1/network/mac-bindings/:id", "jwt", "network"),
        ("network.dhcp.view", "GET", "/api/v1/network/dhcp/leases", "jwt", "network"),
        ("network.sessions.view", "GET", "/api/v1/network/sessions", "jwt", "network"),
        ("network.topology.view", "GET", "/api/v1/network/topology", "jwt", "network"),

        // ── Coverage ─────────────────────────────────────────
        ("coverage.area.view", "GET", "/api/v1/coverage/areas", "jwt", "coverage"),
        ("coverage.area.create", "POST", "/api/v1/coverage/areas", "jwt", "coverage"),
        ("coverage.area.get", "GET", "/api/v1/coverage/areas/:id", "jwt", "coverage"),
        ("coverage.area.update", "PUT", "/api/v1/coverage/areas/:id", "jwt", "coverage"),
        ("coverage.area.delete", "DELETE", "/api/v1/coverage/areas/:id", "jwt", "coverage"),
        ("coverage.pincode.view", "GET", "/api/v1/coverage/areas/:id/pincodes", "jwt", "coverage"),
        ("coverage.pincode.add", "POST", "/api/v1/coverage/areas/:id/pincodes", "jwt", "coverage"),
        ("coverage.pincode.remove", "DELETE", "/api/v1/coverage/areas/:id/pincodes/:pincode", "jwt", "coverage"),
        ("coverage.check", "POST", "/api/v1/coverage/check", "jwt", "coverage"),
        ("coverage.stats", "GET", "/api/v1/coverage/stats", "jwt", "coverage"),

        // ── Installations ────────────────────────────────────
        ("installation.view", "GET", "/api/v1/installations/", "jwt", "installation"),
        ("installation.create", "POST", "/api/v1/installations/", "jwt", "installation"),
        ("installation.my_assignments", "GET", "/api/v1/installations/my-assignments", "jwt", "installation"),
        ("installation.get", "GET", "/api/v1/installations/:id", "jwt", "installation"),
        ("installation.schedule", "PUT", "/api/v1/installations/:id/schedule", "jwt", "installation"),
        ("installation.reschedule", "PUT", "/api/v1/installations/:id/reschedule", "jwt", "installation"),
        ("installation.start", "PUT", "/api/v1/installations/:id/start", "jwt", "installation"),
        ("installation.complete", "PUT", "/api/v1/installations/:id/complete", "jwt", "installation"),
        ("installation.cancel", "PUT", "/api/v1/installations/:id/cancel", "jwt", "installation"),
        ("installation.photos.upload", "POST", "/api/v1/installations/:id/photos", "jwt", "installation"),

        // ── Inventory ────────────────────────────────────────
        ("inventory.view", "GET", "/api/v1/inventory/", "jwt", "inventory"),
        ("inventory.create", "POST", "/api/v1/inventory/", "jwt", "inventory"),
        ("inventory.reports", "GET", "/api/v1/inventory/reports", "jwt", "inventory"),
        ("inventory.alerts", "GET", "/api/v1/inventory/alerts", "jwt", "inventory"),
        ("inventory.get", "GET", "/api/v1/inventory/:id", "jwt", "inventory"),
        ("inventory.delete", "DELETE", "/api/v1/inventory/:id", "jwt", "inventory"),
        ("inventory.assign", "POST", "/api/v1/inventory/:id/assign", "jwt", "inventory"),
        ("inventory.install", "POST", "/api/v1/inventory/:id/install", "jwt", "inventory"),
        ("inventory.return", "POST", "/api/v1/inventory/:id/return", "jwt", "inventory"),
        ("inventory.transfer", "POST", "/api/v1/inventory/:id/transfer", "jwt", "inventory"),
        ("inventory.scrap", "POST", "/api/v1/inventory/:id/scrap", "jwt", "inventory"),
        ("inventory.movements", "GET", "/api/v1/inventory/:id/movements", "jwt", "inventory"),

        // ── Leads ────────────────────────────────────────────
        ("lead.view", "GET", "/api/v1/leads/", "jwt", "lead"),
        ("lead.create", "POST", "/api/v1/leads/", "jwt", "lead"),
        ("lead.pipeline", "GET", "/api/v1/leads/pipeline", "jwt", "lead"),
        ("lead.stats", "GET", "/api/v1/leads/stats", "jwt", "lead"),
        ("lead.get", "GET", "/api/v1/leads/:id", "jwt", "lead"),
        ("lead.update", "PUT", "/api/v1/leads/:id", "jwt", "lead"),
        ("lead.delete", "DELETE", "/api/v1/leads/:id", "jwt", "lead"),
        ("lead.status.update", "POST", "/api/v1/leads/:id/status", "jwt", "lead"),
        ("lead.assign", "POST", "/api/v1/leads/:id/assign", "jwt", "lead"),
        ("lead.activities.view", "GET", "/api/v1/leads/:id/activities", "jwt", "lead"),
        ("lead.activities.create", "POST", "/api/v1/leads/:id/activities", "jwt", "lead"),
        ("lead.convert", "POST", "/api/v1/leads/:id/convert", "jwt", "lead"),

        // ── Referrals ────────────────────────────────────────
        ("referral.program.view", "GET", "/api/v1/referrals/programs", "jwt", "referral"),
        ("referral.program.create", "POST", "/api/v1/referrals/programs", "jwt", "referral"),
        ("referral.program.update", "PUT", "/api/v1/referrals/programs/:id", "jwt", "referral"),
        ("referral.tracking.view", "GET", "/api/v1/referrals/tracking", "jwt", "referral"),
        ("referral.tracking.share", "POST", "/api/v1/referrals/tracking", "jwt", "referral"),
        ("referral.stats", "GET", "/api/v1/referrals/stats/:referrer_id", "jwt", "referral"),
        ("referral.wallet.view", "GET", "/api/v1/referrals/wallet/:customer_id", "jwt", "referral"),
        ("referral.wallet.get_or_create", "POST", "/api/v1/referrals/wallet/:customer_id", "jwt", "referral"),
        ("referral.wallet.credit", "POST", "/api/v1/referrals/wallet/:customer_id/credit", "jwt", "referral"),
        ("referral.wallet.debit", "POST", "/api/v1/referrals/wallet/:customer_id/debit", "jwt", "referral"),
        ("referral.wallet.transactions", "GET", "/api/v1/referrals/wallet/:customer_id/transactions", "jwt", "referral"),

        // ── Notifications ────────────────────────────────────
        ("notification.template.view", "GET", "/api/v1/notifications/templates", "jwt", "notification"),
        ("notification.template.create", "POST", "/api/v1/notifications/templates", "jwt", "notification"),
        ("notification.template.update", "PUT", "/api/v1/notifications/templates/:id", "jwt", "notification"),
        ("notification.template.delete", "DELETE", "/api/v1/notifications/templates/:id", "jwt", "notification"),
        ("notification.channel.view", "GET", "/api/v1/notifications/channels", "jwt", "notification"),
        ("notification.channel.upsert", "POST", "/api/v1/notifications/channels", "jwt", "notification"),
        ("notification.view", "GET", "/api/v1/notifications/", "jwt", "notification"),
        ("notification.send", "POST", "/api/v1/notifications/send", "jwt", "notification"),
        ("notification.retry", "POST", "/api/v1/notifications/:id/retry", "jwt", "notification"),
        ("notification.history.view", "GET", "/api/v1/notifications/history", "jwt", "notification"),

        // ── Events ───────────────────────────────────────────
        ("event.view", "GET", "/api/v1/events/", "jwt", "event"),
        ("event.publish", "POST", "/api/v1/events/", "jwt", "event"),
        ("event.stats", "GET", "/api/v1/events/stats", "jwt", "event"),
        ("event.subscription.view", "GET", "/api/v1/events/subscriptions", "jwt", "event"),
        ("event.subscription.create", "POST", "/api/v1/events/subscriptions", "jwt", "event"),
        ("event.subscription.delete", "DELETE", "/api/v1/events/subscriptions/:id", "jwt", "event"),
        ("event.get", "GET", "/api/v1/events/:id", "jwt", "event"),
        ("event.mark_processed", "POST", "/api/v1/events/:id", "jwt", "event"),
        ("event.aggregate.view", "GET", "/api/v1/events/aggregate/:aggregate_type/:aggregate_id", "jwt", "event"),

        // ── Documents ────────────────────────────────────────
        ("document.view", "GET", "/api/v1/documents/", "jwt", "document"),
        ("document.upload_url", "POST", "/api/v1/documents/upload-url", "jwt", "document"),
        ("document.get", "GET", "/api/v1/documents/:id", "jwt", "document"),
        ("document.delete", "DELETE", "/api/v1/documents/:id", "jwt", "document"),
        ("document.confirm_upload", "POST", "/api/v1/documents/:id/confirm", "jwt", "document"),
        ("document.associate", "PUT", "/api/v1/documents/:id/associate", "jwt", "document"),
        ("document.access_logs", "GET", "/api/v1/documents/:id/access-logs", "jwt", "document"),

        // ── Accounting ───────────────────────────────────────
        ("accounting.account.view", "GET", "/api/v1/accounting/accounts", "jwt", "accounting"),
        ("accounting.account.create", "POST", "/api/v1/accounting/accounts", "jwt", "accounting"),
        ("accounting.journal.view", "GET", "/api/v1/accounting/journal", "jwt", "accounting"),
        ("accounting.journal.create", "POST", "/api/v1/accounting/journal", "jwt", "accounting"),
        ("accounting.journal.lines", "GET", "/api/v1/accounting/journal/:id/lines", "jwt", "accounting"),
        ("accounting.journal.post", "POST", "/api/v1/accounting/journal/:id/post", "jwt", "accounting"),
        ("accounting.journal.void", "POST", "/api/v1/accounting/journal/:id/void", "jwt", "accounting"),
        ("accounting.trial_balance", "GET", "/api/v1/accounting/trial-balance", "jwt", "accounting"),
        ("accounting.profit_loss", "GET", "/api/v1/accounting/statements/profit-loss", "jwt", "accounting"),
        ("accounting.balance_sheet", "GET", "/api/v1/accounting/statements/balance-sheet", "jwt", "accounting"),
        ("accounting.cash_flow", "GET", "/api/v1/accounting/statements/cash-flow", "jwt", "accounting"),
        ("accounting.gst", "GET", "/api/v1/accounting/gst/:return_type", "jwt", "accounting"),

        // ── Payment Gateway ──────────────────────────────────
        ("payment_gateway.view", "GET", "/api/v1/payments/gateways", "jwt", "payment_gateway"),
        ("payment_gateway.create", "POST", "/api/v1/payments/gateways", "jwt", "payment_gateway"),
        ("payment_gateway.update", "PUT", "/api/v1/payments/gateways/:id", "jwt", "payment_gateway"),
        ("payment_gateway.create_link", "POST", "/api/v1/payments/create-link", "jwt", "payment_gateway"),
        ("payment_gateway.transactions.view", "GET", "/api/v1/payments/", "jwt", "payment_gateway"),
        ("payment_gateway.retry", "POST", "/api/v1/payments/retry", "jwt", "payment_gateway"),
        ("payment_gateway.webhook.razorpay", "POST", "/api/v1/payments/webhook/razorpay", "public", "payment_gateway"),
        ("payment_gateway.webhook.payu", "POST", "/api/v1/payments/webhook/payu", "public", "payment_gateway"),
        ("payment_gateway.webhook.instamojo", "POST", "/api/v1/payments/webhook/instamojo", "public", "payment_gateway"),

        // ── Discovery ────────────────────────────────────────
        ("discovery.scan.view", "GET", "/api/v1/discovery/scans", "jwt", "discovery"),
        ("discovery.scan.create", "POST", "/api/v1/discovery/scans", "jwt", "discovery"),
        ("discovery.scan.start", "POST", "/api/v1/discovery/scans/:id/start", "jwt", "discovery"),
        ("discovery.scan.stop", "POST", "/api/v1/discovery/scans/:id/stop", "jwt", "discovery"),
        ("discovery.result.view", "GET", "/api/v1/discovery/results", "jwt", "discovery"),
        ("discovery.result.approve", "POST", "/api/v1/discovery/results/:id/approve", "jwt", "discovery"),
        ("discovery.result.reject", "POST", "/api/v1/discovery/results/:id/reject", "jwt", "discovery"),
        ("discovery.dashboard", "GET", "/api/v1/discovery/dashboard", "jwt", "discovery"),

        // ── Realtime ─────────────────────────────────────────
        ("realtime.health", "GET", "/api/v1/realtime/health", "jwt", "realtime"),
        ("realtime.channels", "GET", "/api/v1/realtime/channels", "jwt", "realtime"),
        ("realtime.stats", "GET", "/api/v1/realtime/stats", "jwt", "realtime"),

        // ── Audit ────────────────────────────────────────────
        ("audit.log.view", "GET", "/api/v1/audit/logs", "jwt", "audit"),
        ("audit.log.export", "POST", "/api/v1/audit/logs", "jwt", "audit"),
        ("audit.log.get", "GET", "/api/v1/audit/logs/:id", "jwt", "audit"),
        ("audit.user_activity", "GET", "/api/v1/audit/user/:user_id", "jwt", "audit"),
        ("audit.resource_history", "GET", "/api/v1/audit/resource/:resource_type/:resource_id", "jwt", "audit"),

        // ── Entity History ───────────────────────────────────
        ("audit.entity_history.search", "GET", "/api/v1/audit/entity-history/", "jwt", "audit"),
        ("audit.entity_history.stats", "GET", "/api/v1/audit/entity-history/stats", "jwt", "audit"),
        ("audit.entity_history.rollback", "POST", "/api/v1/audit/entity-history/rollback", "jwt", "audit"),
        ("audit.entity_history.get", "GET", "/api/v1/audit/entity-history/:id", "jwt", "audit"),
        ("audit.entity_history.entity", "GET", "/api/v1/audit/entity-history/entity/:entity_type/:entity_id", "jwt", "audit"),
    ];

    let mut count: u64 = 0;
    for (name, method, api_url, guard, module) in &permissions {
        // Check if permission already exists
        let existing = permission_entity::Entity::find()
            .filter(permission_entity::Column::Name.eq(*name))
            .filter(permission_entity::Column::Method.eq(*method))
            .filter(permission_entity::Column::ApiUrl.eq(*api_url))
            .one(db)
            .await?;

        if existing.is_none() {
            let new_perm = permission_entity::ActiveModel {
                name: Set(name.to_string()),
                method: Set(method.to_string()),
                api_url: Set(api_url.to_string()),
                guard: Set(guard.to_string()),
                module: Set(module.to_string()),
                ..Default::default()
            };
            new_perm.insert(db).await?;
            count += 1;
        }
    }

    let total = permissions.len() as u64;
    if count > 0 {
        tracing::info!(total, inserted = count, "Permissions seeded successfully");
    } else {
        tracing::debug!(total, "All permissions already exist, nothing to insert");
    }
    Ok(count)
}
