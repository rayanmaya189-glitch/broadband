-- Migration: Add GST fields to branches and invoices
-- Date: 2026-07-13
-- Description: Add branch-level GSTIN, and per-invoice CGST/SGST/IGST breakdown

-- Add GSTIN to branches (each branch configures its own GSTIN for proper GST calculation)
ALTER TABLE branches ADD COLUMN IF NOT EXISTS gstin VARCHAR(15);

-- Add CGST/SGST/IGST columns to invoices for proper GST breakdown
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS cgst_amount NUMERIC(12,2) NOT NULL DEFAULT 0;
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS sgst_amount NUMERIC(12,2) NOT NULL DEFAULT 0;
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS igst_amount NUMERIC(12,2) NOT NULL DEFAULT 0;
