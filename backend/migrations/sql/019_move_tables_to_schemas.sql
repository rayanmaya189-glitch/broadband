-- AeroXe Backend Migration 019: Move Tables to Per-Module Schemas
-- Moves all existing tables from public schema to their respective module schemas
-- WARNING: This migration is irreversible without backup
-- NOTE: No BEGIN/COMMIT - migration runner handles transaction

-- ============================================================
-- Step 1: Drop cross-schema foreign keys that will break
-- ============================================================

-- Customer tables reference branches.id
ALTER TABLE IF EXISTS customers DROP CONSTRAINT IF EXISTS customers_branch_id_fkey;
ALTER TABLE IF EXISTS installation_orders DROP CONSTRAINT IF EXISTS installation_orders_branch_id_fkey;

-- User tables reference branches.id
ALTER TABLE IF EXISTS user_branches DROP CONSTRAINT IF EXISTS user_branches_branch_id_fkey;
ALTER TABLE IF EXISTS users DROP CONSTRAINT IF EXISTS users_branch_id_fkey;

-- RBAC tables
ALTER TABLE IF EXISTS user_roles DROP CONSTRAINT IF EXISTS user_roles_user_id_fkey;
ALTER TABLE IF EXISTS user_roles DROP CONSTRAINT IF EXISTS user_roles_role_id_fkey;
ALTER TABLE IF EXISTS role_permissions DROP CONSTRAINT IF EXISTS role_permissions_role_id_fkey;
ALTER TABLE IF EXISTS role_permissions DROP CONSTRAINT IF EXISTS role_permissions_permission_id_fkey;
ALTER TABLE IF EXISTS user_sessions DROP CONSTRAINT IF EXISTS user_sessions_user_id_fkey;
ALTER TABLE IF EXISTS permission_group_permissions DROP CONSTRAINT IF EXISTS permission_group_permissions_group_id_fkey;
ALTER TABLE IF EXISTS permission_group_permissions DROP CONSTRAINT IF EXISTS permission_group_permissions_permission_id_fkey;
ALTER TABLE IF EXISTS approval_requests DROP CONSTRAINT IF EXISTS approval_requests_workflow_id_fkey;

-- Customer self-reference
ALTER TABLE IF EXISTS customers DROP CONSTRAINT IF EXISTS customers_referred_by_fkey;

-- Subscription tables reference customers
ALTER TABLE IF EXISTS subscriptions DROP CONSTRAINT IF EXISTS subscriptions_customer_id_fkey;
ALTER TABLE IF EXISTS subscriptions DROP CONSTRAINT IF EXISTS subscriptions_plan_id_fkey;

-- Billing tables reference subscriptions
ALTER TABLE IF EXISTS invoices DROP CONSTRAINT IF EXISTS invoices_subscription_id_fkey;
ALTER TABLE IF EXISTS invoice_line_items DROP CONSTRAINT IF EXISTS invoice_line_items_invoice_id_fkey;
ALTER TABLE IF EXISTS payments DROP CONSTRAINT IF EXISTS payments_invoice_id_fkey;
ALTER TABLE IF EXISTS refunds DROP CONSTRAINT IF EXISTS refunds_payment_id_fkey;
ALTER TABLE IF EXISTS payment_reminders DROP CONSTRAINT IF EXISTS payment_reminders_invoice_id_fkey;

-- Device tables reference customers/branches (table is network_devices, not devices)
ALTER TABLE IF EXISTS network_devices DROP CONSTRAINT IF EXISTS network_devices_branch_id_fkey;

