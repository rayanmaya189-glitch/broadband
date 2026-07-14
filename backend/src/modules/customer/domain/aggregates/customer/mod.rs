//! Customer aggregate root.
//!
//! The Customer aggregate is the consistency boundary for all customer-related
//! operations. It enforces business invariants and publishes domain events.

pub mod customer;
pub mod tests;

pub use customer::Customer;
