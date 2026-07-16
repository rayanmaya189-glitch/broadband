use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "journal_entries")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub entry_number: String,
    pub entry_date: chrono::NaiveDate,
    pub description: String,
    pub reference_type: Option<String>,
    pub reference_id: Option<i64>,
    pub total_debit: sea_orm::prelude::Decimal,
    pub total_credit: sea_orm::prelude::Decimal,
    pub status: String,
    pub created_by: Option<i64>,
    pub reviewed_by: Option<i64>,
    pub reviewed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub posted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::journal_entry_line::Entity")]
    JournalEntryLines,
}

impl Related<super::journal_entry_line::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::JournalEntryLines.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
