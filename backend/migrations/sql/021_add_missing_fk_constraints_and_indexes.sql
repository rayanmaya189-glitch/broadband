-- AeroXe Backend Migration 021: Add Missing FK Constraints and Indexes
-- Adds critical foreign key constraints and ORM-level indexes that were
-- missing after the schema migration (019) dropped cross-schema FKs.

-- ============================================================
-- PART 1: Add missing FK constraints (cross-schema)
-- ============================================================

-- customer.customers.branch_id → branches.branches.id
ALTER TABLE customer.customers
    ADD CONSTRAINT fk_customers_branch
    FOREIGN KEY (branch_id) REFERENCES branches.branches(id);

-- subscription.subscriptions.customer_id → customer.customers.id
ALTER TABLE subscription.subscriptions
    ADD CONSTRAINT fk_subscriptions_customer
    FOREIGN KEY (customer_id) REFERENCES customer.customers(id);

-- subscription.subscriptions.plan_id → plans.plans.id
ALTER TABLE subscription.subscriptions
    ADD CONSTRAINT fk_subscriptions_plan
    FOREIGN KEY (plan_id) REFERENCES plans.plans(id);

-- billing.invoices.customer_id → customer.customers.id
ALTER TABLE billing.invoices
    ADD CONSTRAINT fk_invoices_customer
    FOREIGN KEY (customer_id) REFERENCES customer.customers(id);

-- billing.invoices.subscription_id → subscription.subscriptions.id
ALTER TABLE billing.invoices
    ADD CONSTRAINT fk_invoices_subscription
    FOREIGN KEY (subscription_id) REFERENCES subscription.subscriptions(id);

-- billing.payments.invoice_id → billing.invoices.id
ALTER TABLE billing.payments
    ADD CONSTRAINT fk_payments_invoice
    FOREIGN KEY (invoice_id) REFERENCES billing.invoices(id);

-- billing.payments.customer_id → customer.customers.id
ALTER TABLE billing.payments
    ADD CONSTRAINT fk_payments_customer
    FOREIGN KEY (customer_id) REFERENCES customer.customers(id);

-- billing.refunds.payment_id → billing.payments.id
ALTER TABLE billing.refunds
    ADD CONSTRAINT fk_refunds_payment
    FOREIGN KEY (payment_id) REFERENCES billing.payments(id);

-- billing.refunds.invoice_id → billing.invoices.id
ALTER TABLE billing.refunds
    ADD CONSTRAINT fk_refunds_invoice
    FOREIGN KEY (invoice_id) REFERENCES billing.invoices(id);

-- billing.refunds.customer_id → customer.customers.id
ALTER TABLE billing.refunds
    ADD CONSTRAINT fk_refunds_customer
    FOREIGN KEY (customer_id) REFERENCES customer.customers(id);

-- customer.customer_profiles.customer_id → customer.customers.id
ALTER TABLE customer.customer_profiles
    ADD CONSTRAINT fk_customer_profiles_customer
    FOREIGN KEY (customer_id) REFERENCES customer.customers(id);

-- customer.addresses.customer_id → customer.customers.id
ALTER TABLE customer.addresses
    ADD CONSTRAINT fk_addresses_customer
    FOREIGN KEY (customer_id) REFERENCES customer.customers(id);

-- customer.kyc_documents.customer_id → customer.customers.id
ALTER TABLE customer.kyc_documents
    ADD CONSTRAINT fk_kyc_documents_customer
    FOREIGN KEY (customer_id) REFERENCES customer.customers(id);

-- ticket.tickets.customer_id → customer.customers.id
ALTER TABLE ticket.tickets
    ADD CONSTRAINT fk_tickets_customer
    FOREIGN KEY (customer_id) REFERENCES customer.customers(id);

-- ticket.tickets.subscription_id → subscription.subscriptions.id
ALTER TABLE ticket.tickets
    ADD CONSTRAINT fk_tickets_subscription
    FOREIGN KEY (subscription_id) REFERENCES subscription.subscriptions(id);

-- ticket.tickets.branch_id → branches.branches.id
ALTER TABLE ticket.tickets
    ADD CONSTRAINT fk_tickets_branch
    FOREIGN KEY (branch_id) REFERENCES branches.branches(id);

-- ticket.ticket_comments.ticket_id → ticket.tickets.id
ALTER TABLE ticket.ticket_comments
    ADD CONSTRAINT fk_ticket_comments_ticket
    FOREIGN KEY (ticket_id) REFERENCES ticket.tickets(id);

