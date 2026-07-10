use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "trial_balances")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub period_start: Date,
    pub period_end: Date,
    pub account_id: i64,
    pub opening_balance: sea_orm::prelude::Decimal,
    pub total_debit: sea_orm::prelude::Decimal,
    pub total_credit: sea_orm::prelude::Decimal,
    pub closing_balance: sea_orm::prelude::Decimal,
    pub generated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
