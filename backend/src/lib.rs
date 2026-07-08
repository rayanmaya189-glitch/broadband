//! AeroXe Backend — Modular Monolith
//!
//! A production-grade ISP platform backend built with Rust, Axum, and Tokio.

pub mod app;
pub mod config;
pub mod db;
pub mod error;

pub mod interfaces;
pub mod middleware;
pub mod modules;
pub mod services;
pub mod shared;
pub mod utils;
