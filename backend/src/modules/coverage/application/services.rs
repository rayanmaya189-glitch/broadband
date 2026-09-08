use crate::modules::coverage::domain::entities::{
    CoverageArea, CoverageAreaActiveModel, CoverageAreaColumn, CoveragePincode,
};
use crate::shared::errors::AppError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};

pub struct CoverageService;

impl CoverageService {
    pub async fn list_areas(
        db: &DatabaseConnection,
        branch_id: Option<i64>,
        _page: u64,
        _limit: u64,
    ) -> Result<
        (
            Vec<crate::modules::coverage::domain::entities::coverage_area::Model>,
            u64,
        ),
        AppError,
    > {
        let mut query = CoverageArea::find();
        if let Some(bid) = branch_id {
            query = query.filter(CoverageAreaColumn::BranchId.eq(bid));
        }
        let total = query.clone().count(db).await?;
        let areas = query.all(db).await?;
        Ok((areas, total))
    }

    pub async fn check_pincode(
        db: &DatabaseConnection,
        pincode: &str,
    ) -> Result<Option<crate::modules::coverage::domain::entities::coverage_area::Model>, AppError>
    {
        let pin = CoveragePincode::find()
            .filter(
                crate::modules::coverage::domain::entities::coverage_pincode::Column::Pincode
                    .eq(pincode),
            )
            .filter(
                crate::modules::coverage::domain::entities::coverage_pincode::Column::IsActive
                    .eq(true),
            )
            .one(db)
            .await?;
        if let Some(p) = pin {
            let area = CoverageArea::find_by_id(p.coverage_area_id).one(db).await?;
            Ok(area)
        } else {
            Ok(None)
        }
    }

    pub async fn create_area(
        db: &DatabaseConnection,
        branch_id: i64,
        name: String,
        area_type: String,
    ) -> Result<crate::modules::coverage::domain::entities::coverage_area::Model, AppError> {
        let now = chrono::Utc::now();
        let area = CoverageAreaActiveModel {
            branch_id: Set(branch_id),
            name: Set(name),
            area_type: Set(area_type),
            is_active: Set(true),
            fiber_available: Set(Some(true)),
            estimated_installation_days: Set(Some(3)),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(area.insert(db).await?)
    }

    pub async fn update_area(
        db: &DatabaseConnection,
        id: i64,
        name: String,
        area_type: String,
    ) -> Result<crate::modules::coverage::domain::entities::coverage_area::Model, AppError> {
        let area = CoverageArea::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Coverage area {} not found", id)))?;
        let mut active: CoverageAreaActiveModel = area.into();
        active.name = Set(name);
        active.area_type = Set(area_type);
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(db).await?)
    }

    /// Soft-delete a coverage area: deactivate it and its pincodes.
    pub async fn delete_area(db: &DatabaseConnection, id: i64) -> Result<(), AppError> {
        use crate::modules::coverage::domain::entities::coverage_pincode::Column as CoveragePincodeColumn;
        use crate::modules::coverage::domain::entities::{
            CoveragePincode, CoveragePincodeActiveModel,
        };
        let area = CoverageArea::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Coverage area {} not found", id)))?;
        let mut active: CoverageAreaActiveModel = area.into();
        active.is_active = Set(false);
        active.updated_at = Set(chrono::Utc::now());
        active.update(db).await?;

        let pins = CoveragePincode::find()
            .filter(CoveragePincodeColumn::CoverageAreaId.eq(id))
            .all(db)
            .await?;
        for pin in pins {
            let mut pactive: CoveragePincodeActiveModel = pin.into();
            pactive.is_active = Set(false);
            pactive.update(db).await?;
        }
        Ok(())
    }
}