-- Network tables reference branches (network_equipment doesn't exist as a separate table)

-- Bandwidth tables reference customers
ALTER TABLE IF EXISTS bandwidth_profiles DROP CONSTRAINT IF EXISTS bandwidth_profiles_customer_id_fkey;
ALTER TABLE IF EXISTS bandwidth_applications DROP CONSTRAINT IF EXISTS bandwidth_applications_customer_id_fkey;

-- Ticket tables
ALTER TABLE IF EXISTS tickets DROP CONSTRAINT IF EXISTS tickets_customer_id_fkey;
ALTER TABLE IF EXISTS ticket_comments DROP CONSTRAINT IF EXISTS ticket_comments_ticket_id_fkey;

-- Notification tables
ALTER TABLE IF EXISTS notifications DROP CONSTRAINT IF EXISTS notifications_user_id_fkey;
ALTER TABLE IF EXISTS notification_templates DROP CONSTRAINT IF EXISTS notification_templates_branch_id_fkey;

-- Audit tables reference users
ALTER TABLE IF EXISTS audit_logs DROP CONSTRAINT IF EXISTS audit_logs_user_id_fkey;

-- Workflow tables reference users
ALTER TABLE IF EXISTS approval_requests DROP CONSTRAINT IF EXISTS approval_requests_requested_by_fkey;
ALTER TABLE IF EXISTS approval_requests DROP CONSTRAINT IF EXISTS approval_requests_reviewed_by_fkey;

-- Document tables (table is document_files, not documents)
ALTER TABLE IF EXISTS document_files DROP CONSTRAINT IF EXISTS document_files_uploaded_by_fkey;

-- ============================================================
-- Step 2: Move tables to their respective schemas
-- ============================================================

-- branches schema
ALTER TABLE IF EXISTS branches SET SCHEMA branches;
ALTER TABLE IF EXISTS branch_working_hours SET SCHEMA branches;
ALTER TABLE IF EXISTS user_branches SET SCHEMA branches;

-- identity schema (users, sessions)
ALTER TABLE IF EXISTS users SET SCHEMA identity;
ALTER TABLE IF EXISTS user_sessions SET SCHEMA identity;
ALTER TABLE IF EXISTS permission_groups SET SCHEMA identity;
ALTER TABLE IF EXISTS permission_group_permissions SET SCHEMA identity;

-- security schema (RBAC entities declare schema_name = "security")
ALTER TABLE IF EXISTS roles SET SCHEMA security;
ALTER TABLE IF EXISTS permissions SET SCHEMA security;
ALTER TABLE IF EXISTS role_permissions SET SCHEMA security;
ALTER TABLE IF EXISTS user_roles SET SCHEMA security;

-- customer schema
ALTER TABLE IF EXISTS customers SET SCHEMA customer;
ALTER TABLE IF EXISTS customer_profiles SET SCHEMA customer;
ALTER TABLE IF EXISTS addresses SET SCHEMA customer;
ALTER TABLE IF EXISTS kyc_documents SET SCHEMA customer;
ALTER TABLE IF EXISTS customers_history SET SCHEMA customer;

-- subscription schema
ALTER TABLE IF EXISTS subscriptions SET SCHEMA subscription;

-- plans schema
ALTER TABLE IF EXISTS plans SET SCHEMA plans;

-- billing schema
ALTER TABLE IF EXISTS invoices SET SCHEMA billing;
ALTER TABLE IF EXISTS invoice_line_items SET SCHEMA billing;
ALTER TABLE IF EXISTS payments SET SCHEMA billing;
ALTER TABLE IF EXISTS refunds SET SCHEMA billing;
ALTER TABLE IF EXISTS payment_reminders SET SCHEMA billing;

-- device schema (table is network_devices)
ALTER TABLE IF EXISTS network_devices SET SCHEMA device;

-- bandwidth schema
ALTER TABLE IF EXISTS bandwidth_profiles SET SCHEMA bandwidth;
ALTER TABLE IF EXISTS bandwidth_applications SET SCHEMA bandwidth;

-- ticket schema (table is ticket_comments, not ticket_messages)
ALTER TABLE IF EXISTS tickets SET SCHEMA ticket;
ALTER TABLE IF EXISTS ticket_comments SET SCHEMA ticket;

-- notification schema
ALTER TABLE IF EXISTS notifications SET SCHEMA notification;
ALTER TABLE IF EXISTS notification_templates SET SCHEMA notification;

-- audit schema
ALTER TABLE IF EXISTS audit_logs SET SCHEMA audit;

-- workflow schema
ALTER TABLE IF EXISTS approval_workflows SET SCHEMA workflow;
ALTER TABLE IF EXISTS approval_requests SET SCHEMA workflow;

-- coverage schema
ALTER TABLE IF EXISTS coverage_areas SET SCHEMA coverage;
ALTER TABLE IF EXISTS coverage_pincode_map SET SCHEMA coverage;

-- lead schema
ALTER TABLE IF EXISTS leads SET SCHEMA lead;
ALTER TABLE IF EXISTS lead_activities SET SCHEMA lead;

-- installation schema
ALTER TABLE IF EXISTS installation_orders SET SCHEMA installation;

-- document schema (table is document_files, not documents)
ALTER TABLE IF EXISTS document_files SET SCHEMA document;
ALTER TABLE IF EXISTS document_access_logs SET SCHEMA document;

-- network schema (migration 010 tables)
ALTER TABLE IF EXISTS vlans SET SCHEMA network;
ALTER TABLE IF EXISTS vlans_history SET SCHEMA network;
ALTER TABLE IF EXISTS ip_pools SET SCHEMA network;
ALTER TABLE IF EXISTS ip_pools_history SET SCHEMA network;
ALTER TABLE IF EXISTS ip_addresses SET SCHEMA network;
ALTER TABLE IF EXISTS pppoe_sessions SET SCHEMA network;
ALTER TABLE IF EXISTS pppoe_sessions_history SET SCHEMA network;
ALTER TABLE IF EXISTS dhcp_leases SET SCHEMA network;
ALTER TABLE IF EXISTS mac_bindings SET SCHEMA network;
ALTER TABLE IF EXISTS customer_sessions SET SCHEMA network;

-- device schema (migration 009 tables)
ALTER TABLE IF EXISTS device_models SET SCHEMA device;
ALTER TABLE IF EXISTS network_devices_history SET SCHEMA device;
ALTER TABLE IF EXISTS device_ports SET SCHEMA device;
ALTER TABLE IF EXISTS device_logs SET SCHEMA device;
ALTER TABLE IF EXISTS device_metrics SET SCHEMA device;
ALTER TABLE IF EXISTS firmware_updates SET SCHEMA device;

-- discovery schema (migration 009 tables)
ALTER TABLE IF EXISTS discovery_scans SET SCHEMA discovery;
ALTER TABLE IF EXISTS discovery_results SET SCHEMA discovery;
ALTER TABLE IF EXISTS discovery_scan_history SET SCHEMA discovery;
ALTER TABLE IF EXISTS subnet_location_map SET SCHEMA discovery;

-- inventory schema (migration 009 tables)
ALTER TABLE IF EXISTS inventory_items SET SCHEMA inventory;
ALTER TABLE IF EXISTS inventory_movements SET SCHEMA inventory;

-- accounting schema (migration 008 tables)
ALTER TABLE IF EXISTS chart_of_accounts SET SCHEMA accounting;
ALTER TABLE IF EXISTS journal_entries SET SCHEMA accounting;
ALTER TABLE IF EXISTS journal_entry_lines SET SCHEMA accounting;
ALTER TABLE IF EXISTS trial_balances SET SCHEMA accounting;
ALTER TABLE IF EXISTS gst_returns SET SCHEMA accounting;

-- plans schema (migration 005 tables)
ALTER TABLE IF EXISTS plans_history SET SCHEMA plans;
ALTER TABLE IF EXISTS plan_pricing SET SCHEMA plans;
ALTER TABLE IF EXISTS speed_profiles SET SCHEMA plans;
ALTER TABLE IF EXISTS service_packages SET SCHEMA plans;
ALTER TABLE IF EXISTS plan_service_packages SET SCHEMA plans;
ALTER TABLE IF EXISTS bandwidth_profiles_history SET SCHEMA plans;

-- subscription schema (migration 006 tables)
ALTER TABLE IF EXISTS subscriptions_history SET SCHEMA subscription;
ALTER TABLE IF EXISTS service_accounts SET SCHEMA subscription;

-- billing schema (migration 007 tables)
ALTER TABLE IF EXISTS discounts SET SCHEMA billing;
ALTER TABLE IF EXISTS invoices_history SET SCHEMA billing;
ALTER TABLE IF EXISTS refunds_history SET SCHEMA billing;
ALTER TABLE IF EXISTS discounts_history SET SCHEMA billing;

-- ticket schema (migration 011 tables)
ALTER TABLE IF EXISTS ticket_escalations SET SCHEMA ticket;
ALTER TABLE IF EXISTS ticket_attachments SET SCHEMA ticket;
ALTER TABLE IF EXISTS ticket_status_history SET SCHEMA ticket;
ALTER TABLE IF EXISTS tickets_history SET SCHEMA ticket;

-- notification schema (migration 012 tables)
ALTER TABLE IF EXISTS notification_channels SET SCHEMA notification;
ALTER TABLE IF EXISTS notification_history SET SCHEMA notification;

-- events stay in the public schema (outbox_events entity is schema-less;
-- events / event_subscriptions have no entities).

-- ============================================================
-- Step 2b: DEFAULT partitions for monthly-partitioned tables
-- ============================================================
-- Partitioned tables only ship a 2026_07 partition. Add a DEFAULT
-- partition so inserts at any later timestamp succeed.
CREATE TABLE IF NOT EXISTS audit.audit_logs_default PARTITION OF audit.audit_logs DEFAULT;
CREATE TABLE IF NOT EXISTS device.device_logs_default PARTITION OF device.device_logs DEFAULT;
CREATE TABLE IF NOT EXISTS device.device_metrics_default PARTITION OF device.device_metrics DEFAULT;

-- NOTE: monitoring tables (alert_rules, metric_records, monitoring_alerts)
-- are created in migration 020_create_monitoring_tables.sql and do not exist
-- at the time this migration runs. Skip them here.

-- ============================================================
-- Step 3: Grant permissions to application user
-- ============================================================

DO $$
DECLARE
    schema_name TEXT;
BEGIN
    FOR schema_name IN
        SELECT unnest(ARRAY[
            'identity', 'customer', 'subscription', 'billing', 'payment',
            'network', 'device', 'bandwidth', 'branches', 'plans',
            'audit', 'compliance', 'ticket', 'notification', 'coverage',
            'discovery', 'document', 'inventory', 'installation', 'lead',
            'referral', 'gateway', 'security', 'workflow', 'accounting',
            'scheduler', 'monitoring', 'integrations'
        ])
    LOOP
        EXECUTE format('GRANT USAGE ON SCHEMA %I TO CURRENT_USER', schema_name);
        EXECUTE format('GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA %I TO CURRENT_USER', schema_name);
        EXECUTE format('GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA %I TO CURRENT_USER', schema_name);
        EXECUTE format('ALTER DEFAULT PRIVILEGES IN SCHEMA %I GRANT ALL ON TABLES TO CURRENT_USER', schema_name);
        EXECUTE format('ALTER DEFAULT PRIVILEGES IN SCHEMA %I GRANT ALL ON SEQUENCES TO CURRENT_USER', schema_name);
    END LOOP;
END $$;