-- network.pppoe_sessions.customer_id → customer.customers.id
ALTER TABLE network.pppoe_sessions
    ADD CONSTRAINT fk_pppoe_sessions_customer
    FOREIGN KEY (customer_id) REFERENCES customer.customers(id);

-- network.pppoe_sessions.subscription_id → subscription.subscriptions.id
ALTER TABLE network.pppoe_sessions
    ADD CONSTRAINT fk_pppoe_sessions_subscription
    FOREIGN KEY (subscription_id) REFERENCES subscription.subscriptions(id);

-- network.pppoe_sessions.branch_id → branches.branches.id
ALTER TABLE network.pppoe_sessions
    ADD CONSTRAINT fk_pppoe_sessions_branch
    FOREIGN KEY (branch_id) REFERENCES branches.branches(id);

-- network.mac_bindings.customer_id → customer.customers.id
ALTER TABLE network.mac_bindings
    ADD CONSTRAINT fk_mac_bindings_customer
    FOREIGN KEY (customer_id) REFERENCES customer.customers(id);

-- network.mac_bindings.subscription_id → subscription.subscriptions.id
ALTER TABLE network.mac_bindings
    ADD CONSTRAINT fk_mac_bindings_subscription
    FOREIGN KEY (subscription_id) REFERENCES subscription.subscriptions(id);

-- network.mac_bindings.branch_id → branches.branches.id
ALTER TABLE network.mac_bindings
    ADD CONSTRAINT fk_mac_bindings_branch
    FOREIGN KEY (branch_id) REFERENCES branches.branches(id);

-- network.dhcp_leases.branch_id → branches.branches.id
ALTER TABLE network.dhcp_leases
    ADD CONSTRAINT fk_dhcp_leases_branch
    FOREIGN KEY (branch_id) REFERENCES branches.branches(id);

-- network.ip_pools.branch_id → branches.branches.id
ALTER TABLE network.ip_pools
    ADD CONSTRAINT fk_ip_pools_branch
    FOREIGN KEY (branch_id) REFERENCES branches.branches(id);

-- network.vlans.branch_id → branches.branches.id
ALTER TABLE network.vlans
    ADD CONSTRAINT fk_vlans_branch
    FOREIGN KEY (branch_id) REFERENCES branches.branches(id);

-- lead.leads.branch_id → branches.branches.id
ALTER TABLE lead.leads
    ADD CONSTRAINT fk_leads_branch
    FOREIGN KEY (branch_id) REFERENCES branches.branches(id);

-- lead.leads.interested_plan_id → plans.plans.id
ALTER TABLE lead.leads
    ADD CONSTRAINT fk_leads_plan
    FOREIGN KEY (interested_plan_id) REFERENCES plans.plans(id);

-- device.network_devices.branch_id → branches.branches.id
ALTER TABLE device.network_devices
    ADD CONSTRAINT fk_network_devices_branch
    FOREIGN KEY (branch_id) REFERENCES branches.branches(id);

-- device.device_ports.device_id → device.network_devices.id
ALTER TABLE device.device_ports
    ADD CONSTRAINT fk_device_ports_device
    FOREIGN KEY (device_id) REFERENCES device.network_devices(id);

-- device.device_ports.customer_id → customer.customers.id
ALTER TABLE device.device_ports
    ADD CONSTRAINT fk_device_ports_customer
    FOREIGN KEY (customer_id) REFERENCES customer.customers(id);

-- device.device_metrics.device_id → device.network_devices.id
ALTER TABLE device.device_metrics
    ADD CONSTRAINT fk_device_metrics_device
    FOREIGN KEY (device_id) REFERENCES device.network_devices(id);

-- device.device_logs.device_id → device.network_devices.id
ALTER TABLE device.device_logs
    ADD CONSTRAINT fk_device_logs_device
    FOREIGN KEY (device_id) REFERENCES device.network_devices(id);

-- identity.user_sessions.user_id → identity.users.id
ALTER TABLE identity.user_sessions
    ADD CONSTRAINT fk_user_sessions_user
    FOREIGN KEY (user_id) REFERENCES identity.users(id);

-- identity.users.branch_id → branches.branches.id
ALTER TABLE identity.users
    ADD CONSTRAINT fk_users_branch
    FOREIGN KEY (branch_id) REFERENCES branches.branches(id);

-- notification.delivery_history.notification_id → notification.notifications.id
ALTER TABLE notification.notification_delivery_history
    ADD CONSTRAINT fk_delivery_history_notification
    FOREIGN KEY (notification_id) REFERENCES notification.notifications(id);

-- inventory.inventory_items.branch_id → branches.branches.id
ALTER TABLE inventory.inventory_items
    ADD CONSTRAINT fk_inventory_items_branch
    FOREIGN KEY (branch_id) REFERENCES branches.branches(id);

