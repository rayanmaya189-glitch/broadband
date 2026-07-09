-- ═══════════════════════════════════════════════════════════════
-- AeroXe ISP Platform — Dunning State Column
-- ═══════════════════════════════════════════════════════════════

-- Add dedicated dunning columns to invoices table so dunning state
-- is no longer stored in the generic notes field.
ALTER TABLE invoices ADD COLUMN dunning_stage VARCHAR(30) DEFAULT NULL;
ALTER TABLE invoices ADD COLUMN dunning_notified_at TIMESTAMPTZ DEFAULT NULL;

CREATE INDEX idx_invoices_dunning ON invoices(dunning_stage) WHERE dunning_stage IS NOT NULL;
