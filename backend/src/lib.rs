//! AeroXe Broadband ISP Platform - Backend Library
//!
//! Modular monolith with Domain-Driven Design architecture.

// Allow widespread clippy warnings at crate level
#![allow(clippy::too_many_arguments)]
#![allow(clippy::should_implement_trait)]

pub mod config;
pub mod infrastructure;
pub mod migration;
pub mod modules;
pub mod routes;
pub mod shared;
pub mod workers;

// Re-export commonly used types
pub use shared::app_state::AppState;
pub use shared::errors::AppError;

// Test infrastructure

