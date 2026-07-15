pub mod device_port;
pub mod network_device;

pub use network_device::ActiveModel as NetworkDeviceActiveModel;
pub use network_device::Column as NetworkDeviceColumn;
pub use network_device::Entity as NetworkDevice;

pub use device_port::ActiveModel as DevicePortActiveModel;
pub use device_port::Column as DevicePortColumn;
pub use device_port::Entity as DevicePort;
