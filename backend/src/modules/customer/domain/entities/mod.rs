pub mod address;
pub mod customer;
pub mod customer_profile;
pub mod kyc_document;

pub use customer::ActiveModel as CustomerActiveModel;
pub use customer::Column as CustomerColumn;
pub use customer::Entity as Customer;

pub use address::ActiveModel as AddressActiveModel;
pub use address::Column as AddressColumn;
pub use address::Entity as Address;

pub use customer_profile::ActiveModel as CustomerProfileActiveModel;
pub use customer_profile::Entity as CustomerProfile;

pub use kyc_document::ActiveModel as KycDocumentActiveModel;
pub use kyc_document::Entity as KycDocument;