-- inventory.inventory_movements.item_id → inventory.inventory_items.id
ALTER TABLE inventory.inventory_movements
    ADD CONSTRAINT fk_inventory_movements_item
    FOREIGN KEY (item_id) REFERENCES inventory.inventory_items(id);

-- referral.customer_wallets.customer_id → customer.customers.id
ALTER TABLE referral.customer_wallets
    ADD CONSTRAINT fk_customer_wallets_customer
    FOREIGN KEY (customer_id) REFERENCES customer.customers(id);

-- referral.wallet_transactions.wallet_id → referral.customer_wallets.id
ALTER TABLE referral.wallet_transactions
    ADD CONSTRAINT fk_wallet_transactions_wallet
    FOREIGN KEY (wallet_id) REFERENCES referral.customer_wallets(id);

-- referral.referral_tracking.program_id → referral.referral_programs.id
ALTER TABLE referral.referral_tracking
    ADD CONSTRAINT fk_referral_tracking_program
    FOREIGN KEY (program_id) REFERENCES referral.referral_programs(id);

-- referral.referral_tracking.referrer_id → customer.customers.id
ALTER TABLE referral.referral_tracking
    ADD CONSTRAINT fk_referral_tracking_referrer
    FOREIGN KEY (referrer_id) REFERENCES customer.customers(id);

-- referral.referral_tracking.referee_id → customer.customers.id
ALTER TABLE referral.referral_tracking
    ADD CONSTRAINT fk_referral_tracking_referee
    FOREIGN KEY (referee_id) REFERENCES customer.customers(id);

-- installation.installation_orders.customer_id → customer.customers.id
ALTER TABLE installation.installation_orders
    ADD CONSTRAINT fk_installation_orders_customer
    FOREIGN KEY (customer_id) REFERENCES customer.customers(id);

-- installation.installation_orders.branch_id → branches.branches.id
ALTER TABLE installation.installation_orders
    ADD CONSTRAINT fk_installation_orders_branch
    FOREIGN KEY (branch_id) REFERENCES branches.branches(id);

-- installation.installation_orders.subscription_id → subscription.subscriptions.id
ALTER TABLE installation.installation_orders
    ADD CONSTRAINT fk_installation_orders_subscription
    FOREIGN KEY (subscription_id) REFERENCES subscription.subscriptions(id);

-- installation.installation_photos.installation_order_id → installation.installation_orders.id
ALTER TABLE installation.installation_photos
    ADD CONSTRAINT fk_installation_photos_order
    FOREIGN KEY (installation_order_id) REFERENCES installation.installation_orders(id);

-- installation.installation_equipment.installation_order_id → installation.installation_orders.id
ALTER TABLE installation.installation_equipment
    ADD CONSTRAINT fk_installation_equipment_order
    FOREIGN KEY (installation_order_id) REFERENCES installation.installation_orders(id);

-- bandwidth.bandwidth_applications.subscription_id → subscription.subscriptions.id
ALTER TABLE bandwidth.bandwidth_applications
    ADD CONSTRAINT fk_bandwidth_applications_subscription
    FOREIGN KEY (subscription_id) REFERENCES subscription.subscriptions(id);

-- bandwidth.bandwidth_profiles.plan_id → plans.plans.id
ALTER TABLE bandwidth.bandwidth_profiles
    ADD CONSTRAINT fk_bandwidth_profiles_plan
    FOREIGN KEY (plan_id) REFERENCES plans.plans(id);

-- payment.payment_links.invoice_id → billing.invoices.id
ALTER TABLE payment.payment_links
    ADD CONSTRAINT fk_payment_links_invoice
    FOREIGN KEY (invoice_id) REFERENCES billing.invoices(id);

-- payment.payment_links.customer_id → customer.customers.id
ALTER TABLE payment.payment_links
    ADD CONSTRAINT fk_payment_links_customer
    FOREIGN KEY (customer_id) REFERENCES customer.customers(id);

-- payment.payment_links.branch_id → branches.branches.id
ALTER TABLE payment.payment_links
    ADD CONSTRAINT fk_payment_links_branch
    FOREIGN KEY (branch_id) REFERENCES branches.branches(id);

-- gateway.api_keys.branch_id → branches.branches.id
ALTER TABLE gateway.api_keys
    ADD CONSTRAINT fk_api_keys_branch
    FOREIGN KEY (branch_id) REFERENCES branches.branches(id);

-- gateway.rate_limit_rules.branch_id → branches.branches.id
ALTER TABLE gateway.rate_limit_rules
    ADD CONSTRAINT fk_rate_limit_rules_branch
    FOREIGN KEY (branch_id) REFERENCES branches.branches(id);

