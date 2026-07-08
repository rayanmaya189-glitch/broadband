use crate::modules::branch::model::branch::Branch;
use crate::modules::branch::response::branch_response::BranchResponse;

pub fn branch_to_response(branch: &Branch) -> BranchResponse {
    BranchResponse {
        id: branch.id,
        name: branch.name.clone(),
        code: branch.code.clone(),
        address: branch.address.clone(),
        city: branch.city.clone(),
        state: branch.state.clone(),
        pincode: branch.pincode.clone(),
        phone: branch.phone.clone(),
        email: branch.email.clone(),
        is_active: branch.is_active,
        timezone: branch.timezone.clone(),
        created_at: branch.created_at,
        updated_at: branch.updated_at,
    }
}
