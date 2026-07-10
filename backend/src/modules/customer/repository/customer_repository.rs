use sea_orm::{*, sea_query::Expr};

use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::{total_pages, PaginatedResponse};
use crate::modules::customer::model::customer_entity::{self, Model as CustomerModel};
use crate::modules::customer::model::customer_profile_entity::{self, Model as ProfileModel};
use crate::modules::customer::model::kyc_document_entity::{self, Model as KycDocModel};
use crate::modules::customer::model::customer_address_entity::{self, Model as AddressModel};
use crate::modules::customer::response::customer_response::{CustomerResponse, AddressResponse};

pub struct CustomerRepository {
    db: DatabaseConnection,
}

impl CustomerRepository {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<CustomerModel>, AppError> {
        let model = customer_entity::Entity::find_by_id(id)
            .one(&self.db)
            .await
            ?;
        Ok(model)
    }

    pub async fn find_by_code(&self, code: &str) -> Result<Option<CustomerModel>, AppError> {
        let model = customer_entity::Entity::find()
            .filter(customer_entity::Column::CustomerCode.eq(code))
            .one(&self.db)
            .await
            ?;
        Ok(model)
    }

    pub async fn create(
        &self,
        customer_code: &str,
        first_name: &str,
        last_name: Option<&str>,
        email: Option<&str>,
        phone: &str,
        alternate_phone: Option<&str>,
        branch_id: i64,
        lead_id: Option<i64>,
        referred_by: Option<i64>,
        created_by: Option<i64>,
        notes: Option<&str>,
    ) -> Result<CustomerModel, AppError> {
        let active = customer_entity::ActiveModel {
            customer_code: Set(customer_code.to_string()),
            first_name: Set(first_name.to_string()),
            last_name: Set(last_name.map(|s| s.to_string())),
            email: Set(email.map(|s| s.to_string())),
            phone: Set(phone.to_string()),
            alternate_phone: Set(alternate_phone.map(|s| s.to_string())),
            branch_id: Set(branch_id),
            lead_id: Set(lead_id),
            referred_by: Set(referred_by),
            created_by: Set(created_by),
            notes: Set(notes.map(|s| s.to_string())),
            ..Default::default()
        };
        let model = active.insert(&self.db).await?;
        Ok(model)
    }

    pub async fn update(
        &self,
        id: i64,
        first_name: Option<&str>,
        last_name: Option<&str>,
        email: Option<&str>,
        phone: Option<&str>,
        alternate_phone: Option<&str>,
        notes: Option<&str>,
    ) -> Result<CustomerModel, AppError> {
        let model = customer_entity::Entity::find_by_id(id)
            .one(&self.db)
            .await
            ?
            .ok_or_else(|| AppError::NotFound("Customer not found".into()))?;

        let mut active: customer_entity::ActiveModel = model.into();
        if let Some(v) = first_name { active.first_name = Set(v.to_string()); }
        if let Some(v) = last_name { active.last_name = Set(Some(v.to_string())); }
        if let Some(v) = email { active.email = Set(Some(v.to_string())); }
        if let Some(v) = phone { active.phone = Set(v.to_string()); }
        if let Some(v) = alternate_phone { active.alternate_phone = Set(Some(v.to_string())); }
        if let Some(v) = notes { active.notes = Set(Some(v.to_string())); }
        active.updated_at = Set(chrono::Utc::now().into());
        let updated = active.update(&self.db).await?;
        Ok(updated)
    }

    pub async fn update_status(&self, id: i64, status: &str) -> Result<CustomerModel, AppError> {
        let model = customer_entity::Entity::find_by_id(id)
            .one(&self.db)
            .await
            ?
            .ok_or_else(|| AppError::NotFound("Customer not found".into()))?;

        let mut active: customer_entity::ActiveModel = model.into();
        active.status = Set(status.to_string());
        active.updated_at = Set(chrono::Utc::now().into());
        let updated = active.update(&self.db).await?;
        Ok(updated)
    }

