//! Compliance module.
//!
//! Handles KYC verification, GDPR consent, data retention policies,
//! and privacy rules. Listens to relevant events and triggers compliance checks.
//!
//! # Database Schema
//! `compliance` (tables: `kyc_verifications`, `consents`, `data_retention_policies`)
//!
//! # Architecture
//! - **Domain**: Consent aggregate, KYC aggregate, data retention value objects
//! - **Application**: Commands (verify_kyc, manage_consent), Services (retention_service)
//! - **Infrastructure**: Repository implementations, KYC provider adapters

pub mod domain;
pub mod application;
pub mod infrastructure;
