use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "referral_tracking")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub program_id: i64,
    pub referrer_id: i64,
    pub referee_id: Option<i64>,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub referral_code: String,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub referee_phone: String,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub status: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
