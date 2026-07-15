pub mod notification;
pub mod notification_template;

pub use notification_template::ActiveModel as NotificationTemplateActiveModel;
pub use notification_template::Entity as NotificationTemplate;

pub use notification::ActiveModel as NotificationActiveModel;
pub use notification::Column as NotificationColumn;
pub use notification::Entity as Notification;
