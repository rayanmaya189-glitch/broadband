use crate::modules::installation::domain::entities::{
    InstallationOrder, InstallationOrderActiveModel, InstallationOrderColumn,
};
use crate::shared::errors::AppError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};

pub struct InstallationService;

impl InstallationService {
    pub async fn list_orders(
        db: &DatabaseConnection,
        branch_id: Option<i64>,
        _page: u64,
        _limit: u64,
    ) -> Result<
        (
            Vec<crate::modules::installation::domain::entities::installation_order::Model>,
            u64,
        ),
        AppError,
    > {
        let mut query = InstallationOrder::find();
        if let Some(bid) = branch_id {
            query = query.filter(InstallationOrderColumn::BranchId.eq(bid));
        }
        let t = query.clone().count(db).await?;
        Ok((query.all(db).await?, t))
    }

    pub async fn create_order(
        db: &DatabaseConnection,
        customer_id: i64,
        branch_id: i64,
        subscription_id: Option<i64>,
    ) -> Result<crate::modules::installation::domain::entities::installation_order::Model, AppError>
    {
        let now = chrono::Utc::now();
        let order = InstallationOrderActiveModel {
            customer_id: Set(customer_id),
            branch_id: Set(branch_id),
            subscription_id: Set(subscription_id),
            status: Set("pending".to_string()),
            installation_type: Set(Some("new".to_string())),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(order.insert(db).await?)
    }

    pub async fn schedule_order(
        db: &DatabaseConnection,
        id: i64,
        scheduled_date: chrono::NaiveDate,
        scheduled_time_slot: Option<String>,
        technician_id: Option<i64>,
    ) -> Result<crate::modules::installation::domain::entities::installation_order::Model, AppError>
    {
        let order = InstallationOrder::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Installation {} not found", id)))?;
        let mut active = <crate::modules::installation::domain::entities::installation_order::Entity as sea_orm::EntityTrait>::ActiveModel::from(order);
        active.status = Set("scheduled".to_string());
        active.scheduled_date = Set(Some(scheduled_date));
        active.scheduled_time_slot = Set(scheduled_time_slot);
        active.assigned_technician_id = Set(technician_id);
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(db).await?)
    }

    pub async fn complete_order(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<crate::modules::installation::domain::entities::installation_order::Model, AppError>
    {
        let order = InstallationOrder::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Installation {} not found", id)))?;
        let mut active = <crate::modules::installation::domain::entities::installation_order::Entity as sea_orm::EntityTrait>::ActiveModel::from(order);
        active.status = Set("completed".to_string());
        active.completed_at = Set(Some(chrono::Utc::now()));
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(db).await?)
    }

    pub async fn cancel_order(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<crate::modules::installation::domain::entities::installation_order::Model, AppError>
    {
        let order = InstallationOrder::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Installation {} not found", id)))?;
        let mut active = <crate::modules::installation::domain::entities::installation_order::Entity as sea_orm::EntityTrait>::ActiveModel::from(order);
        active.status = Set("cancelled".to_string());
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(db).await?)
    }

    // ─── Equipment ────────────────────────────────────────────────────

    pub async fn list_equipment(
        db: &DatabaseConnection,
        order_id: i64,
    ) -> Result<Vec<crate::modules::installation::domain::entities::installation_equipment::Model>, AppError> {
        use crate::modules::installation::domain::entities::InstallationEquipment;
        use crate::modules::installation::domain::entities::installation_equipment::Column;
        let items = InstallationEquipment::find()
            .filter(Column::InstallationOrderId.eq(order_id))
            .all(db)
            .await?;
        Ok(items)
    }

    pub async fn add_equipment(
        db: &DatabaseConnection,
        order_id: i64,
        equipment_type: String,
        model_name: Option<String>,
        serial_number: Option<String>,
        quantity: i32,
        notes: Option<String>,
    ) -> Result<crate::modules::installation::domain::entities::installation_equipment::Model, AppError> {
        use crate::modules::installation::domain::entities::installation_equipment;
        let now = chrono::Utc::now();
        let item = installation_equipment::ActiveModel {
            installation_order_id: Set(order_id),
            equipment_type: Set(equipment_type),
            model_name: Set(model_name),
            serial_number: Set(serial_number),
            quantity: Set(quantity),
            status: Set("assigned".to_string()),
            notes: Set(notes),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(item.insert(db).await?)
    }

    pub async fn update_equipment_status(
        db: &DatabaseConnection,
        equipment_id: i64,
        status: &str,
    ) -> Result<crate::modules::installation::domain::entities::installation_equipment::Model, AppError> {
        use crate::modules::installation::domain::entities::{InstallationEquipment, installation_equipment};
        let item = InstallationEquipment::find_by_id(equipment_id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Equipment {} not found", equipment_id)))?;
        let mut active: installation_equipment::ActiveModel = item.into();
        active.status = Set(status.to_string());
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(db).await?)
    }
}
