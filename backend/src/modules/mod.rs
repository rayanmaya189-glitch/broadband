//! Domain modules — each module is an isolated bounded context.
//!
//! Module structure per domain:
//!   <module>/
//!     mod.rs          — public re-exports
//!     domain/         — entities, value objects, aggregates, repository traits
//!     application/    — service layer (use cases)
//!     infrastructure/ — repository implementations, external adapters
//!     interfaces/     — HTTP handlers, DTOs, request/response types

pub mod auth;
pub mod billing;
pub mod bandwidth;
pub mod branches;
pub mod customers;
pub mod plans;
pub mod subscriptions;
pub mod users;

// Stub modules — to be implemented in subsequent phases
pub mod accounting;
pub mod coverage;
pub mod devices;
pub mod discovery;
pub mod documents;
pub mod events;
pub mod installations;
pub mod inventory;
pub mod leads;
pub mod network;
pub mod notifications;
pub mod payment_gateway;
pub mod referrals;
pub mod realtime;
pub mod tickets;
pub mod audit;
