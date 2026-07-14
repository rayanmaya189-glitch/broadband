//! Network application service.

use crate::common::errors::app_error::AppError;
use crate::modules::network::domain::aggregates::vlan::vlan::Vlan;

/// Application service for network operations.
pub struct NetworkApplicationService;

impl NetworkApplicationService {
    /// Create a new VLAN.
    pub async fn create_vlan(
        &self,
        id: i64,
        command: crate::modules::network::application::commands::create_vlan::CreateVlanCommand,
    ) -> Result<Vlan, AppError> {
        crate::modules::network::application::commands::create_vlan::CreateVlanHandler::handle(id, command)
    }
}
