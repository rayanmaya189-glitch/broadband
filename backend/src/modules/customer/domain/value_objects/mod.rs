//! Value objects for the customer domain.
//!
//! Value objects are immutable and compared by value, not identity.

pub mod email;
pub mod phone;
pub mod customer_status;
pub mod customer_id;

pub use email::Email;
pub use phone::Phone;
pub use customer_status::CustomerStatus;
pub use customer_id::CustomerId;
