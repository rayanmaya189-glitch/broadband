use crate::shared::errors::AppError;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;

pub type CustomerModel = crate::modules::customer::domain::entities::customer::Model;
pub type AddressModel = crate::modules::customer::domain::entities::address::Model;

#[async_trait]
pub trait CustomerServiceTrait: Send + Sync {
    async fn list_customers(
        &self,
        db: &DatabaseConnection,
        branch_id: Option<i64>,
        page: u64,
        limit: u64,
    ) -> Result<(Vec<CustomerModel>, u64), AppError>;

    async fn get_customer(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<CustomerModel, AppError>;

    async fn create_customer(
        &self,
        db: &DatabaseConnection,
        branch_id: i64,
        name: String,
        email: Option<String>,
        phone: String,
        alternate_phone: Option<String>,
    ) -> Result<CustomerModel, AppError>;

    async fn update_customer_status(
        &self,
        db: &DatabaseConnection,
        id: i64,
        new_status: &str,
    ) -> Result<CustomerModel, AppError>;

    async fn get_addresses(
        &self,
        db: &DatabaseConnection,
        customer_id: i64,
    ) -> Result<Vec<AddressModel>, AppError>;

    async fn add_address(
        &self,
        db: &DatabaseConnection,
        customer_id: i64,
        address_type: String,
        line1: String,
        line2: Option<String>,
        city: String,
        state: String,
        pincode: String,
        landmark: Option<String>,
    ) -> Result<AddressModel, AppError>;

    async fn delete_customer(&self, db: &DatabaseConnection, id: i64) -> Result<(), AppError>;
}