    pub async fn soft_delete(&self, id: i64) -> Result<(), AppError> {
        let model = customer_entity::Entity::find_by_id(id)
            .one(&self.db)
            .await
            ?
            .ok_or_else(|| AppError::NotFound("Customer not found".into()))?;
        let mut active: customer_entity::ActiveModel = model.into();
        active.updated_at = Set(chrono::Utc::now().into());
        active.update(&self.db).await?;
        Ok(())
    }

    pub async fn list(
        &self,
        offset: u32,
        limit: u32,
        status: Option<&str>,
        branch_id: Option<i64>,
        search: Option<&str>,
    ) -> Result<PaginatedResponse<CustomerResponse>, AppError> {
        let page_size = limit.max(1) as u64;
        let page_num = ((offset / limit) + 1).max(1) as u64;

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
                Condition::any()
                    .add(customer_entity::Column::FirstName.contains(&pattern))
                    .add(customer_entity::Column::LastName.contains(&pattern))
                    .add(customer_entity::Column::Phone.contains(&pattern))
                    .add(customer_entity::Column::Email.contains(&pattern))
                    .add(customer_entity::Column::CustomerCode.contains(&pattern)),
            );
        }

        let paginator = select
            .order_by_desc(customer_entity::Column::CreatedAt)
            .paginate(&self.db, page_size);

        let total = paginator.num_items().await? as i64;
        let models = paginator.fetch_page(page_num - 1).await?;
        let data = models.into_iter().map(CustomerResponse::from_model).collect();
        let tp = total_pages(total, limit);
        Ok(PaginatedResponse { data, total, page: page_num as u32, limit, total_pages: tp })
    }

    pub async fn phone_exists(&self, phone: &str, exclude: Option<i64>) -> Result<bool, AppError> {
        let mut select = customer_entity::Entity::find()
            .filter(customer_entity::Column::Phone.eq(phone));
        if let Some(id) = exclude {
            select = select.filter(customer_entity::Column::Id.ne(id));
        }
        let count = select.count(&self.db).await?;
        Ok(count > 0)
    }

    pub async fn generate_customer_code(&self, branch_code: &str) -> Result<String, AppError> {
        let month = chrono::Utc::now().format("%Y%m").to_string();
        let pattern = format!("AX-{branch_code}-{month}-%");
        let count = customer_entity::Entity::find()
            .filter(customer_entity::Column::CustomerCode.like(&pattern))
            .count(&self.db)
            .await
            ? as i64;
        Ok(format!("AX-{branch_code}-{month}-{:04}", count + 1))
    }

    // ── Profile ──────────────────────────────────────────────

    pub async fn get_profile(&self, customer_id: i64) -> Result<Option<ProfileModel>, AppError> {
        let model = customer_profile_entity::Entity::find()
            .filter(customer_profile_entity::Column::CustomerId.eq(customer_id))
            .one(&self.db)
            .await
            ?;
        Ok(model)
    }

    pub async fn upsert_profile(
        &self,
        customer_id: i64,
        date_of_birth: Option<chrono::NaiveDate>,
        gender: Option<&str>,
        nationality: Option<&str>,
        id_proof_type: Option<&str>,
        id_proof_number: Option<&str>,
        pan_number: Option<&str>,
        aadhaar_number: Option<&str>,
        gstin: Option<&str>,
        company_name: Option<&str>,
        designation: Option<&str>,
        occupation: Option<&str>,
        annual_income_range: Option<&str>,
        preferred_language: Option<&str>,
        communication_opt_in: Option<bool>,
        notes: Option<&str>,
    ) -> Result<ProfileModel, AppError> {
        let existing = self.get_profile(customer_id).await?;
        if let Some(model) = existing {
            let mut active: customer_profile_entity::ActiveModel = model.into();
            if let Some(v) = date_of_birth { active.date_of_birth = Set(Some(v)); }
            if let Some(v) = gender { active.gender = Set(Some(v.to_string())); }
            if let Some(v) = nationality { active.nationality = Set(Some(v.to_string())); }
            if let Some(v) = id_proof_type { active.id_proof_type = Set(Some(v.to_string())); }
            if let Some(v) = id_proof_number { active.id_proof_number = Set(Some(v.to_string())); }
            if let Some(v) = pan_number { active.pan_number = Set(Some(v.to_string())); }
            if let Some(v) = aadhaar_number { active.aadhaar_number = Set(Some(v.to_string())); }
            if let Some(v) = gstin { active.gstin = Set(Some(v.to_string())); }
            if let Some(v) = company_name { active.company_name = Set(Some(v.to_string())); }
            if let Some(v) = designation { active.designation = Set(Some(v.to_string())); }
            if let Some(v) = occupation { active.occupation = Set(Some(v.to_string())); }
            if let Some(v) = annual_income_range { active.annual_income_range = Set(Some(v.to_string())); }
            if let Some(v) = preferred_language { active.preferred_language = Set(Some(v.to_string())); }
            if let Some(v) = communication_opt_in { active.communication_opt_in = Set(v); }
            if let Some(v) = notes { active.notes = Set(Some(v.to_string())); }
            active.updated_at = Set(chrono::Utc::now().into());
            let updated = active.update(&self.db).await?;
            Ok(updated)
        } else {
            let active = customer_profile_entity::ActiveModel {
                customer_id: Set(customer_id),
                date_of_birth: Set(date_of_birth),
                gender: Set(gender.map(|s| s.to_string())),
                nationality: Set(nationality.map(|s| s.to_string())),
                id_proof_type: Set(id_proof_type.map(|s| s.to_string())),
                id_proof_number: Set(id_proof_number.map(|s| s.to_string())),
                pan_number: Set(pan_number.map(|s| s.to_string())),
                aadhaar_number: Set(aadhaar_number.map(|s| s.to_string())),
                gstin: Set(gstin.map(|s| s.to_string())),
                company_name: Set(company_name.map(|s| s.to_string())),
                designation: Set(designation.map(|s| s.to_string())),
                occupation: Set(occupation.map(|s| s.to_string())),
                annual_income_range: Set(annual_income_range.map(|s| s.to_string())),
                preferred_language: Set(preferred_language.map(|s| s.to_string())),
                communication_opt_in: Set(communication_opt_in.unwrap_or(false)),
                notes: Set(notes.map(|s| s.to_string())),
                ..Default::default()
            };
            let inserted = active.insert(&self.db).await?;
            Ok(inserted)
        }
    }

    // ── KYC Documents ───────────────────────────────────────

    pub async fn list_kyc_documents(&self, customer_id: i64) -> Result<Vec<KycDocModel>, AppError> {
        let models = kyc_document_entity::Entity::find()
            .filter(kyc_document_entity::Column::CustomerId.eq(customer_id))
            .order_by_desc(kyc_document_entity::Column::UploadedAt)
            .all(&self.db)
            .await
            ?;
        Ok(models)
    }

    pub async fn create_kyc_document(
        &self,
        customer_id: i64,
        document_type: &str,
        document_url: &str,
        file_name: Option<&str>,
        file_size: Option<i64>,
        mime_type: Option<&str>,
    ) -> Result<KycDocModel, AppError> {
        let active = kyc_document_entity::ActiveModel {
            customer_id: Set(customer_id),
            document_type: Set(document_type.to_string()),
            document_url: Set(document_url.to_string()),
            file_name: Set(file_name.map(|s| s.to_string())),
            file_size_bytes: Set(file_size),
            mime_type: Set(mime_type.map(|s| s.to_string())),
            ..Default::default()
        };
        let model = active.insert(&self.db).await?;
        Ok(model)
    }

    pub async fn delete_kyc_document(&self, id: i64, customer_id: i64) -> Result<(), AppError> {
        kyc_document_entity::Entity::delete_many()
            .filter(kyc_document_entity::Column::Id.eq(id))
            .filter(kyc_document_entity::Column::CustomerId.eq(customer_id))
            .exec(&self.db)
            .await
            ?;
        Ok(())
    }

    // ── Addresses ───────────────────────────────────────────

    pub async fn list_addresses(&self, customer_id: i64) -> Result<Vec<AddressModel>, AppError> {
        let models = customer_address_entity::Entity::find()
            .filter(customer_address_entity::Column::CustomerId.eq(customer_id))
            .order_by_desc(customer_address_entity::Column::IsPrimary)
            .order_by_desc(customer_address_entity::Column::CreatedAt)
            .all(&self.db)
            .await
            ?;
        Ok(models)
    }

    pub async fn create_address(
        &self,
        customer_id: i64,
        address_type: &str,
        address_line1: &str,
        address_line2: Option<&str>,
        city: &str,
        state: &str,
        pincode: &str,
        landmark: Option<&str>,
        is_primary: bool,
    ) -> Result<AddressModel, AppError> {
        if is_primary {
            customer_address_entity::Entity::update_many()
                .filter(customer_address_entity::Column::CustomerId.eq(customer_id))
                .col_expr(
                    customer_address_entity::Column::IsPrimary,
                    Expr::val(false).into(),
                )
                .exec(&self.db)
                .await
                ?;
        }
        let active = customer_address_entity::ActiveModel {
            customer_id: Set(customer_id),
            address_type: Set(address_type.to_string()),
            address_line1: Set(address_line1.to_string()),
            address_line2: Set(address_line2.map(|s| s.to_string())),
            city: Set(city.to_string()),
            state: Set(state.to_string()),
            pincode: Set(pincode.to_string()),
            landmark: Set(landmark.map(|s| s.to_string())),
            is_primary: Set(is_primary),
            ..Default::default()
        };
        let model = active.insert(&self.db).await?;
        Ok(model)
    }

    pub async fn update_address(
        &self,
        id: i64,
        customer_id: i64,
        address_type: Option<&str>,
        address_line1: Option<&str>,
        address_line2: Option<&str>,
        city: Option<&str>,
        state: Option<&str>,
        pincode: Option<&str>,
        landmark: Option<&str>,
        is_primary: Option<bool>,
    ) -> Result<AddressModel, AppError> {
        if is_primary == Some(true) {
            customer_address_entity::Entity::update_many()
                .filter(customer_address_entity::Column::CustomerId.eq(customer_id))
                .col_expr(
                    customer_address_entity::Column::IsPrimary,
                    Expr::val(false).into(),
                )
                .exec(&self.db)
                .await
                ?;
        }
        let model = customer_address_entity::Entity::find()
            .filter(customer_address_entity::Column::Id.eq(id))
            .filter(customer_address_entity::Column::CustomerId.eq(customer_id))
            .one(&self.db)
            .await
            ?
            .ok_or_else(|| AppError::NotFound("Address not found".into()))?;

        let mut active: customer_address_entity::ActiveModel = model.into();
        if let Some(v) = address_type { active.address_type = Set(v.to_string()); }
        if let Some(v) = address_line1 { active.address_line1 = Set(v.to_string()); }
        if let Some(v) = address_line2 { active.address_line2 = Set(Some(v.to_string())); }
        if let Some(v) = city { active.city = Set(v.to_string()); }
        if let Some(v) = state { active.state = Set(v.to_string()); }
        if let Some(v) = pincode { active.pincode = Set(v.to_string()); }
        if let Some(v) = landmark { active.landmark = Set(Some(v.to_string())); }
        if let Some(v) = is_primary { active.is_primary = Set(v); }
        active.updated_at = Set(chrono::Utc::now().into());
        let updated = active.update(&self.db).await?;
        Ok(updated)
    }

    pub async fn delete_address(&self, id: i64, customer_id: i64) -> Result<(), AppError> {
        customer_address_entity::Entity::delete_many()
            .filter(customer_address_entity::Column::Id.eq(id))
            .filter(customer_address_entity::Column::CustomerId.eq(customer_id))
            .exec(&self.db)
            .await
            ?;
        Ok(())
    }
}
