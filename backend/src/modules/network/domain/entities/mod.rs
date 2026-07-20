pub mod dhcp_lease;
pub mod ip_pool;
pub mod mac_binding;
pub mod pppoe_session;
pub mod vlan;

pub use dhcp_lease::ActiveModel as DhcpLeaseActiveModel;
pub use dhcp_lease::Column as DhcpLeaseColumn;
pub use dhcp_lease::Entity as DhcpLease;

pub use vlan::ActiveModel as VlanActiveModel;
pub use vlan::Column as VlanColumn;
pub use vlan::Entity as Vlan;

pub use ip_pool::ActiveModel as IpPoolActiveModel;
pub use ip_pool::Column as IpPoolColumn;
pub use ip_pool::Entity as IpPool;

pub use pppoe_session::ActiveModel as PppoeSessionActiveModel;
pub use pppoe_session::Column as PppoeSessionColumn;
pub use pppoe_session::Entity as PppoeSession;

pub use mac_binding::ActiveModel as MacBindingActiveModel;
pub use mac_binding::Column as MacBindingColumn;
pub use mac_binding::Entity as MacBinding;
