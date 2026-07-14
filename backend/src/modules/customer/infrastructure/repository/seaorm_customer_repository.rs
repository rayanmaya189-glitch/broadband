//! SeaORM implementation of the customer repository.

use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set};

use crate::common::errors::app_error::AppError;
use crate::modules::customer::domain::aggregates::customer::customer::Customer;
use crate::modules::customer::domain::value_objects::{
    customer_id::CustomerId, customer_status::CustomerStatus, email::Email, phone::Phone,
};
use crate::modules::customer::infrastructure::repository::customer_repository_trait::CustomerRepositoryTrait;
use crate::modules::customer::model::customer_entity;

/// SeaORM implementation of the customer repository.
pub struct SeaOrmCustomerRepository {
    db: DatabaseConnection,
}

impl SeaOrmCustomerRepository {
    /// Create a new SeaORM customer repository.
    pub fn new(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }

    /// Convert a SeaORM entity model to a domain aggregate.
    fn to_domain(model: customer_entity::Model) -> Result<Customer, AppError> {
        let email = model
            .email
            .as_ref()
            .map(|e| Email::new(e))
            .transpose()
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid email in database: {}", e)))?;

        let phone = Phone::new(&model.phone)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid phone in database: {}", e)))?;

        let status = CustomerStatus::from_str(&model.status)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid status in database: {}", e)))?;

        Ok(Customer {
            id: CustomerId::new(model.id),
            customer_code: model.customer_code,
            first_name: model.first_name,
            last_name: model.last_name,
            email,
            phone,
            alternate_phone: None, // TODO: Load from profile if needed
            status,
            branch_id: model.branch_id,
            lead_id: model.lead_id,
            referred_by: model.referred_by,
            created_by: model.created_by,
            kyc_status: model.kyc_status,
            notes: model.notes,
            created_at: model.created_at.with_timezone(&chrono::Utc),
            updated_at: model.updated_at.with_timezone(&chrono::Utc),
        })
    }
}

#[async_trait]
impl CustomerRepositoryTrait for SeaOrmCustomerRepository {
    async fn find_by_id(&self, id: i64) -> Result<Option<Customer>, AppError> {
        let model = customer_entity::Entity::find_by_id(id)
            .one(&self.db)
            .await?;

        model.map(Self::to_domain).transpose()
    }

    async fn find_by_code(&self, code: &str) -> Result<Option<Customer>, AppError> {
        let model = customer_entity::Entity::find()
            .filter(customer_entity::Column::CustomerCode.eq(code))
            .one(&self.db)
            .await?;

        model.map(Self::to_domain).transpose()
    }

    async fn save(&self, customer: &mut Customer) -> Result<(), AppError> {
        // Create active model without ID (let SeaORM auto-generate)
        let active = customer_entity::ActiveModel {
            customer_code: Set(customer.customer_code.clone()),
            first_name: Set(customer.first_name.clone()),
            last_name: Set(customer.last_name.clone()),
            email: Set(customer.email.as_ref().map(|e| e.as_str().to_string())),
            phone: Set(customer.phone.as_str().to_string()),
            alternate_phone: Set(customer.alternate_phone.as_ref().map(|p| p.as_str().to_string())),
            status: Set(customer.status.as_str().to_string()),
            branch_id: Set(customer.branch_id),
            lead_id: Set(customer.lead_id),
            referred_by: Set(customer.referred_by),
            created_by: Set(customer.created_by),
            kyc_status: Set(customer.kyc_status.clone()),
            notes: Set(customer.notes.clone()),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        };

        let result = active.insert(&self.db).await?;

        // Update the customer with the generated ID
        customer.set_id(result.id);

        Ok(())
    }

