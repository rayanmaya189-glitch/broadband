use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, QueryOrder, PaginatorTrait, Set, ActiveModelTrait};
use crate::common::errors::app_error::AppError;
use crate::modules::monitoring::model::metric_entity::{self, Model as MetricModel};
pub struct MetricRepository<'a> { db: &'a DatabaseConnection }
impl<'a> MetricRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }
    pub async fn record(&self, metric_name: &str, metric_type: &str, value: f64, tags: Option<serde_json::Value>) -> Result<MetricModel, AppError> {
        let active = metric_entity::ActiveModel {
            metric_name: Set(metric_name.to_owned()), metric_type: Set(metric_type.to_owned()),
            value: Set(value), tags: Set(tags), recorded_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }
    pub async fn list(&self, metric_name: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<MetricModel>, i64), AppError> {
        let page_size = per_page as u64; let page_num = if per_page > 0 { ((page - 1).max(0) as u64 * page_size) / page_size } else { 0 };
        let mut select = metric_entity::Entity::find();
        if let Some(m) = metric_name { select = select.filter(metric_entity::Column::MetricName.eq(m)); }
        let total = select.clone().count(self.db).await? as i64;
        let items = select.order_by_desc(metric_entity::Column::RecordedAt).paginate(self.db, page_size).fetch_page(page_num).await?;
        Ok((items, total))
    }
}
