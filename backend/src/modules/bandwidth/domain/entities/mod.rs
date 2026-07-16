pub mod bandwidth_application;
pub mod bandwidth_profile;

pub use bandwidth_application::ActiveModel as BandwidthApplicationActiveModel;
pub use bandwidth_application::Entity as BandwidthApplication;

pub use bandwidth_profile::ActiveModel as BandwidthProfileActiveModel;
pub use bandwidth_profile::Column as BandwidthProfileColumn;
pub use bandwidth_profile::Entity as BandwidthProfile;
