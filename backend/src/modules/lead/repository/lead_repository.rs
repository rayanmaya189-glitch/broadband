use sqlx::PgPool;

use crate::modules::lead::model::lead::{Lead, LeadActivity};

pub struct LeadRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> LeadRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }
    pub fn pool(&self) -> &'a PgPool { self.pool }

    pub async fn list(
        &self, branch_id: Option<i64>, status: Option<&str>, source: Option<&str>,
        assigned_to: Option<i64>, page: i64, per_page: i64,
    ) -> Result<(Vec<Lead>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM leads l
             WHERE ($1::bigint IS NULL OR l.branch_id = $1)
             AND ($2::text IS NULL OR l.status = $2)
             AND ($3::text IS NULL OR l.source = $3)
             AND ($4::bigint IS NULL OR l.assigned_to = $4)",
        )
        .bind(branch_id).bind(status).bind(source).bind(assigned_to)
        .fetch_one(self.pool).await?;

        let leads: Vec<Lead> = sqlx::query_as(
            "SELECT l.* FROM leads l
             WHERE ($1::bigint IS NULL OR l.branch_id = $1)
             AND ($2::text IS NULL OR l.status = $2)
             AND ($3::text IS NULL OR l.source = $3)
             AND ($4::bigint IS NULL OR l.assigned_to = $4)
             ORDER BY l.created_at DESC LIMIT $5 OFFSET $6",
        )
        .bind(branch_id).bind(status).bind(source).bind(assigned_to)
        .bind(per_page).bind(offset)
        .fetch_all(self.pool).await?;

        Ok((leads, count_row.0))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Lead>, sqlx::Error> {
        sqlx::query_as::<_, Lead>("SELECT * FROM leads WHERE id = $1")
            .bind(id).fetch_optional(self.pool).await
    }

    pub async fn create(
        &self, branch_id: i64, name: &str, phone: &str, email: Option<&str>,
        source: &str, interested_plan_id: Option<i64>,
        estimated_install_date: Option<chrono::NaiveDate>, address: Option<&str>,
        latitude: Option<f64>, longitude: Option<f64>, notes: Option<&str>,
    ) -> Result<Lead, sqlx::Error> {
        sqlx::query_as::<_, Lead>(
            "INSERT INTO leads (branch_id, name, phone, email, source, interested_plan_id, estimated_install_date, address, latitude, longitude, notes)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) RETURNING *",
        )
        .bind(branch_id).bind(name).bind(phone).bind(email).bind(source)
        .bind(interested_plan_id).bind(estimated_install_date).bind(address)
        .bind(latitude).bind(longitude).bind(notes)
        .fetch_one(self.pool).await
    }

    pub async fn update(
        &self, id: i64, name: Option<&str>, phone: Option<&str>, email: Option<&str>,
        source: Option<&str>, interested_plan_id: Option<i64>,
        estimated_install_date: Option<chrono::NaiveDate>, address: Option<&str>,
        latitude: Option<f64>, longitude: Option<f64>, notes: Option<&str>,
    ) -> Result<Lead, sqlx::Error> {
        sqlx::query_as::<_, Lead>(
            "UPDATE leads SET name = COALESCE($2, name), phone = COALESCE($3, phone), email = COALESCE($4, email),
             source = COALESCE($5, source), interested_plan_id = COALESCE($6, interested_plan_id),
             estimated_install_date = COALESCE($7, estimated_install_date), address = COALESCE($8, address),
             latitude = COALESCE($9, latitude), longitude = COALESCE($10, longitude),
             notes = COALESCE($11, notes), updated_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(id).bind(name).bind(phone).bind(email).bind(source)
        .bind(interested_plan_id).bind(estimated_install_date).bind(address)
        .bind(latitude).bind(longitude).bind(notes)
        .fetch_one(self.pool).await
    }

    pub async fn update_status(&self, id: i64, status: &str, lost_reason: Option<&str>) -> Result<Lead, sqlx::Error> {
        sqlx::query_as::<_, Lead>(
            "UPDATE leads SET status = $2, lost_reason = CASE WHEN $2 = 'lost' THEN $3 ELSE lost_reason END, updated_at = NOW()
             WHERE id = $1 RETURNING *",
        )
        .bind(id).bind(status).bind(lost_reason)
        .fetch_one(self.pool).await
    }

    pub async fn assign(&self, id: i64, assigned_to: i64) -> Result<Lead, sqlx::Error> {
        sqlx::query_as::<_, Lead>(
            "UPDATE leads SET assigned_to = $2, updated_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(id).bind(assigned_to)
        .fetch_one(self.pool).await
    }

    pub async fn convert(&self, id: i64, customer_id: i64) -> Result<Lead, sqlx::Error> {
        sqlx::query_as::<_, Lead>(
            "UPDATE leads SET status = 'converted', converted_customer_id = $2, converted_at = NOW(), updated_at = NOW()
             WHERE id = $1 RETURNING *",
        )
        .bind(id).bind(customer_id)
        .fetch_one(self.pool).await
    }

    pub async fn delete(&self, id: i64) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM leads WHERE id = $1").bind(id).execute(self.pool).await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn list_activities(&self, lead_id: i64) -> Result<Vec<LeadActivity>, sqlx::Error> {
        sqlx::query_as::<_, LeadActivity>(
            "SELECT * FROM lead_activities WHERE lead_id = $1 ORDER BY created_at ASC",
        )
        .bind(lead_id).fetch_all(self.pool).await
    }

    pub async fn add_activity(
        &self, lead_id: i64, activity_type: &str, description: &str,
        performed_by: i64, scheduled_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<LeadActivity, sqlx::Error> {
        sqlx::query_as::<_, LeadActivity>(
            "INSERT INTO lead_activities (lead_id, activity_type, description, performed_by, scheduled_at)
             VALUES ($1, $2, $3, $4, $5) RETURNING *",
        )
        .bind(lead_id).bind(activity_type).bind(description).bind(performed_by).bind(scheduled_at)
        .fetch_one(self.pool).await
    }

    pub async fn get_pipeline_counts(&self) -> Result<Vec<(String, i64)>, sqlx::Error> {
        sqlx::query_as(
            "SELECT status, COUNT(*) FROM leads GROUP BY status ORDER BY
             CASE status WHEN 'new' THEN 0 WHEN 'contacted' THEN 1 WHEN 'interested' THEN 2
             WHEN 'surveyed' THEN 3 WHEN 'quoted' THEN 4 WHEN 'converted' THEN 5 ELSE 6 END",
        )
        .fetch_all(self.pool).await
    }

    pub async fn get_stats(&self) -> Result<(i64, i64), sqlx::Error> {
        sqlx::query_as(
            "SELECT COUNT(*), COUNT(*) FILTER (WHERE status = 'converted' AND converted_at >= date_trunc('month', NOW())) FROM leads",
        )
        .fetch_one(self.pool).await
    }

    pub async fn get_source_counts(&self) -> Result<Vec<(String, i64)>, sqlx::Error> {
        sqlx::query_as("SELECT source, COUNT(*) FROM leads GROUP BY source ORDER BY count DESC")
            .fetch_all(self.pool).await
    }

    pub async fn get_status_counts(&self) -> Result<Vec<(String, i64)>, sqlx::Error> {
        sqlx::query_as("SELECT status, COUNT(*) FROM leads GROUP BY status ORDER BY count DESC")
            .fetch_all(self.pool).await
    }
}
