//! Background workers for the AeroXe Broadband platform.
//!
//! Workers are long-running tasks that process events, sync data,
//! and perform background operations. They run as separate tokio tasks
//! with graceful shutdown support via CancellationToken.

pub mod device_sync_worker;
pub mod bandwidth_worker;
pub mod billing_worker;
pub mod notification_worker;
