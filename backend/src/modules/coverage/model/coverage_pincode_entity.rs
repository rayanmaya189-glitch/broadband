use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "coverage_pincode_map")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub coverage_area_id: i64,
    #[sea_orm(column_type = "String(StringLen::N(10))")]
    pub pincode: String,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub city: String,
    pub district: Option<String>,
    pub state: Option<String>,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::coverage_area_entity::Entity", from = "Column::CoverageAreaId", to = "super::coverage_area_entity::Column::Id")]
    CoverageArea,
}

impl Related<super::coverage_area_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CoverageArea.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
