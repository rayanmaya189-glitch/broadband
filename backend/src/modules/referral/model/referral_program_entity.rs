use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "referral_programs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub name: String,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub status: String,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub referrer_reward_type: String,
    pub referrer_reward_value: sea_orm::prelude::Decimal,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub referee_reward_type: String,
    pub referee_reward_value: sea_orm::prelude::Decimal,
    pub max_referrals_per_customer: Option<i32>,
    pub start_date: Date,
    pub end_date: Date,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
