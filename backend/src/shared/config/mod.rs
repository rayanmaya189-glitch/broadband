//! Shared configuration constants.
//!
//! Kept as plain constants rather than a DB-backed table for now; the billing
//! worker and the DunningConfig API response must read from the same source so
//! the actual suspension behaviour matches what the API reports.

pub mod dunning;
