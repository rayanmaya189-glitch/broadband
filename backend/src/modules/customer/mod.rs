//! Customer module.
//!
//! This module implements the Customer bounded context following DDD principles.
//!
//! # Architecture
//!
//! ## Domain Layer (`domain/`)
//! - **Aggregates**: Customer aggregate root with business rules
//! - **Value Objects**: Email, Phone, CustomerStatus, CustomerId
//! - **Rules**: Business rules for customer operations
//!
//! ## Application Layer (`application/`)
//! - **Commands**: CreateCustomer, UpdateCustomer, TransitionStatus, SubmitKyc, VerifyKyc
//! - **Queries**: GetCustomer, ListCustomers, GetProfile, ListKycDocuments, ListAddresses
//! - **Services**: CustomerApplicationService orchestrates domain operations
//!
//! ## Infrastructure Layer (`infrastructure/`)
//! - **Repository**: SeaORM implementation of CustomerRepositoryTrait
//! - **Messaging**: CustomerEventPublisher for NATS events
//!
//! ## API Layer (`api/`)
//! - **HTTP**: Axum controllers and routers
//!
//! ## Legacy (maintained for backward compatibility)
//! - `model/`: SeaORM entity models
//! - `repository/`: Legacy repository implementation
//! - `service/`: Legacy service implementation
//! - `controller/`: Legacy HTTP controllers

// DDD layers
pub mod domain;
pub mod application;
pub mod infrastructure;

// Legacy layers (maintained for backward compatibility)
pub mod controller;
pub mod model;
pub mod repository;
pub mod request;
pub mod response;
pub mod router;
pub mod service;
