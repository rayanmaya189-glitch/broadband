pub mod device_log;
pub mod device_metric;
pub mod device_port;
pub mod network_device;

pub use network_device::ActiveModel as NetworkDeviceActiveModel;
pub use network_device::Column as NetworkDeviceColumn;
pub use network_device::Entity as NetworkDevice;

pub use device_port::ActiveModel as DevicePortActiveModel;
pub use device_port::Column as DevicePortColumn;
pub use device_port::Entity as DevicePort;

pub use device_log::ActiveModel as DeviceLogActiveModel;
pub use device_log::Column as DeviceLogColumn;
pub use device_log::Entity as DeviceLog;

pub use device_metric::ActiveModel as DeviceMetricActiveModel;
pub use device_metric::Column as DeviceMetricColumn;
pub use device_metric::Entity as DeviceMetric;
