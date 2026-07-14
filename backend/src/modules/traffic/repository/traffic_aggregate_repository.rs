use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, QueryOrder, PaginatorTrait};
use crate::common::errors::app_error::AppError;
use crate::modules::traffic::model::traffic_aggregate_entity::{self, Model as AggregateModel};
pub struct TrafficAggregateRepository<'a> { db: &'a DatabaseConnection }
impl<'a> TrafficAggregateRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }
    pub async fn list(&self, customer_id: Option<i64>, period: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<AggregateModel>, i64), AppError> {
        let page_size = per_page as u64; let page_num = if per_page > 0 { ((page - 1).max(0) as u64 * page_size) / page_size } else { 0 };
        let mut select = traffic_aggregate_entity::Entity::find();
        if let Some(cid) = customer_id { select = select.filter(traffic_aggregate_entity::Column::CustomerId.eq(cid)); }
        if let Some(p) = period { select = select.filter(traffic_aggregate_entity::Column::Period.eq(p)); }
        let total = select.clone().count(self.db).await? as i64;
        let items = select.order_by_desc(traffic_aggregate_entity::Column::PeriodStart).paginate(self.db, page_size).fetch_page(page_num).await?;
        Ok((items, total))
    }
}
