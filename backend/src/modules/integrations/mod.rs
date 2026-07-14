//! Integrations module.
//!
//! Adapters to external ISP systems: Mikrotik, Huawei, RADIUS,
//! payment gateways, and SMS providers.
//!
//! # Architecture
//! Each integration is a separate adapter that implements a common trait.
//! The adapter pattern allows swapping implementations without changing business logic.

pub mod mikrotik;
pub mod huawei;
pub mod radius;
pub mod payment_gateway;
pub mod sms_provider;
