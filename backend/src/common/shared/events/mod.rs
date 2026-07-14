//! Shared event type definitions.
//!
//! These structs represent the payload of NATS events published across modules.
//! They provide type safety and documentation for the event-driven architecture.

pub mod customer_events;
pub mod billing_events;
pub mod network_events;
