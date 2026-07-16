//! Integration Adapters for External Systems
//!
//! This module contains adapters for integrating with external ISP infrastructure:
//! - MikroTik RouterOS (bandwidth control, queue management)
//! - Huawei OLT (GPON/XPON device management)
//! - SMS Providers (MSG91, Twilio for OTP and notifications)
//! - RADIUS (PPPoE authentication)

pub mod factory;
pub mod mikrotik;
pub mod huawei;
pub mod sms;
pub mod radius;

pub use factory::{DeviceAdapterFactory, DeviceType, NetworkDeviceAdapter};
