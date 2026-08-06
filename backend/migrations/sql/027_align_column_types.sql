-- AeroXe Backend Migration 033: Align column types to SeaORM entity contracts
-- Entities model network identifiers as plain strings; DDL used inet/macaddr/cidr/time
-- types that reject text binds on INSERT/UPDATE. No code uses inet/cidr containment, so
-- converting to text is lossless for the app (canonical ::text form is preserved).

-- audit
ALTER TABLE audit.audit_logs ALTER COLUMN ip_address TYPE TEXT USING ip_address::text;

-- device
ALTER TABLE device.network_devices ALTER COLUMN management_ip TYPE TEXT USING management_ip::text;

-- discovery
ALTER TABLE discovery.discovery_results ALTER COLUMN discovered_ip TYPE TEXT USING discovered_ip::text;
ALTER TABLE discovery.discovery_results ALTER COLUMN discovered_mac TYPE TEXT USING discovered_mac::text;
ALTER TABLE discovery.discovery_scans ALTER COLUMN target_subnets TYPE JSONB USING to_jsonb(target_subnets);

-- identity
ALTER TABLE identity.user_sessions ALTER COLUMN ip_address TYPE TEXT USING ip_address::text;

-- network
ALTER TABLE network.dhcp_leases ALTER COLUMN ip_address TYPE TEXT USING ip_address::text;
ALTER TABLE network.dhcp_leases ALTER COLUMN mac_address TYPE TEXT USING mac_address::text;
ALTER TABLE network.ip_pools ALTER COLUMN cidr TYPE TEXT USING cidr::text;
ALTER TABLE network.ip_pools ALTER COLUMN dns_primary TYPE TEXT USING dns_primary::text;
ALTER TABLE network.ip_pools ALTER COLUMN dns_secondary TYPE TEXT USING dns_secondary::text;
ALTER TABLE network.ip_pools ALTER COLUMN gateway TYPE TEXT USING gateway::text;
ALTER TABLE network.mac_bindings ALTER COLUMN assigned_ip TYPE TEXT USING assigned_ip::text;
ALTER TABLE network.mac_bindings ALTER COLUMN mac_address TYPE TEXT USING mac_address::text;
ALTER TABLE network.pppoe_sessions ALTER COLUMN assigned_ip TYPE TEXT USING assigned_ip::text;
ALTER TABLE network.pppoe_sessions ALTER COLUMN nas_ip_address TYPE TEXT USING nas_ip_address::text;
ALTER TABLE network.pppoe_sessions ALTER COLUMN pppoe_server_ip TYPE TEXT USING pppoe_server_ip::text;

-- subscription
ALTER TABLE subscription.subscriptions ALTER COLUMN ip_address TYPE TEXT USING ip_address::text;
ALTER TABLE subscription.subscriptions ALTER COLUMN mac_address TYPE TEXT USING mac_address::text;

-- branches (working hours are handled as strings end-to-end)
ALTER TABLE branches.branch_working_hours ALTER COLUMN open_time TYPE TEXT USING open_time::text;
ALTER TABLE branches.branch_working_hours ALTER COLUMN close_time TYPE TEXT USING close_time::text;
