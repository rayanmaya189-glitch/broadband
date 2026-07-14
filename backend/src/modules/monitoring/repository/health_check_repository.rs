use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, QueryOrder, PaginatorTrait, Set, ActiveModelTrait};
use crate::common::errors::app_error::AppError;
use crate::modules::monitoring::model::health_check_entity::{self, Model as HealthCheckModel};
pub struct HealthCheckRepository<'a> { db: &'a DatabaseConnection }
impl<'a> HealthCheckRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }
    pub async fn list(&self, service_name: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<HealthCheckModel>, i64), AppError> {
        let page_size = per_page as u64; let page_num = if per_page > 0 { ((page - 1).max(0) as u64 * page_size) / page_size } else { 0 };
        let mut select = health_check_entity::Entity::find();
        if let Some(s) = service_name { select = select.filter(health_check_entity::Column::ServiceName.eq(s)); }
        let total = select.clone().count(self.db).await? as i64;
        let items = select.order_by_desc(health_check_entity::Column::CheckedAt).paginate(self.db, page_size).fetch_page(page_num).await?;
        Ok((items, total))
    }
    pub async fn record(&self, service_name: &str, status: &str, response_time_ms: Option<i32>, error_message: Option<&str>) -> Result<HealthCheckModel, AppError> {
        let active = health_check_entity::ActiveModel {
            service_name: Set(service_name.to_owned()), status: Set(status.to_owned()),
            response_time_ms: Set(response_time_ms), error_message: Set(error_message.map(|s| s.to_owned())),
            checked_at: Set(chrono::Utc::now().into()), ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }
    pub async fn get_latest(&self, service_name: &str) -> Result<Option<HealthCheckModel>, AppError> {
        Ok(health_check_entity::Entity::find().filter(health_check_entity::Column::ServiceName.eq(service_name)).order_by_desc(health_check_entity::Column::CheckedAt).one(self.db).await?)
    }
}
