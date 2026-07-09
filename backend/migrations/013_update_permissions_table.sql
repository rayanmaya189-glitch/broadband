-- ═══════════════════════════════════════════════════════════════
-- Migration 013: Update permissions table with method, api_url, guard
-- ═══════════════════════════════════════════════════════════════

-- Drop the old UNIQUE constraint on `name` alone
-- (original schema defined: name VARCHAR(100) UNIQUE NOT NULL)
DO $$ BEGIN
    ALTER TABLE permissions DROP CONSTRAINT IF EXISTS permissions_name_key;
EXCEPTION WHEN undefined_object THEN NULL;
END $$;

-- Add new columns
ALTER TABLE permissions ADD COLUMN method VARCHAR(10) NOT NULL DEFAULT 'GET';
ALTER TABLE permissions ADD COLUMN api_url VARCHAR(500) NOT NULL DEFAULT '';
ALTER TABLE permissions ADD COLUMN guard VARCHAR(50) NOT NULL DEFAULT 'jwt';

-- Remove description column (no longer needed)
ALTER TABLE permissions DROP COLUMN IF EXISTS description;

-- Clean up old seed rows from migration 001 that have empty api_url
-- (they would conflict with the new seeder's entries by name)
DELETE FROM permissions WHERE api_url = '' OR api_url IS NULL;

-- Add composite unique constraint on (name, method, api_url) to prevent duplicates
ALTER TABLE permissions ADD CONSTRAINT uq_permissions_name_method_url UNIQUE (name, method, api_url);

-- Add indexes for faster lookups
CREATE INDEX IF NOT EXISTS idx_permissions_method ON permissions(method);
CREATE INDEX IF NOT EXISTS idx_permissions_guard ON permissions(guard);
