pub mod plan;
pub mod plan_pricing;
pub mod speed_profile;

pub use plan::Entity as Plan;
pub use plan::ActiveModel as PlanActiveModel;
pub use plan::Column as PlanColumn;

pub use plan_pricing::Entity as PlanPricing;
pub use plan_pricing::ActiveModel as PlanPricingActiveModel;
pub use plan_pricing::Column as PlanPricingColumn;

pub use speed_profile::Entity as SpeedProfile;
pub use speed_profile::ActiveModel as SpeedProfileActiveModel;
pub use speed_profile::Column as SpeedProfileColumn;
