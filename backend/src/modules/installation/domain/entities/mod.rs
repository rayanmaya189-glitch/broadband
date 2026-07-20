pub mod installation_equipment;
pub mod installation_order;
pub mod installation_photo;

pub use installation_equipment::ActiveModel as InstallationEquipmentActiveModel;
pub use installation_equipment::Column as InstallationEquipmentColumn;
pub use installation_equipment::Entity as InstallationEquipment;

pub use installation_order::ActiveModel as InstallationOrderActiveModel;
pub use installation_order::Column as InstallationOrderColumn;
pub use installation_order::Entity as InstallationOrder;

pub use installation_photo::ActiveModel as InstallationPhotoActiveModel;
pub use installation_photo::Column as InstallationPhotoColumn;
pub use installation_photo::Entity as InstallationPhoto;
