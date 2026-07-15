//! AeroXe Broadband ISP Platform - Backend Library
//!
//! Modular monolith with Domain-Driven Design architecture.

pub mod config;
pub mod shared;
pub mod infrastructure;
pub mod modules;
pub mod workers;
pub mod routes;

// Re-export commonly used types
pub use shared::errors::AppError;
pub use shared::app_state::AppState;
