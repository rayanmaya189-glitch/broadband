use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};

use crate::modules::coverage::domain::entities::{coverage_area, coverage_pincode};
use crate::shared::errors::AppError;

pub struct CoverageRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> CoverageRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_area_by_id(&self, id: i64) -> Result<Option<coverage_area::Model>, AppError> {
        Ok(coverage_area::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn find_areas_by_branch(
        &self,
        branch_id: i64,
    ) -> Result<Vec<coverage_area::Model>, AppError> {
        Ok(coverage_area::Entity::find()
            .filter(coverage_area::Column::BranchId.eq(branch_id))
            .filter(coverage_area::Column::IsActive.eq(true))
            .order_by_desc(coverage_area::Column::CreatedAt)
            .all(self.db)
            .await?)
    }

    pub async fn count_areas(&self) -> Result<i64, AppError> {
        Ok(coverage_area::Entity::find().count(self.db).await? as i64)
    }

    pub async fn create_area(
        &self,
        branch_id: i64,
        name: String,
        description: Option<String>,
        area_type: String,
        fiber_available: Option<bool>,
        estimated_installation_days: Option<i32>,
    ) -> Result<coverage_area::Model, AppError> {
        let now = chrono::Utc::now();
        let model = coverage_area::ActiveModel {
            branch_id: Set(branch_id),
            name: Set(name),
            description: Set(description),
            area_type: Set(area_type),
            is_active: Set(true),
            fiber_available: Set(fiber_available),
            estimated_installation_days: Set(estimated_installation_days),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(model.insert(self.db).await?)
    }

    pub async fn update_area(
        &self,
        model: coverage_area::Model,
        name: Option<String>,
        description: Option<String>,
        fiber_available: Option<bool>,
        estimated_installation_days: Option<i32>,
        is_active: Option<bool>,
    ) -> Result<coverage_area::Model, AppError> {
        let mut active: coverage_area::ActiveModel = model.into();
        if let Some(v) = name {
            active.name = Set(v);
        }
        if let Some(v) = description {
            active.description = Set(Some(v));
        }
        if let Some(v) = fiber_available {
            active.fiber_available = Set(Some(v));
        }
        if let Some(v) = estimated_installation_days {
            active.estimated_installation_days = Set(Some(v));
        }
        if let Some(v) = is_active {
            active.is_active = Set(v);
        }
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(self.db).await?)
    }

    pub async fn check_pincode_availability(
        &self,
        pincode: &str,
    ) -> Result<Option<coverage_area::Model>, AppError> {
        let pin = coverage_pincode::Entity::find()
            .filter(coverage_pincode::Column::Pincode.eq(pincode))
            .filter(coverage_pincode::Column::IsActive.eq(true))
            .one(self.db)
            .await?;
        if let Some(p) = pin {
            Ok(self.find_area_by_id(p.coverage_area_id).await?)
        } else {
            Ok(None)
        }
    }

    pub async fn add_pincode(
        &self,
        coverage_area_id: i64,
        pincode: String,
        city: String,
        district: Option<String>,
        state: Option<String>,
    ) -> Result<coverage_pincode::Model, AppError> {
        let model = coverage_pincode::ActiveModel {
            coverage_area_id: Set(coverage_area_id),
            pincode: Set(pincode),
            city: Set(city),
            district: Set(district),
            state: Set(state),
            is_active: Set(true),
            created_at: Set(chrono::Utc::now()),
            ..Default::default()
        };
        Ok(model.insert(self.db).await?)
    }

    pub async fn find_pincodes_by_area(
        &self,
        coverage_area_id: i64,
    ) -> Result<Vec<coverage_pincode::Model>, AppError> {
        Ok(coverage_pincode::Entity::find()
            .filter(coverage_pincode::Column::CoverageAreaId.eq(coverage_area_id))
            .all(self.db)
            .await?)
    }
}
