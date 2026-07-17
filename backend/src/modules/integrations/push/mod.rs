//! Push Notification Integration Module
//!
//! Supports FCM (Firebase Cloud Messaging) for Android and web push notifications:
//! - Single device messaging
//! - Topic-based messaging
//! - Priority and TTL configuration
//! - Delivery status tracking

pub mod fcm;

pub use fcm::{FcmAdapter, FcmConfig, PushDeliveryStatus};
