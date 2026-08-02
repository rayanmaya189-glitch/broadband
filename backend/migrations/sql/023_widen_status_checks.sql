-- Widen billing.invoices.status to accept 'voided', the canonical spelling
-- used by void_invoice(), the payment settle guards, and the invoice.voided
-- event. Keep 'void' for any legacy rows written before the spelling was
-- unified.
ALTER TABLE billing.invoices DROP CONSTRAINT IF EXISTS invoices_status_check;
ALTER TABLE billing.invoices ADD CONSTRAINT invoices_status_check CHECK (
    status IN ('draft', 'pending', 'sent', 'paid', 'partial', 'overdue', 'void', 'voided', 'refunded')
);

-- Add 'terminated' to the subscription lifecycle so the customer-termination
-- cleanup flow can mark active/suspended subscriptions as terminated.
ALTER TABLE subscription.subscriptions DROP CONSTRAINT IF EXISTS subscriptions_status_check;
ALTER TABLE subscription.subscriptions ADD CONSTRAINT subscriptions_status_check CHECK (
    status IN ('active', 'suspended', 'cancelled', 'expired', 'terminated')
);