-- gateway.request_logs.user_id → identity.users.id
ALTER TABLE gateway.request_logs
    ADD CONSTRAINT fk_request_logs_user
    FOREIGN KEY (user_id) REFERENCES identity.users(id);

-- gateway.request_logs.branch_id → branches.branches.id
ALTER TABLE gateway.request_logs
    ADD CONSTRAINT fk_request_logs_branch
    FOREIGN KEY (branch_id) REFERENCES branches.branches(id);

-- monitoring.monitoring_alerts.device_id → device.network_devices.id
ALTER TABLE monitoring.monitoring_alerts
    ADD CONSTRAINT fk_monitoring_alerts_device
    FOREIGN KEY (device_id) REFERENCES device.network_devices(id);

-- monitoring.monitoring_alerts.branch_id → branches.branches.id
ALTER TABLE monitoring.monitoring_alerts
    ADD CONSTRAINT fk_monitoring_alerts_branch
    FOREIGN KEY (branch_id) REFERENCES branches.branches(id);

-- monitoring.metric_records.device_id → device.network_devices.id
ALTER TABLE monitoring.metric_records
    ADD CONSTRAINT fk_metric_records_device
    FOREIGN KEY (device_id) REFERENCES device.network_devices(id);

-- monitoring.metric_records.branch_id → branches.branches.id
ALTER TABLE monitoring.metric_records
    ADD CONSTRAINT fk_metric_records_branch
    FOREIGN KEY (branch_id) REFERENCES branches.branches(id);

-- ============================================================
-- PART 2: Add missing composite indexes for common query patterns
-- ============================================================

-- Billing: customer + status (invoice listing)
CREATE INDEX IF NOT EXISTS idx_invoices_customer_status ON billing.invoices(customer_id, status);

-- Billing: customer + status (payment listing)
CREATE INDEX IF NOT EXISTS idx_payments_customer_status ON billing.payments(customer_id, status);

-- Subscription: customer + status (active subscription lookup)
CREATE INDEX IF NOT EXISTS idx_subscriptions_customer_status ON subscription.subscriptions(customer_id, status);

-- Ticket: customer + status (customer ticket history)
CREATE INDEX IF NOT EXISTS idx_tickets_customer_status ON ticket.tickets(customer_id, status);

-- Ticket: assigned_to + status (agent workload)
CREATE INDEX IF NOT EXISTS idx_tickets_assigned_status ON ticket.tickets(assigned_to, status);

-- PPPoE: subscription_id (session lookup by subscription)
CREATE INDEX IF NOT EXISTS idx_pppoe_sessions_subscription ON network.pppoe_sessions(subscription_id);

-- PPPoE: customer_id (session lookup by customer)
CREATE INDEX IF NOT EXISTS idx_pppoe_sessions_customer ON network.pppoe_sessions(customer_id);

-- MAC binding: customer_id (binding lookup by customer)
CREATE INDEX IF NOT EXISTS idx_mac_bindings_customer ON network.mac_bindings(customer_id);

-- MAC binding: subscription_id (binding lookup by subscription)
CREATE INDEX IF NOT EXISTS idx_mac_bindings_subscription ON network.mac_bindings(subscription_id);

-- Lead: branch + status (lead listing)
CREATE INDEX IF NOT EXISTS idx_leads_branch_status ON lead.leads(branch_id, status);

-- Notification: recipient_id + status (notification history)
CREATE INDEX IF NOT EXISTS idx_notifications_recipient_status ON notification.notifications(recipient_id, status);

-- Device: branch_id (device listing by branch)
CREATE INDEX IF NOT EXISTS idx_network_devices_branch ON device.network_devices(branch_id);

-- Audit log: user_id + created_at (user activity audit)
CREATE INDEX IF NOT EXISTS idx_audit_logs_user_time ON audit.audit_logs(user_id, created_at DESC);

-- Outbox: status + created_at (pending event processing)
CREATE INDEX IF NOT EXISTS idx_outbox_events_status_time ON outbox_events(status, created_at);

-- Refund: customer_id (refund lookup by customer)
CREATE INDEX IF NOT EXISTS idx_refunds_customer ON billing.refunds(customer_id);

-- Subscription: branch_id + status (branch subscription listing)
CREATE INDEX IF NOT EXISTS idx_subscriptions_branch_status ON subscription.subscriptions(branch_id, status);

-- Invoice: branch_id + status + due_date (branch billing dashboard)
CREATE INDEX IF NOT EXISTS idx_invoices_branch_status_due ON billing.invoices(branch_id, status, due_date);
