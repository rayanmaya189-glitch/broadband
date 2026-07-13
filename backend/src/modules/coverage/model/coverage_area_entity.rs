use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "coverage_areas")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub branch_id: i64,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub name: String,
    pub description: Option<String>,
    #[sea_orm(column_type = "String(StringLen::N(30))")]
    pub area_type: String,
    pub pincodes: Option<sea_orm::prelude::Json>,
    pub is_active: bool,
    pub max_customers: Option<i32>,
    pub current_customers: Option<i32>,
    pub fiber_available: bool,
    pub estimated_installation_days: Option<i32>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::coverage_pincode_entity::Entity")]
    Pincode,
}

impl Related<super::coverage_pincode_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Pincode.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
