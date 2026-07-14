//! Network repository trait.

use async_trait::async_trait;

use crate::common::errors::app_error::AppError;
use crate::modules::network::domain::aggregates::vlan::vlan::Vlan;

/// Repository trait for network operations.
#[async_trait]
pub trait NetworkRepositoryTrait: Send + Sync {
    /// Find VLAN by ID.
    async fn find_vlan_by_id(&self, id: i64) -> Result<Option<Vlan>, AppError>;

    /// Save a VLAN.
    async fn save_vlan(&self, vlan: &mut Vlan) -> Result<(), AppError>;

    /// Update a VLAN.
    async fn update_vlan(&self, vlan: &Vlan) -> Result<(), AppError>;

    /// List VLANs for a branch.
    async fn list_vlans_by_branch(&self, branch_id: i64) -> Result<Vec<Vlan>, AppError>;
}
