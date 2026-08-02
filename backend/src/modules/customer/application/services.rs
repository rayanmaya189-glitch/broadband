use crate::modules::customer::domain::entities::{
    Address, AddressActiveModel, AddressColumn, Customer, CustomerActiveModel, CustomerColumn,
};
use crate::shared::errors::AppError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter, Set,
};

pub struct CustomerService;

impl CustomerService {
    pub async fn list_customers(
        db: &DatabaseConnection,
        branch_id: Option<i64>,
        page: u64,
        limit: u64,
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
        let paginator = query.paginate(db, limit);
        let total = paginator.num_items().await?;
        let customers = paginator.fetch_page(page.saturating_sub(1)).await?;
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
        db: &impl ConnectionTrait,
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
        let uuid = crate::shared::utils::uuid_v7::new_v7_compact();
        let customer_code = format!("AX-CUST-{}", &uuid[uuid.len() - 12..]);

        let referee_phone = phone.clone();

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
        let customer = new_customer.insert(db).await?;

        // Link this new customer to any pending referral that was shared with
        // their phone number so the reward can be paid out once they activate.
        crate::modules::referral::application::services::ReferralService::register_referee(
            db,
            &referee_phone,
            customer.id,
        )
        .await?;

        Ok(customer)
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
        db: &impl ConnectionTrait,
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

        // Cleanup: terminate active PPPoE sessions for this customer
        (async {
            use crate::modules::network::domain::entities::{
                PppoeSession, PppoeSessionActiveModel, PppoeSessionColumn,
            };
            let sessions = PppoeSession::find()
                .filter(PppoeSessionColumn::CustomerId.eq(id))
                .filter(PppoeSessionColumn::Status.eq("active"))
                .all(db)
                .await?;
            for s in sessions {
                let mut m: PppoeSessionActiveModel = s.into();
                m.status = Set("terminated".to_string());
                m.updated_at = Set(chrono::Utc::now());
                m.update(db).await?;
            }
            Ok::<_, sea_orm::DbErr>(())
        })
        .await
        .ok();

        // Cleanup: cancel active subscriptions for this customer
        (async {
            use crate::modules::subscription::domain::entities::{
                Subscription, SubscriptionActiveModel, SubscriptionColumn,
            };
            let subs = Subscription::find()
                .filter(SubscriptionColumn::CustomerId.eq(id))
                .filter(SubscriptionColumn::Status.eq("active"))
                .all(db)
                .await?;
            for s in subs {
                let mut m: SubscriptionActiveModel = s.into();
                m.status = Set("cancelled".to_string());
                m.updated_at = Set(chrono::Utc::now());
                m.update(db).await?;
            }
            Ok::<_, sea_orm::DbErr>(())
        })
        .await
        .ok();

        // Cleanup: release MAC bindings for this customer
        (async {
            use crate::modules::network::domain::entities::{
                MacBinding, MacBindingActiveModel, MacBindingColumn,
            };
            let bindings = MacBinding::find()
                .filter(MacBindingColumn::CustomerId.eq(id))
                .all(db)
                .await?;
            for b in bindings {
                let mut m: MacBindingActiveModel = b.into();
                m.is_active = Set(false);
                m.updated_at = Set(chrono::Utc::now());
                m.update(db).await?;
            }
            Ok::<_, sea_orm::DbErr>(())
        })
        .await
        .ok();

        // Cleanup: close open tickets for this customer
        (async {
            use crate::modules::ticket::domain::entities::{
                Ticket, TicketActiveModel, TicketColumn,
            };
            let tickets = Ticket::find()
                .filter(TicketColumn::CustomerId.eq(Some(id)))
                .filter(TicketColumn::Status.ne("closed"))
                .all(db)
                .await?;
            for t in tickets {
                let mut m: TicketActiveModel = t.into();
                m.status = Set("closed".to_string());
                m.closed_at = Set(Some(chrono::Utc::now()));
                m.updated_at = Set(chrono::Utc::now());
                m.resolution_notes = Set(Some("Customer deleted".to_string()));
                m.update(db).await?;
            }
            Ok::<_, sea_orm::DbErr>(())
        })
        .await
        .ok();

        Ok(())
    }

    pub async fn update_customer(
        db: &DatabaseConnection,
        id: i64,
        name: Option<String>,
        email: Option<String>,
        phone: Option<String>,
        alternate_phone: Option<String>,
    ) -> Result<crate::modules::customer::domain::entities::customer::Model, AppError> {
        let customer = Self::get_customer(db, id).await?;
        let mut active: CustomerActiveModel = customer.into();
        if let Some(n) = name {
            active.name = Set(n);
        }
        if let Some(e) = email {
            active.email = Set(Some(e));
        }
        if let Some(p) = phone {
            active.phone = Set(p);
        }
        if let Some(a) = alternate_phone {
            active.alternate_phone = Set(Some(a));
        }
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(db).await?)
    }

    pub async fn search_customers(
        db: &DatabaseConnection,
        query: Option<String>,
        status: Option<String>,
    ) -> Result<Vec<crate::modules::customer::domain::entities::customer::Model>, AppError> {
        let mut q = Customer::find().filter(CustomerColumn::DeletedAt.is_null());
        if let Some(s) = status {
            q = q.filter(CustomerColumn::Status.eq(s));
        }
        if let Some(search) = query {
            q = q.filter(
                CustomerColumn::Name
                    .contains(&search)
                    .or(CustomerColumn::Phone.contains(&search))
                    .or(CustomerColumn::CustomerCode.contains(&search)),
            );
        }
        Ok(q.all(db).await?)
    }
}
