use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "inventory_items")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub branch_id: i64,
    #[sea_orm(column_type = "String(StringLen::N(30))")]
    pub item_type: String,
    pub device_model_id: Option<i64>,
    pub serial_number: Option<String>,
    pub barcode: Option<String>,
    pub purchase_date: Option<Date>,
    pub purchase_price: Option<sea_orm::prelude::Decimal>,
    pub warranty_expiry: Option<Date>,
    pub supplier: Option<String>,
    #[sea_orm(column_type = "String(StringLen::N(30))")]
    pub status: String,
    pub assigned_to: Option<i64>,
    pub assigned_to_branch_id: Option<i64>,
    pub notes: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
