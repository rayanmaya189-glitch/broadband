pub mod notification_template;
pub mod notification;

pub use notification_template::Entity as NotificationTemplate;
pub use notification_template::ActiveModel as NotificationTemplateActiveModel;

pub use notification::Entity as Notification;
pub use notification::ActiveModel as NotificationActiveModel;
pub use notification::Column as NotificationColumn;
