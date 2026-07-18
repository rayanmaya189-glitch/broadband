//! Integration Adapters for External Systems
//!
//! This module contains adapters for integrating with external ISP infrastructure:
//! - MikroTik RouterOS (bandwidth control, queue management)
//! - Huawei OLT (GPON/XPON device management)
//! - SMS Providers (MSG91, Twilio for OTP and notifications)
//! - RADIUS (PPPoE authentication)

pub mod factory;
pub mod huawei;
pub mod mikrotik;
pub mod push;
pub mod radius;
pub mod sms;
pub mod smtp;
pub mod whatsapp;

pub use factory::{DeviceAdapterFactory, DeviceType, NetworkDeviceAdapter};
