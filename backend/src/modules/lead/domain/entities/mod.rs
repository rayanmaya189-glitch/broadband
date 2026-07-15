pub mod lead;
pub mod lead_activity;

pub use lead::ActiveModel as LeadActiveModel;
pub use lead::Column as LeadColumn;
pub use lead::Entity as Lead;

pub use lead_activity::ActiveModel as LeadActivityActiveModel;
pub use lead_activity::Entity as LeadActivity;
