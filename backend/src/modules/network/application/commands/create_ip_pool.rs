//! Create IP pool command handler.

use crate::common::errors::app_error::AppError;
use crate::modules::network::domain::aggregates::ip_pool::ip_pool::{IpPool, IpPoolError};
use crate::modules::network::domain::rules::network_rules;

/// Command to create an IP pool.
#[derive(Debug, Clone)]
pub struct CreateIpPoolCommand {
    pub name: String,
    pub network_address: String,
    pub subnet_mask: String,
    pub gateway: String,
    pub vlan_id: Option<i64>,
    pub branch_id: i64,
}

/// Command handler for creating IP pools.
pub struct CreateIpPoolHandler;

impl CreateIpPoolHandler {
    pub fn handle(id: i64, command: CreateIpPoolCommand) -> Result<IpPool, AppError> {
        network_rules::validate_ip_address(&command.network_address)
            .map_err(|e| AppError::Validation(e))?;

        network_rules::validate_subnet_mask(&command.subnet_mask)
            .map_err(|e| AppError::Validation(e))?;

        let mut pool = IpPool::create(
            id,
            command.name,
            command.network_address,
            command.subnet_mask,
            command.gateway,
            command.branch_id,
        )
        .map_err(|e| AppError::Validation(e.to_string()))?;

        pool.vlan_id = command.vlan_id;

        Ok(pool)
    }
}