    async fn update(&self, customer: &Customer) -> Result<(), AppError> {
        let existing = customer_entity::Entity::find_by_id(customer.id.inner())
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Customer not found".to_string()))?;

        let mut active: customer_entity::ActiveModel = existing.into();
        active.customer_code = Set(customer.customer_code.clone());
        active.first_name = Set(customer.first_name.clone());
        active.last_name = Set(customer.last_name.clone());
        active.email = Set(customer.email.as_ref().map(|e| e.as_str().to_string()));
        active.phone = Set(customer.phone.as_str().to_string());
        active.alternate_phone = Set(customer.alternate_phone.as_ref().map(|p| p.as_str().to_string()));
        active.status = Set(customer.status.as_str().to_string());
        active.branch_id = Set(customer.branch_id);
        active.lead_id = Set(customer.lead_id);
        active.referred_by = Set(customer.referred_by);
        active.created_by = Set(customer.created_by);
        active.kyc_status = Set(customer.kyc_status.clone());
        active.notes = Set(customer.notes.clone());
        active.updated_at = Set(customer.updated_at.into());

        active.update(&self.db).await?;
        Ok(())
    }

    async fn phone_exists(&self, phone: &str, exclude_id: Option<i64>) -> Result<bool, AppError> {
        let mut select = customer_entity::Entity::find()
            .filter(customer_entity::Column::Phone.eq(phone));

        if let Some(id) = exclude_id {
            select = select.filter(customer_entity::Column::Id.ne(id));
        }

        let count = select.count(&self.db).await?;
        Ok(count > 0)
    }

    async fn generate_customer_code(&self, branch_code: &str) -> Result<String, AppError> {
        let month = chrono::Utc::now().format("%Y%m").to_string();
        let pattern = format!("AX-{branch_code}-{month}-%");

        let count = customer_entity::Entity::find()
            .filter(customer_entity::Column::CustomerCode.like(&pattern))
            .count(&self.db)
            .await? as i64;

        Ok(format!("AX-{branch_code}-{month}-{:04}", count + 1))
    }

    async fn list(
        &self,
        offset: u32,
        limit: u32,
        status: Option<&str>,
        branch_id: Option<i64>,
        search: Option<&str>,
    ) -> Result<Vec<Customer>, AppError> {
        let mut select = customer_entity::Entity::find();

        if let Some(s) = status {
            select = select.filter(customer_entity::Column::Status.eq(s));
        }
        if let Some(bid) = branch_id {
            select = select.filter(customer_entity::Column::BranchId.eq(bid));
        }
        if let Some(s) = search {
            let pattern = format!("%{s}%");
            select = select.filter(
                sea_orm::Condition::any()
                    .add(customer_entity::Column::FirstName.contains(&pattern))
                    .add(customer_entity::Column::LastName.contains(&pattern))
                    .add(customer_entity::Column::Phone.contains(&pattern))
                    .add(customer_entity::Column::Email.contains(&pattern))
                    .add(customer_entity::Column::CustomerCode.contains(&pattern)),
            );
        }

        let models = select
            .order_by_desc(customer_entity::Column::CreatedAt)
            .offset(offset as u64)
            .limit(limit as u64)
            .all(&self.db)
            .await?;

        models.into_iter().map(Self::to_domain).collect()
    }

    async fn count(
        &self,
        status: Option<&str>,
        branch_id: Option<i64>,
        search: Option<&str>,
    ) -> Result<i64, AppError> {
        let mut select = customer_entity::Entity::find();

        if let Some(s) = status {
            select = select.filter(customer_entity::Column::Status.eq(s));
        }
        if let Some(bid) = branch_id {
            select = select.filter(customer_entity::Column::BranchId.eq(bid));
        }
        if let Some(s) = search {
            let pattern = format!("%{s}%");
            select = select.filter(
                sea_orm::Condition::any()
                    .add(customer_entity::Column::FirstName.contains(&pattern))
                    .add(customer_entity::Column::LastName.contains(&pattern))
                    .add(customer_entity::Column::Phone.contains(&pattern))
                    .add(customer_entity::Column::Email.contains(&pattern))
                    .add(customer_entity::Column::CustomerCode.contains(&pattern)),
            );
        }

        let count = select.count(&self.db).await?;
        Ok(count as i64)
    }
}
