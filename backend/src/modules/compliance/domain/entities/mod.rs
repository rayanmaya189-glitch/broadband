pub mod kyc_verification;
pub mod consent;
pub mod data_retention_policy;

pub use kyc_verification::Entity as KycVerification;
pub use kyc_verification::ActiveModel as KycVerificationActiveModel;

pub use consent::Entity as Consent;
pub use consent::ActiveModel as ConsentActiveModel;

pub use data_retention_policy::Entity as DataRetentionPolicy;
pub use data_retention_policy::ActiveModel as DataRetentionPolicyActiveModel;
