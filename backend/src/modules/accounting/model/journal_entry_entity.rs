use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "journal_entries")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    #[sea_orm(column_type = "String(StringLen::N(50))", unique)]
    pub entry_number: String,
    pub entry_date: Date,
    #[sea_orm(column_type = "String(StringLen::N(500))")]
    pub description: String,
    pub reference_type: Option<String>,
    pub reference_id: Option<i64>,
    pub total_debit: sea_orm::prelude::Decimal,
    pub total_credit: sea_orm::prelude::Decimal,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub status: String,
    pub posted_at: Option<DateTimeWithTimeZone>,
    pub created_by: Option<i64>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
