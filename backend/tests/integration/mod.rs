//! Repository/integration tests. These are compiled into the `integration`
//! test target via `tests/integration.rs` and require Docker to run
//! (ignored by default).

pub mod accounting_repository_test;
pub mod audit_repository_test;
pub mod billing_repository_test;
pub mod customer_repository_test;
pub mod device_repository_test;
pub mod identity_repository_test;
pub mod network_repository_test;
pub mod referral_reward_test;
pub mod security_abuse_tests;
pub mod security_jwt_tests;
pub mod security_repository_test;
pub mod subscription_repository_test;
pub mod ticket_repository_test;
