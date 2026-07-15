pub mod discovery_scan;
pub mod discovery_result;

pub use discovery_scan::Entity as DiscoveryScan;
pub use discovery_scan::ActiveModel as DiscoveryScanActiveModel;

pub use discovery_result::Entity as DiscoveryResult;
pub use discovery_result::ActiveModel as DiscoveryResultActiveModel;
pub use discovery_result::Column as DiscoveryResultColumn;
