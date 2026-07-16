-- AeroXe Backend Migration 001: Create PostgreSQL Extensions
-- Required extensions for PostGIS, UUID generation, and crypto operations

-- Enable UUID extension for UUID generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Enable pgcrypto for encryption functions
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Enable PostGIS for spatial/geographic queries (coverage areas, device locations)
CREATE EXTENSION IF NOT EXISTS "postgis";

-- Enable pg_trgm for fuzzy text search (customer name search, etc.)
CREATE EXTENSION IF NOT EXISTS "pg_trgm";
