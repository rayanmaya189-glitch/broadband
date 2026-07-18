pub mod consent;
pub mod data_retention_policy;
pub mod kyc_verification;

pub use kyc_verification::ActiveModel as KycVerificationActiveModel;
pub use kyc_verification::Entity as KycVerification;

pub use consent::ActiveModel as ConsentActiveModel;
pub use consent::Entity as Consent;

pub use data_retention_policy::ActiveModel as DataRetentionPolicyActiveModel;
pub use data_retention_policy::Entity as DataRetentionPolicy;
