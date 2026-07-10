use chrono::NaiveDate;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "customer_profiles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub customer_id: i64,
    pub date_of_birth: Option<NaiveDate>,
    pub gender: Option<String>,
    pub nationality: Option<String>,
    pub id_proof_type: Option<String>,
    pub id_proof_number: Option<String>,
    pub id_proof_expiry: Option<NaiveDate>,
    pub pan_number: Option<String>,
    pub aadhaar_number: Option<String>,
    pub gstin: Option<String>,
    pub company_name: Option<String>,
    pub designation: Option<String>,
    pub occupation: Option<String>,
    pub annual_income_range: Option<String>,
    pub preferred_language: Option<String>,
    pub communication_opt_in: bool,
    pub notes: Option<String>,
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
