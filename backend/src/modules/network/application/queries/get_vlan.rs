//! Get VLAN query handler.

use crate::common::errors::app_error::AppError;
use crate::modules::network::domain::aggregates::vlan::vlan::Vlan;

/// Query to get a VLAN by ID.
#[derive(Debug, Clone)]
pub struct GetVlanQuery {
    pub vlan_id: i64,
}

/// Query handler for getting a VLAN.
pub struct GetVlanHandler;

impl GetVlanHandler {
    pub fn execute(vlan: Option<Vlan>) -> Result<Vlan, AppError> {
        vlan.ok_or_else(|| AppError::NotFound("VLAN not found".to_string()))
    }
}
