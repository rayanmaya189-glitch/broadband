-- Wallet topups and manual deposits are recorded without an invoice. Make
-- invoice_id nullable on both payment tables; the existing FKs still enforce
-- referential integrity whenever an invoice is present.
ALTER TABLE billing.payments ALTER COLUMN invoice_id DROP NOT NULL;
ALTER TABLE payment.payment_links ALTER COLUMN invoice_id DROP NOT NULL;
