//! Shared event type definitions.
//!
//! These structs represent the payload of NATS events published across modules.
//! They provide type safety and documentation for the event-driven architecture.
//!
//! # Event Architecture
//!
//! All cross-module communication happens over versioned domain events published to NATS.
//! Every event is wrapped in an `EventEnvelope<T>` which carries metadata like event_id,
//! version, occurred_at, and producer.
//!
//! ## NATS Subject Naming Convention
//! Format: `aeroxe.<context>.<entity>.<action>.<version>`
//! Example: `aeroxe.customer.created.v1`

pub mod envelope;
pub mod customer_events;
pub mod billing_events;
pub mod network_events;

pub use envelope::{DomainEvent, EventEnvelope};
