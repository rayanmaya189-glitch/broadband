//! HTTP interface — router assembly, health check, and route mounting.

pub mod health;
pub mod router;

pub use router::create_router;
