//! Database layer — PostgreSQL connection pool and migration runner.

pub mod connection;
pub mod migrations;

pub use connection::{DatabasePool, new_pool};
