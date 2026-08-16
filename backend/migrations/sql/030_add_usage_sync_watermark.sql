-- AeroXe Backend Migration 039: usage-sync watermark on PPPoE sessions
--
-- The billing worker accumulated session bytes into the subscription on every
-- poll (`sub.bytes_used += session.bytes_in + session.bytes_out`), so a session
-- that stayed up across several polls was counted multiple times. These two
-- columns record how much of the session's counters have already been synced,
-- letting the worker add only the delta.

ALTER TABLE network.pppoe_sessions
    ADD COLUMN IF NOT EXISTS usage_synced_bytes_in BIGINT NOT NULL DEFAULT 0;

ALTER TABLE network.pppoe_sessions
    ADD COLUMN IF NOT EXISTS usage_synced_bytes_out BIGINT NOT NULL DEFAULT 0;
