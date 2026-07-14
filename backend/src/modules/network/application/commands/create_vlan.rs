//! Create VLAN command handler.

use crate::common::errors::app_error::AppError;
use crate::modules::network::domain::aggregates::vlan::vlan::{Vlan, VlanError};
use crate::modules::network::domain::rules::network_rules;

/// Command to create a VLAN.
#[derive(Debug, Clone)]
pub struct CreateVlanCommand {
    pub vlan_number: i32,
    pub name: String,
    pub branch_id: i64,
    pub description: Option<String>,
}

/// Command handler for creating VLANs.
pub struct CreateVlanHandler;

impl CreateVlanHandler {
    pub fn handle(id: i64, command: CreateVlanCommand) -> Result<Vlan, AppError> {
        network_rules::validate_vlan_number(command.vlan_number)
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let mut vlan = Vlan::create(id, command.vlan_number, command.name, command.branch_id)
            .map_err(|e| AppError::Validation(e.to_string()))?;

        vlan.description = command.description;

        Ok(vlan)
    }
}
