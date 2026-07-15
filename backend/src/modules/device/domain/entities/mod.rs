pub mod network_device;
pub mod device_port;

pub use network_device::Entity as NetworkDevice;
pub use network_device::ActiveModel as NetworkDeviceActiveModel;
pub use network_device::Column as NetworkDeviceColumn;

pub use device_port::Entity as DevicePort;
pub use device_port::ActiveModel as DevicePortActiveModel;
pub use device_port::Column as DevicePortColumn;
