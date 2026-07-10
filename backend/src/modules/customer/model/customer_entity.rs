use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "customers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub customer_code: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: String,
    pub alternate_phone: Option<String>,
    pub status: String,
    pub branch_id: i64,
    pub lead_id: Option<i64>,
    pub referred_by: Option<i64>,
    pub created_by: Option<i64>,
    pub kyc_status: String,
    pub notes: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::customer_profile_entity::Entity")]
    Profile,

    #[sea_orm(has_many = "super::kyc_document_entity::Entity")]
    KycDocument,

    #[sea_orm(has_many = "super::customer_address_entity::Entity")]
    Address,
}

impl Related<super::customer_profile_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Profile.def()
    }
}

impl Related<super::kyc_document_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::KycDocument.def()
    }
}

impl Related<super::customer_address_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Address.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
