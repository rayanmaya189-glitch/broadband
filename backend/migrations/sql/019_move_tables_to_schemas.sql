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

-- Device tables reference customers/branches
ALTER TABLE IF EXISTS devices DROP CONSTRAINT IF EXISTS devices_customer_id_fkey;
ALTER TABLE IF EXISTS devices DROP CONSTRAINT IF EXISTS devices_branch_id_fkey;

-- Network tables reference branches
ALTER TABLE IF EXISTS network_equipment DROP CONSTRAINT IF EXISTS network_equipment_branch_id_fkey;

-- Bandwidth tables reference customers
ALTER TABLE IF EXISTS bandwidth_profiles DROP CONSTRAINT IF EXISTS bandwidth_profiles_customer_id_fkey;
ALTER TABLE IF EXISTS bandwidth_applications DROP CONSTRAINT IF EXISTS bandwidth_applications_customer_id_fkey;

-- Ticket tables
ALTER TABLE IF EXISTS tickets DROP CONSTRAINT IF EXISTS tickets_customer_id_fkey;
ALTER TABLE IF EXISTS ticket_messages DROP CONSTRAINT IF EXISTS ticket_messages_ticket_id_fkey;

-- Notification tables
ALTER TABLE IF EXISTS notifications DROP CONSTRAINT IF EXISTS notifications_user_id_fkey;
ALTER TABLE IF EXISTS notification_templates DROP CONSTRAINT IF EXISTS notification_templates_branch_id_fkey;

-- Audit tables reference users
ALTER TABLE IF EXISTS audit_logs DROP CONSTRAINT IF EXISTS audit_logs_user_id_fkey;

-- Workflow tables reference users
ALTER TABLE IF EXISTS approval_requests DROP CONSTRAINT IF EXISTS approval_requests_requested_by_fkey;
ALTER TABLE IF EXISTS approval_requests DROP CONSTRAINT IF EXISTS approval_requests_reviewed_by_fkey;

-- Document tables
ALTER TABLE IF EXISTS documents DROP CONSTRAINT IF EXISTS documents_uploaded_by_fkey;

-- Monitoring tables reference devices/branches
ALTER TABLE IF EXISTS monitoring_alerts DROP CONSTRAINT IF EXISTS monitoring_alerts_device_id_fkey;
ALTER TABLE IF EXISTS monitoring_alerts DROP CONSTRAINT IF EXISTS monitoring_alerts_branch_id_fkey;

-- ============================================================
-- Step 2: Move tables to their respective schemas
-- ============================================================

-- branches schema
ALTER TABLE IF EXISTS branches SET SCHEMA branches;
ALTER TABLE IF EXISTS branch_working_hours SET SCHEMA branches;
ALTER TABLE IF EXISTS user_branches SET SCHEMA branches;

-- identity schema (users, sessions, roles, permissions)
ALTER TABLE IF EXISTS users SET SCHEMA identity;
ALTER TABLE IF EXISTS user_sessions SET SCHEMA identity;
ALTER TABLE IF EXISTS roles SET SCHEMA identity;
ALTER TABLE IF EXISTS permissions SET SCHEMA identity;
ALTER TABLE IF EXISTS role_permissions SET SCHEMA identity;
ALTER TABLE IF EXISTS user_roles SET SCHEMA identity;
ALTER TABLE IF EXISTS permission_groups SET SCHEMA identity;
ALTER TABLE IF EXISTS permission_group_permissions SET SCHEMA identity;

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

-- device schema
ALTER TABLE IF EXISTS devices SET SCHEMA device;

-- network schema
ALTER TABLE IF EXISTS network_equipment SET SCHEMA network;

-- bandwidth schema
ALTER TABLE IF EXISTS bandwidth_profiles SET SCHEMA bandwidth;
ALTER TABLE IF EXISTS bandwidth_applications SET SCHEMA bandwidth;

-- ticket schema
ALTER TABLE IF EXISTS tickets SET SCHEMA ticket;
ALTER TABLE IF EXISTS ticket_messages SET SCHEMA ticket;

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

-- document schema
ALTER TABLE IF EXISTS documents SET SCHEMA document;

-- monitoring schema
ALTER TABLE IF EXISTS metric_records SET SCHEMA monitoring;
ALTER TABLE IF EXISTS alert_rules SET SCHEMA monitoring;
ALTER TABLE IF EXISTS monitoring_alerts SET SCHEMA monitoring;

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
