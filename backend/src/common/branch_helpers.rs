//! Shared helpers for looking up branch-level data (state, GSTIN).
//!
//! These were previously duplicated in both `billing_repository` and
//! `accounting_repository`.  Both now call the functions here, passing
//! a reference to the database connection.

use sea_orm::{DatabaseConnection, EntityTrait};

use crate::common::errors::app_error::AppError;

/// Return the `state` field of a branch, or `None` if the branch
/// does not exist / has no state set.
pub async fn get_branch_state(
    db: &DatabaseConnection,
    branch_id: i64,
) -> Result<Option<String>, AppError> {
    use crate::modules::branch::model::branch_entity;
    let branch = branch_entity::Entity::find_by_id(branch_id)
        .one(db)
        .await?;
    Ok(branch.and_then(|b| b.state))
}

/// Return the `gstin` field of a branch, or `None` if the branch
/// does not exist / has no GSTIN set.
pub async fn get_branch_gstin(
    db: &DatabaseConnection,
    branch_id: i64,
) -> Option<String> {
    use crate::modules::branch::model::branch_entity;
    branch_entity::Entity::find_by_id(branch_id)
        .one(db)
        .await
        .ok()
        .flatten()
        .and_then(|b| b.gstin)
}
