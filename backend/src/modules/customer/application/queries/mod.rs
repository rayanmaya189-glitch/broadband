//! Customer query handlers.
//!
//! Queries represent requests to read state without modifying it.

pub mod get_customer;
pub mod list_customers;
pub mod get_customer_profile;
pub mod list_kyc_documents;
pub mod list_addresses;

pub use get_customer::GetCustomerQuery;
pub use list_customers::ListCustomersQuery;
pub use get_customer_profile::GetCustomerProfileQuery;
pub use list_kyc_documents::ListKycDocumentsQuery;
pub use list_addresses::ListAddressesQuery;
