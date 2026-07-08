use crate::modules::customer::model::customer::Customer;
use crate::modules::customer::response::customer_response::CustomerResponse;

pub fn customer_to_response(c: &Customer) -> CustomerResponse {
    CustomerResponse {
        id: c.id,
        customer_code: c.customer_code.clone(),
        first_name: c.first_name.clone(),
        last_name: c.last_name.clone(),
        email: c.email.clone(),
        phone: c.phone.clone(),
        alternate_phone: c.alternate_phone.clone(),
        status: c.status.clone(),
        branch_id: c.branch_id,
        kyc_status: c.kyc_status.clone(),
        notes: c.notes.clone(),
        created_at: c.created_at,
        updated_at: c.updated_at,
    }
}
