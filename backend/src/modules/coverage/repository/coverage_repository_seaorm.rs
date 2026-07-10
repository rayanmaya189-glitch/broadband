//! SeaORM-based repository for the Coverage domain.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};

use crate::common::errors::app_error::AppError;
use crate::modules::coverage::model::coverage_area_entity::{self, Model as CoverageAreaModel};
use crate::modules::coverage::model::coverage_pincode_entity::{self, Model as CoveragePincodeModel};

pub struct CoverageRepositorySeaorm<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> CoverageRepositorySeaorm<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    pub async fn list(&self, branch_id: Option<i64>) -> Result<Vec<CoverageAreaModel>, AppError> {
        let mut select = coverage_area_entity::Entity::find()
            .filter(coverage_area_entity::Column::IsActive.eq(true));
        if let Some(bid) = branch_id {
            select = select.filter(coverage_area_entity::Column::BranchId.eq(bid));
        }
        let areas = select.order_by_asc(coverage_area_entity::Column::Name).all(self.db).await?;
        Ok(areas)
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<CoverageAreaModel>, AppError> {
        Ok(coverage_area_entity::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn create(
        &self, branch_id: i64, name: &str, description: Option<&str>, area_type: &str,
        fiber_available: bool, est_days: Option<i32>, max_customers: Option<i32>,
    ) -> Result<CoverageAreaModel, AppError> {
        let now = chrono::Utc::now();
        let active = coverage_area_entity::ActiveModel {
            branch_id: Set(branch_id),
            name: Set(name.to_owned()),
            description: Set(description.map(|s| s.to_owned())),
            area_type: Set(area_type.to_owned()),
            fiber_available: Set(fiber_available),
            estimated_installation_days: Set(est_days),
            max_customers: Set(max_customers),
            is_active: Set(true),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn update(
        &self, id: i64, name: Option<&str>, description: Option<&str>, area_type: Option<&str>,
        fiber_available: Option<bool>, est_days: Option<i32>, max_customers: Option<i32>,
    ) -> Result<CoverageAreaModel, AppError> {
        let existing = coverage_area_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Coverage area not found".into()))?;
        let mut active = existing.into_active_model();
        if let Some(v) = name { active.name = Set(v.to_owned()); }
        if let Some(v) = description { active.description = Set(Some(v.to_owned())); }
        if let Some(v) = area_type { active.area_type = Set(v.to_owned()); }
        if let Some(v) = fiber_available { active.fiber_available = Set(v); }
        if let Some(v) = est_days { active.estimated_installation_days = Set(Some(v)); }
        if let Some(v) = max_customers { active.max_customers = Set(Some(v)); }
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn deactivate(&self, id: i64) -> Result<bool, AppError> {
        let existing = coverage_area_entity::Entity::find_by_id(id).one(self.db).await?;
        match existing {
            Some(e) => {
                let mut active = e.into_active_model();
                active.is_active = Set(false);
                active.updated_at = Set(chrono::Utc::now().into());
                active.update(self.db).await?;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    pub async fn check_pincode(&self, pincode: &str) -> Result<Option<CoverageAreaModel>, AppError> {
        // Complex join query - use raw SQL
        let stmt = sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT ca.* FROM coverage_areas ca JOIN coverage_pincode_map cpm ON ca.id = cpm.coverage_area_id WHERE cpm.pincode = $1 AND ca.is_active = true LIMIT 1".to_string()
        );
        let results = coverage_area_entity::Entity::find()
            .from_raw_sql(stmt)
            .all(self.db).await?;
        Ok(results.into_iter().next())
    }

    pub async fn list_pincodes(&self, area_id: i64) -> Result<Vec<CoveragePincodeModel>, AppError> {
        let pincodes = coverage_pincode_entity::Entity::find()
            .filter(coverage_pincode_entity::Column::CoverageAreaId.eq(area_id))
            .order_by_asc(coverage_pincode_entity::Column::Pincode)
            .all(self.db).await?;
        Ok(pincodes)
    }

    pub async fn add_pincode(&self, area_id: i64, pincode: &str, city: &str, district: Option<&str>, state: Option<&str>) -> Result<CoveragePincodeModel, AppError> {
        // Check if exists
        let existing = coverage_pincode_entity::Entity::find()
            .filter(coverage_pincode_entity::Column::CoverageAreaId.eq(area_id))
            .filter(coverage_pincode_entity::Column::Pincode.eq(pincode))
            .one(self.db).await?;
        match existing {
            Some(e) => {
                let mut active = e.into_active_model();
                active.is_active = Set(true);
                Ok(active.update(self.db).await?)
            }
            None => {
                let now = chrono::Utc::now();
                let active = coverage_pincode_entity::ActiveModel {
                    coverage_area_id: Set(area_id),
                    pincode: Set(pincode.to_owned()),
                    city: Set(city.to_owned()),
                    district: Set(district.map(|s| s.to_owned())),
                    state: Set(state.map(|s| s.to_owned())),
                    is_active: Set(true),
                    created_at: Set(now.into()),
                    ..Default::default()
                };
                Ok(active.insert(self.db).await?)
            }
        }
    }

    pub async fn remove_pincode(&self, area_id: i64, pincode: &str) -> Result<bool, AppError> {
        let result = coverage_pincode_entity::Entity::delete_many()
            .filter(coverage_pincode_entity::Column::CoverageAreaId.eq(area_id))
            .filter(coverage_pincode_entity::Column::Pincode.eq(pincode))
            .exec(self.db).await?;
        Ok(result.rows_affected > 0)
    }
}
