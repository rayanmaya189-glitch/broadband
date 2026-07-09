-- ═══════════════════════════════════════════════════════════════
-- AeroXe ISP Platform — Row-Level Security (RLS) Policies
-- ═══════════════════════════════════════════════════════════════
--
-- RLS is enforced at the database level via PostgreSQL session variables
-- set by the middleware: app.current_branch_id and app.is_company_wide.
--
-- Branch-scoped tables (per §5-branches.md):
--   customers, subscriptions, tickets, leads, invoices, payments,
--   refunds, network_devices, vlans, ip_pools, pppoe_sessions,
--   coverage_areas, installation_orders, inventory_items,
--   mac_bindings, dhcp_leases, customer_sessions, discovery_scans,
--   events, discount_usage
--
-- Company-wide tables (no RLS):
--   users, roles, permissions, plans, speed_profiles, bandwidth_profiles,
--   notification_templates, document_files, audit_logs

-- ── Helper function ──────────────────────────────────────────
-- Checks if the current user belongs to the row's branch.
-- Returns TRUE if:
--   1. The user is company-wide (is_company_wide = true), OR
--   2. The row's branch_id matches the user's branch_id
CREATE OR REPLACE FUNCTION fn_branch_scope()
RETURNS BOOLEAN AS $$
BEGIN
    IF current_setting('app.is_company_wide', true)::BOOLEAN = TRUE THEN
        RETURN TRUE;
    END IF;
    RETURN branch_id = current_setting('app.current_branch_id', true)::BIGINT;
END;
$$ LANGUAGE plpgsql STABLE;

-- ── Customers ────────────────────────────────────────────────
ALTER TABLE customers ENABLE ROW LEVEL SECURITY;
CREATE POLICY branch_scope_customers ON customers
    FOR ALL
    USING (fn_branch_scope());

-- ── Subscriptions ────────────────────────────────────────────
ALTER TABLE subscriptions ENABLE ROW LEVEL SECURITY;
CREATE POLICY branch_scope_subscriptions ON subscriptions
    FOR ALL
    USING (fn_branch_scope());

-- ── Tickets ──────────────────────────────────────────────────
ALTER TABLE tickets ENABLE ROW LEVEL SECURITY;
CREATE POLICY branch_scope_tickets ON tickets
    FOR ALL
    USING (fn_branch_scope());

-- ── Leads ────────────────────────────────────────────────────
ALTER TABLE leads ENABLE ROW LEVEL SECURITY;
CREATE POLICY branch_scope_leads ON leads
    FOR ALL
    USING (fn_branch_scope());

-- ── Invoices ─────────────────────────────────────────────────
ALTER TABLE invoices ENABLE ROW LEVEL SECURITY;
CREATE POLICY branch_scope_invoices ON invoices
    FOR ALL
    USING (fn_branch_scope());

-- ── Payments ─────────────────────────────────────────────────
ALTER TABLE payments ENABLE ROW LEVEL SECURITY;
CREATE POLICY branch_scope_payments ON payments
    FOR ALL
    USING (fn_branch_scope());

-- ── Refunds ──────────────────────────────────────────────────
ALTER TABLE refunds ENABLE ROW LEVEL SECURITY;
CREATE POLICY branch_scope_refunds ON refunds
    FOR ALL
    USING (fn_branch_scope());

-- ── Network Devices ──────────────────────────────────────────
ALTER TABLE network_devices ENABLE ROW LEVEL SECURITY;
CREATE POLICY branch_scope_network_devices ON network_devices
    FOR ALL
    USING (fn_branch_scope());

-- ── VLANs ────────────────────────────────────────────────────
ALTER TABLE vlans ENABLE ROW LEVEL SECURITY;
CREATE POLICY branch_scope_vlans ON vlans
    FOR ALL
    USING (fn_branch_scope());

-- ── IP Pools ─────────────────────────────────────────────────
ALTER TABLE ip_pools ENABLE ROW LEVEL SECURITY;
CREATE POLICY branch_scope_ip_pools ON ip_pools
    FOR ALL
    USING (fn_branch_scope());

-- ── PPPoE Sessions ──────────────────────────────────────────
ALTER TABLE pppoe_sessions ENABLE ROW LEVEL SECURITY;
CREATE POLICY branch_scope_pppoe_sessions ON pppoe_sessions
    FOR ALL
    USING (fn_branch_scope());

-- ── Coverage Areas ───────────────────────────────────────────
ALTER TABLE coverage_areas ENABLE ROW LEVEL SECURITY;
CREATE POLICY branch_scope_coverage_areas ON coverage_areas
    FOR ALL
    USING (fn_branch_scope());

-- ── Installation Orders ──────────────────────────────────────
ALTER TABLE installation_orders ENABLE ROW LEVEL SECURITY;
CREATE POLICY branch_scope_installation_orders ON installation_orders
    FOR ALL
    USING (fn_branch_scope());

-- ── Inventory Items ──────────────────────────────────────────
ALTER TABLE inventory_items ENABLE ROW LEVEL SECURITY;
CREATE POLICY branch_scope_inventory_items ON inventory_items
    FOR ALL
    USING (fn_branch_scope());

-- ── MAC Bindings ─────────────────────────────────────────────
ALTER TABLE mac_bindings ENABLE ROW LEVEL SECURITY;
CREATE POLICY branch_scope_mac_bindings ON mac_bindings
    FOR ALL
    USING (fn_branch_scope());

-- ── DHCP Leases ──────────────────────────────────────────────
ALTER TABLE dhcp_leases ENABLE ROW LEVEL SECURITY;
CREATE POLICY branch_scope_dhcp_leases ON dhcp_leases
    FOR ALL
    USING (fn_branch_scope());

-- ── Customer Sessions ────────────────────────────────────────
ALTER TABLE customer_sessions ENABLE ROW LEVEL SECURITY;
CREATE POLICY branch_scope_customer_sessions ON customer_sessions
    FOR ALL
    USING (fn_branch_scope());

-- ── Discovery Scans ──────────────────────────────────────────
ALTER TABLE discovery_scans ENABLE ROW LEVEL SECURITY;
CREATE POLICY branch_scope_discovery_scans ON discovery_scans
    FOR ALL
    USING (fn_branch_scope());

-- ── Events ───────────────────────────────────────────────────
-- Events use caused_by_branch_id instead of branch_id
CREATE OR REPLACE FUNCTION fn_branch_scope_events()
RETURNS BOOLEAN AS $$
BEGIN
    IF current_setting('app.is_company_wide', true)::BOOLEAN = TRUE THEN
        RETURN TRUE;
    END IF;
    RETURN caused_by_branch_id = current_setting('app.current_branch_id', true)::BIGINT;
END;
$$ LANGUAGE plpgsql STABLE;

ALTER TABLE events ENABLE ROW LEVEL SECURITY;
CREATE POLICY branch_scope_events ON events
    FOR ALL
    USING (fn_branch_scope_events());

-- ── Grant access to the application role ─────────────────────
-- The application connects as a single DB user. RLS policies
-- apply to ALL queries from that role, which is exactly what
-- we want — the middleware sets the session variables per-request.
