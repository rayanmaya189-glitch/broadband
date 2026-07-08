use crate::modules::plan::model::plan::Plan;
use crate::modules::plan::response::plan_response::PlanResponse;

pub fn plan_to_response(p: &Plan) -> PlanResponse {
    PlanResponse {
        id: p.id, name: p.name.clone(), code: p.code.clone(), description: p.description.clone(),
        speed_down_mbps: p.speed_down_mbps, speed_up_mbps: p.speed_up_mbps,
        data_cap_gb: p.data_cap_gb, price_monthly: p.price_monthly,
        price_quarterly: p.price_quarterly, price_half_yearly: p.price_half_yearly,
        price_yearly: p.price_yearly, gst_percent: p.gst_percent,
        is_active: p.is_active, is_promotional: p.is_promotional,
        category: p.category.clone(), created_at: p.created_at, updated_at: p.updated_at,
    }
}
