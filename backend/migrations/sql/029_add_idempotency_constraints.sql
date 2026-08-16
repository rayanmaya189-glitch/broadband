-- AeroXe Backend Migration 038: DB-backed idempotency constraints
-- Prevent double-processing from concurrent gateway webhook replays, the
-- auto-billing job, and duplicate invoice generation:
--   1. billing.payments(gateway_transaction_id)       UNIQUE
--   2. billing.invoices(subscription_id, billing_period_start) UNIQUE
--   3. payment.webhook_logs(gateway_id, event_id)      UNIQUE

-- Collapse any pre-existing duplicate payments, keeping the earliest row and
-- never deleting rows that a refund already references.
DELETE FROM billing.payments a
USING billing.payments b
WHERE a.id > b.id
  AND a.gateway_transaction_id IS NOT NULL
  AND a.gateway_transaction_id = b.gateway_transaction_id
  AND NOT EXISTS (SELECT 1 FROM billing.refunds r WHERE r.payment_id = a.id);

-- The old non-unique index is replaced by the unique one.
DROP INDEX IF EXISTS billing.idx_payments_gateway_tx;
CREATE UNIQUE INDEX IF NOT EXISTS uq_payments_gateway_transaction_id
    ON billing.payments(gateway_transaction_id);

-- Collapse duplicate invoices for the same subscription/period, keeping the
-- earliest row and skipping rows referenced by downstream records.
DELETE FROM billing.invoices a
USING billing.invoices b
WHERE a.id > b.id
  AND a.subscription_id = b.subscription_id
  AND a.billing_period_start = b.billing_period_start
  AND NOT EXISTS (SELECT 1 FROM billing.refunds r WHERE r.invoice_id = a.id)
  AND NOT EXISTS (SELECT 1 FROM billing.payment_reminders r WHERE r.invoice_id = a.id)
  AND NOT EXISTS (SELECT 1 FROM payment.payment_links l WHERE l.invoice_id = a.id);

CREATE UNIQUE INDEX IF NOT EXISTS uq_invoices_subscription_period
    ON billing.invoices(subscription_id, billing_period_start);

-- Collapse duplicate webhook logs for the same gateway event.
DELETE FROM payment.webhook_logs a
USING payment.webhook_logs b
WHERE a.id > b.id
  AND a.gateway_id = b.gateway_id
  AND a.event_id = b.event_id;

-- The old prefix index on gateway_id is redundant with the unique composite.
DROP INDEX IF EXISTS payment.idx_webhook_logs_gateway;
CREATE UNIQUE INDEX IF NOT EXISTS uq_webhook_logs_gateway_event
    ON payment.webhook_logs(gateway_id, event_id);
