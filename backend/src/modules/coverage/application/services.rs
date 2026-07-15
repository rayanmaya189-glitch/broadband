use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, Set};
use crate::shared::errors::AppError;
use crate::modules::coverage::domain::entities::{CoverageArea, CoveragePincode, CoverageAreaColumn, CoverageAreaActiveModel};

pub struct CoverageService;

impl CoverageService {
    pub async fn list_areas(db: &DatabaseConnection, branch_id: Option<i64>) -> Result<Vec<crate::modules::coverage::domain::entities::coverage_area::Model>, AppError> {
        let mut query = CoverageArea::find();
        if let Some(bid) = branch_id { query = query.filter(CoverageAreaColumn::BranchId.eq(bid)); }
        Ok(query.all(db).await?)
    }

    pub async fn check_pincode(db: &DatabaseConnection, pincode: &str) -> Result<Option<crate::modules::coverage::domain::entities::coverage_area::Model>, AppError> {
        let pin = CoveragePincode::find()
            .filter(crate::modules::coverage::domain::entities::coverage_pincode::Column::Pincode.eq(pincode))
            .filter(crate::modules::coverage::domain::entities::coverage_pincode::Column::IsActive.eq(true))
            .one(db).await?;
        if let Some(p) = pin {
            let area = CoverageArea::find_by_id(p.coverage_area_id).one(db).await?;
            Ok(area)
        } else {
            Ok(None)
        }
    }

    pub async fn create_area(db: &DatabaseConnection, branch_id: i64, name: String, area_type: String) -> Result<crate::modules::coverage::domain::entities::coverage_area::Model, AppError> {
        let now = chrono::Utc::now();
        let area = CoverageAreaActiveModel {
            branch_id: Set(branch_id), name: Set(name), area_type: Set(area_type),
            is_active: Set(true), fiber_available: Set(Some(true)), estimated_installation_days: Set(Some(3)),
            created_at: Set(now), updated_at: Set(now), ..Default::default()
        };
        Ok(area.insert(db).await?)
    }
}
