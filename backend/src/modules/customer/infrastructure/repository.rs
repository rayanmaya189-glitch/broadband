use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use crate::shared::errors::AppError;
use crate::modules::customer::domain::entities::{Customer, CustomerColumn};

pub struct CustomerRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> CustomerRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<<Customer as sea_orm::EntityTrait>::Model>, AppError> {
        Ok(Customer::find_by_id(id).one(self.db).await?)
    }

    pub async fn find_by_phone(&self, phone: &str) -> Result<Option<<Customer as sea_orm::EntityTrait>::Model>, AppError> {
        Ok(Customer::find().filter(CustomerColumn::Phone.eq(phone)).one(self.db).await?)
    }

    pub async fn find_by_customer_code(&self, code: &str) -> Result<Option<<Customer as sea_orm::EntityTrait>::Model>, AppError> {
        Ok(Customer::find().filter(CustomerColumn::CustomerCode.eq(code)).one(self.db).await?)
    }
}
