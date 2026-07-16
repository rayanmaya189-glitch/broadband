//! RADIUS Integration Module
//!
//! Supports RADIUS (Remote Authentication Dial-In User Service) for:
//! - PPPoE customer authentication
//! - Accounting (session tracking)
//! - CoA (Change of Authorization) for dynamic bandwidth changes
//!
//! Protocol: RFC 2865 (Authentication), RFC 2866 (Accounting)

pub mod adapter;

pub use adapter::{RadiusAdapter, RadiusConfig, RadiusRequest, RadiusResponse, RadiusAttribute};
