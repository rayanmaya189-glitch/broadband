//! Notification delivery services.
//!
//! Each channel has its own delivery implementation backed by real APIs:
//! - `telegram` — Telegram Bot API for push-style messages
//! - `whatsapp` — WhatsApp Business API v23 for rich media messages
//! - `email` — SMTP via `lettre` for email delivery with OTP support
//!
//! The `orchestrator` module picks up queued notifications from the database
//! and routes them to the correct channel delivery service.

pub mod email;
pub mod orchestrator;
pub mod telegram;
pub mod whatsapp;

pub use orchestrator::NotificationOrchestrator;
