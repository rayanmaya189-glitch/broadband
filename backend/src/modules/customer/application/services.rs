use crate::modules::customer::domain::entities::{
    Address, AddressActiveModel, AddressColumn, Customer, CustomerActiveModel, CustomerColumn,
};
use crate::shared::errors::AppError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};

pub struct CustomerService;

impl CustomerService {
    pub async fn list_customers(
        db: &DatabaseConnection,
        branch_id: Option<i64>,
        _page: u64,
        _limit: u64,
    ) -> Result<
        (
            Vec<crate::modules::customer::domain::entities::customer::Model>,
            u64,
        ),
        AppError,
    > {
        let mut query = Customer::find().filter(CustomerColumn::DeletedAt.is_null());
        if let Some(bid) = branch_id {
            query = query.filter(CustomerColumn::BranchId.eq(bid));
        }
        let total = query.clone().count(db).await?;
        let customers = query.all(db).await?;
        Ok((customers, total))
    }

    pub async fn get_customer(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<crate::modules::customer::domain::entities::customer::Model, AppError> {
        Customer::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Customer {} not found", id)))
    }

    pub async fn create_customer(
        db: &DatabaseConnection,
        branch_id: i64,
        name: String,
        email: Option<String>,
        phone: String,
        alternate_phone: Option<String>,
    ) -> Result<crate::modules::customer::domain::entities::customer::Model, AppError> {
        let existing = Customer::find()
            .filter(CustomerColumn::Phone.eq(&phone))
            .one(db)
            .await?;
        if existing.is_some() {
            return Err(AppError::Conflict(
                "Phone number already registered".to_string(),
            ));
        }

        let now = chrono::Utc::now();
        let customer_code = format!("AX-CUST-{}", now.format("%Y%m%d%H%M%S"));

        let new_customer = CustomerActiveModel {
            customer_code: Set(customer_code),
            branch_id: Set(branch_id),
            name: Set(name),
            email: Set(email),
            phone: Set(phone),
            alternate_phone: Set(alternate_phone),
            status: Set("registered".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(new_customer.insert(db).await?)
    }

    pub async fn update_customer_status(
        db: &DatabaseConnection,
        id: i64,
        new_status: &str,
    ) -> Result<crate::modules::customer::domain::entities::customer::Model, AppError> {
        let customer = Self::get_customer(db, id).await?;
        let mut active: CustomerActiveModel = customer.into();
        active.status = Set(new_status.to_string());
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(db).await?)
    }

    pub async fn get_addresses(
        db: &DatabaseConnection,
        customer_id: i64,
    ) -> Result<Vec<crate::modules::customer::domain::entities::address::Model>, AppError> {
        let addresses = Address::find()
            .filter(AddressColumn::CustomerId.eq(customer_id))
            .all(db)
            .await?;
        Ok(addresses)
    }

    pub async fn add_address(
        db: &DatabaseConnection,
        customer_id: i64,
        address_type: String,
        line1: String,
        line2: Option<String>,
        city: String,
        state: String,
        pincode: String,
        landmark: Option<String>,
    ) -> Result<crate::modules::customer::domain::entities::address::Model, AppError> {
        let now = chrono::Utc::now();
        let new_address = AddressActiveModel {
            customer_id: Set(customer_id),
            address_type: Set(address_type),
            line1: Set(line1),
            line2: Set(line2),
            city: Set(city),
            state: Set(state),
            pincode: Set(pincode),
            country: Set("India".to_string()),
            landmark: Set(landmark),
            is_primary: Set(true),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(new_address.insert(db).await?)
    }

    pub async fn delete_customer(db: &DatabaseConnection, id: i64) -> Result<(), AppError> {
        let customer = Self::get_customer(db, id).await?;
        let mut active: CustomerActiveModel = customer.into();
        active.deleted_at = Set(Some(chrono::Utc::now()));
        active.updated_at = Set(chrono::Utc::now());
        active.update(db).await?;
        Ok(())
    }
}


