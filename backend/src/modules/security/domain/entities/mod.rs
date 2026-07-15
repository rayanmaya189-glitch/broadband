pub mod permission;
pub mod role;
pub mod role_permission;
pub mod user_role;

pub use role::ActiveModel as RoleActiveModel;
pub use role::Column as RoleColumn;
pub use role::Entity as Role;

pub use permission::ActiveModel as PermissionActiveModel;
pub use permission::Column as PermissionColumn;
pub use permission::Entity as Permission;

pub use user_role::ActiveModel as UserRoleActiveModel;
pub use user_role::Column as UserRoleColumn;
pub use user_role::Entity as UserRole;

pub use role_permission::ActiveModel as RolePermissionActiveModel;
pub use role_permission::Column as RolePermissionColumn;
pub use role_permission::Entity as RolePermission;
