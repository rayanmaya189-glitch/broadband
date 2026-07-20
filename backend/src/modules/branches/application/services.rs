use crate::modules::branches::domain::entities::{branch, branch_user, branch_working_hours};
use crate::shared::errors::AppError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};

pub struct BranchService;

impl BranchService {
    pub async fn list_branches(db: &DatabaseConnection) -> Result<Vec<branch::Model>, AppError> {
        let branches = branch::Entity::find()
            .filter(branch::Column::IsActive.eq(true))
            .order_by_asc(branch::Column::Name)
            .all(db)
            .await?;
        Ok(branches)
    }

    pub async fn get_branch(db: &DatabaseConnection, id: i64) -> Result<branch::Model, AppError> {
        branch::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Branch {} not found", id)))
    }

    pub async fn create_branch(
        db: &DatabaseConnection,
        name: String,
        slug: String,
        code: String,
        city: String,
        state: String,
        address: Option<String>,
        phone: Option<String>,
        email: Option<String>,
    ) -> Result<branch::Model, AppError> {
        let now = chrono::Utc::now();
        let new_branch = branch::ActiveModel {
            name: Set(name),
            slug: Set(slug),
            code: Set(code),
            city: Set(city),
            state: Set(state),
            address: Set(address),
            phone: Set(phone),
            email: Set(email),
            timezone: Set("Asia/Kolkata".to_string()),
            is_active: Set(true),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        let result = new_branch.insert(db).await?;
        Ok(result)
    }

    pub async fn update_branch(
        db: &DatabaseConnection,
        id: i64,
        name: Option<String>,
        city: Option<String>,
        state: Option<String>,
        address: Option<String>,
        phone: Option<String>,
        email: Option<String>,
    ) -> Result<branch::Model, AppError> {
        let branch_model = branch::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Branch {} not found", id)))?;
        let mut active: branch::ActiveModel = branch_model.into();
        if let Some(n) = name {
            active.name = Set(n);
        }
        if let Some(c) = city {
            active.city = Set(c);
        }
        if let Some(s) = state {
            active.state = Set(s);
        }
        if let Some(a) = address {
            active.address = Set(Some(a));
        }
        if let Some(p) = phone {
            active.phone = Set(Some(p));
        }
        if let Some(e) = email {
            active.email = Set(Some(e));
        }
        active.updated_at = Set(chrono::Utc::now());
        let result = active.update(db).await?;
        Ok(result)
    }

    pub async fn deactivate_branch(db: &DatabaseConnection, id: i64) -> Result<(), AppError> {
        let branch_model = branch::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Branch {} not found", id)))?;
        let mut active: branch::ActiveModel = branch_model.into();
        active.is_active = Set(false);
        active.updated_at = Set(chrono::Utc::now());
        active.update(db).await?;
        Ok(())
    }

    pub async fn get_hierarchy(db: &DatabaseConnection) -> Result<serde_json::Value, AppError> {
        let branches = branch::Entity::find()
            .filter(branch::Column::IsActive.eq(true))
            .order_by_asc(branch::Column::State)
            .order_by_asc(branch::Column::Name)
            .all(db)
            .await?;

        // Group by state to form hierarchy regions
        let mut regions: std::collections::HashMap<String, Vec<serde_json::Value>> = std::collections::HashMap::new();
        for b in branches {
            let node = serde_json::json!({
                "id": b.id,
                "name": b.name,
                "slug": b.slug,
                "code": b.code,
                "city": b.city,
                "state": b.state,
            });
            regions.entry(b.state).or_default().push(node);
        }

        let hierarchy: Vec<serde_json::Value> = regions
            .into_iter()
            .map(|(state, branches)| {
                serde_json::json!({
                    "region": state,
                    "branch_count": branches.len(),
                    "branches": branches,
                })
            })
            .collect();

        Ok(serde_json::json!({
            "total_regions": hierarchy.len(),
            "total_branches": hierarchy.iter().map(|r| r["branch_count"].as_u64().unwrap_or(0)).sum::<u64>(),
            "regions": hierarchy,
        }))
    }

    // ─── Working Hours ──────────────────────────────────────────────────

    pub async fn get_working_hours(
        db: &DatabaseConnection,
        branch_id: i64,
    ) -> Result<Vec<branch_working_hours::Model>, AppError> {
        Self::get_branch(db, branch_id).await?;
        let hours = branch_working_hours::Entity::find()
            .filter(branch_working_hours::Column::BranchId.eq(branch_id))
            .order_by_asc(branch_working_hours::Column::DayOfWeek)
            .all(db)
            .await?;
        Ok(hours)
    }

    pub async fn update_working_hours(
        db: &DatabaseConnection,
        branch_id: i64,
        hours: Vec<(i32, String, String, bool)>,
    ) -> Result<Vec<branch_working_hours::Model>, AppError> {
        Self::get_branch(db, branch_id).await?;
        let now = chrono::Utc::now();
        let mut results = Vec::new();
        for (day_of_week, open_time, close_time, is_closed) in hours {
            let existing = branch_working_hours::Entity::find()
                .filter(branch_working_hours::Column::BranchId.eq(branch_id))
                .filter(branch_working_hours::Column::DayOfWeek.eq(day_of_week))
                .one(db)
                .await?;
            match existing {
                Some(m) => {
                    let mut active: branch_working_hours::ActiveModel = m.into();
                    active.open_time = Set(open_time);
                    active.close_time = Set(close_time);
                    active.is_closed = Set(is_closed);
                    active.updated_at = Set(now);
                    results.push(active.update(db).await?);
                }
                None => {
                    let active = branch_working_hours::ActiveModel {
                        branch_id: Set(branch_id),
                        day_of_week: Set(day_of_week),
                        open_time: Set(open_time),
                        close_time: Set(close_time),
                        is_closed: Set(is_closed),
                        created_at: Set(now),
                        updated_at: Set(now),
                        ..Default::default()
                    };
                    results.push(active.insert(db).await?);
                }
            }
        }
        Ok(results)
    }

    // ─── Branch Stats ───────────────────────────────────────────────────

    pub async fn get_branch_stats(
        db: &DatabaseConnection,
        branch_id: i64,
    ) -> Result<serde_json::Value, AppError> {
        Self::get_branch(db, branch_id).await?;

        let customer_count = branch_user::Entity::find()
            .filter(branch_user::Column::BranchId.eq(branch_id))
            .count(db)
            .await? as u64;

        let hours = Self::get_working_hours(db, branch_id).await?;

        Ok(serde_json::json!({
            "branch_id": branch_id,
            "assigned_users": customer_count,
            "working_hours_count": hours.len(),
        }))
    }

    // ─── Branch Users ───────────────────────────────────────────────────

    pub async fn assign_user(
        db: &DatabaseConnection,
        branch_id: i64,
        user_id: i64,
        role: String,
    ) -> Result<branch_user::Model, AppError> {
        Self::get_branch(db, branch_id).await?;
        let now = chrono::Utc::now();
        let active = branch_user::ActiveModel {
            branch_id: Set(branch_id),
            user_id: Set(user_id),
            role: Set(role),
            assigned_at: Set(now),
            created_at: Set(now),
            ..Default::default()
        };
        let result = active.insert(db).await?;
        Ok(result)
    }

    pub async fn remove_user(
        db: &DatabaseConnection,
        branch_id: i64,
        user_id: i64,
    ) -> Result<(), AppError> {
        let record = branch_user::Entity::find()
            .filter(branch_user::Column::BranchId.eq(branch_id))
            .filter(branch_user::Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User {} not assigned to branch {}", user_id, branch_id)))?;
        record.delete(db).await?;
        Ok(())
    }
}
