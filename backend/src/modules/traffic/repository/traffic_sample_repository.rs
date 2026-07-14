use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, QueryOrder, PaginatorTrait, Set, ActiveModelTrait};
use crate::common::errors::app_error::AppError;
use crate::modules::traffic::model::traffic_sample_entity::{self, Model as SampleModel};
pub struct TrafficSampleRepository<'a> { db: &'a DatabaseConnection }
impl<'a> TrafficSampleRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }
    pub async fn record(&self, customer_id: Option<i64>, subscription_id: Option<i64>, branch_id: Option<i64>, interface_name: Option<&str>, bytes_in: i64, bytes_out: i64, packets_in: i64, packets_out: i64, sample_duration_seconds: i32) -> Result<SampleModel, AppError> {
        let active = traffic_sample_entity::ActiveModel {
            customer_id: Set(customer_id), subscription_id: Set(subscription_id), branch_id: Set(branch_id),
            interface_name: Set(interface_name.map(|s| s.to_owned())),
            bytes_in: Set(bytes_in), bytes_out: Set(bytes_out),
            packets_in: Set(packets_in), packets_out: Set(packets_out),
            sample_duration_seconds: Set(sample_duration_seconds),
            recorded_at: Set(chrono::Utc::now().into()), ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }
    pub async fn list(&self, customer_id: Option<i64>, page: i64, per_page: i64) -> Result<(Vec<SampleModel>, i64), AppError> {
        let page_size = per_page as u64; let page_num = if per_page > 0 { ((page - 1).max(0) as u64 * page_size) / page_size } else { 0 };
        let mut select = traffic_sample_entity::Entity::find();
        if let Some(cid) = customer_id { select = select.filter(traffic_sample_entity::Column::CustomerId.eq(cid)); }
        let total = select.clone().count(self.db).await? as i64;
        let items = select.order_by_desc(traffic_sample_entity::Column::RecordedAt).paginate(self.db, page_size).fetch_page(page_num).await?;
        Ok((items, total))
    }
}
