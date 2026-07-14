//! Customer repository.
//!
//! Domain repository trait and SeaORM implementation.

pub mod customer_repository_trait;
pub mod seaorm_customer_repository;

pub use customer_repository_trait::CustomerRepositoryTrait;
pub use seaorm_customer_repository::SeaOrmCustomerRepository;
