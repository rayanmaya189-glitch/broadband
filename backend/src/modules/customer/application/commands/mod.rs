//! Customer command handlers.
//!
//! Commands represent user意图 (intent) to change state.

pub mod create_customer;
pub mod update_customer;
pub mod transition_customer_status;
pub mod submit_kyc;
pub mod verify_kyc;

pub use create_customer::CreateCustomerHandler;
pub use update_customer::UpdateCustomerHandler;
pub use transition_customer_status::TransitionCustomerStatusHandler;
pub use submit_kyc::SubmitKycHandler;
pub use verify_kyc::VerifyKycHandler;
