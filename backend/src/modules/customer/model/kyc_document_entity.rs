use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "kyc_documents")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub customer_id: i64,
    pub document_type: String,
    pub document_url: String,
    pub file_name: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub mime_type: Option<String>,
    pub verification_status: String,
    pub rejection_reason: Option<String>,
    pub verified_by: Option<i64>,
    pub verified_at: Option<DateTimeWithTimeZone>,
    pub uploaded_at: DateTimeWithTimeZone,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::customer_entity::Entity",
        from = "Column::CustomerId",
        to = "super::customer_entity::Column::Id"
    )]
    Customer,
}

impl Related<super::customer_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
