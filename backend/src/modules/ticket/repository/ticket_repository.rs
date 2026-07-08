use sqlx::PgPool;

use crate::modules::ticket::model::ticket::{Ticket, TicketComment};

pub struct TicketRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> TicketRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }
    pub fn pool(&self) -> &'a PgPool { self.pool }

    pub async fn list(
        &self,
        branch_id: Option<i64>,
        status: Option<&str>,
        priority: Option<&str>,
        category: Option<&str>,
        assigned_to: Option<i64>,
        customer_id: Option<i64>,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<Ticket>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM tickets t
             WHERE ($1::bigint IS NULL OR t.branch_id = $1)
             AND ($2::text IS NULL OR t.status = $2)
             AND ($3::text IS NULL OR t.priority = $3)
             AND ($4::text IS NULL OR t.category = $4)
             AND ($5::bigint IS NULL OR t.assigned_to = $5)
             AND ($6::bigint IS NULL OR t.customer_id = $6)",
        )
        .bind(branch_id).bind(status).bind(priority).bind(category).bind(assigned_to).bind(customer_id)
        .fetch_one(self.pool).await?;

        let tickets: Vec<Ticket> = sqlx::query_as(
            "SELECT t.* FROM tickets t
             WHERE ($1::bigint IS NULL OR t.branch_id = $1)
             AND ($2::text IS NULL OR t.status = $2)
             AND ($3::text IS NULL OR t.priority = $3)
             AND ($4::text IS NULL OR t.category = $4)
             AND ($5::bigint IS NULL OR t.assigned_to = $5)
             AND ($6::bigint IS NULL OR t.customer_id = $6)
             ORDER BY t.created_at DESC LIMIT $7 OFFSET $8",
        )
        .bind(branch_id).bind(status).bind(priority).bind(category).bind(assigned_to).bind(customer_id)
        .bind(per_page).bind(offset)
        .fetch_all(self.pool).await?;

        Ok((tickets, count_row.0))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Ticket>, sqlx::Error> {
        sqlx::query_as::<_, Ticket>("SELECT * FROM tickets WHERE id = $1")
            .bind(id).fetch_optional(self.pool).await
    }

    pub async fn create(
        &self, ticket_number: &str, branch_id: i64, customer_id: Option<i64>,
        subscription_id: Option<i64>, created_by: i64, category: &str,
        subcategory: Option<&str>, priority: &str, subject: &str,
        description: &str, source: &str,
    ) -> Result<Ticket, sqlx::Error> {
        sqlx::query_as::<_, Ticket>(
            "INSERT INTO tickets (ticket_number, branch_id, customer_id, subscription_id, created_by, category, subcategory, priority, subject, description, source)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) RETURNING *",
        )
        .bind(ticket_number).bind(branch_id).bind(customer_id).bind(subscription_id)
        .bind(created_by).bind(category).bind(subcategory).bind(priority)
        .bind(subject).bind(description).bind(source)
        .fetch_one(self.pool).await
    }

    pub async fn update_status(&self, id: i64, status: &str, resolution_notes: Option<&str>) -> Result<Ticket, sqlx::Error> {
        sqlx::query_as::<_, Ticket>(
            "UPDATE tickets SET status = $2, resolution_notes = COALESCE($3, resolution_notes),
             resolved_at = CASE WHEN $2 = 'resolved' THEN NOW() ELSE resolved_at END,
             closed_at = CASE WHEN $2 = 'closed' THEN NOW() ELSE closed_at END,
             updated_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(id).bind(status).bind(resolution_notes)
        .fetch_one(self.pool).await
    }

    pub async fn assign(&self, id: i64, assigned_to: i64) -> Result<Ticket, sqlx::Error> {
        sqlx::query_as::<_, Ticket>(
            "UPDATE tickets SET assigned_to = $2, status = 'assigned', updated_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(id).bind(assigned_to)
        .fetch_one(self.pool).await
    }

    pub async fn escalate(&self, id: i64, escalated_to: i64, new_priority: Option<&str>) -> Result<Ticket, sqlx::Error> {
        sqlx::query_as::<_, Ticket>(
            "UPDATE tickets SET escalated_to = $2, priority = COALESCE($3, priority), status = 'escalated', updated_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(id).bind(escalated_to).bind(new_priority)
        .fetch_one(self.pool).await
    }

    pub async fn increment_reopen(&self, id: i64) -> Result<Ticket, sqlx::Error> {
        sqlx::query_as::<_, Ticket>(
            "UPDATE tickets SET reopen_count = reopen_count + 1, status = 'open', updated_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(id).fetch_one(self.pool).await
    }

    pub async fn set_feedback(&self, id: i64, rating: Option<i32>, feedback: Option<&str>) -> Result<Ticket, sqlx::Error> {
        sqlx::query_as::<_, Ticket>(
            "UPDATE tickets SET satisfaction_rating = $2, satisfaction_feedback = $3, updated_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(id).bind(rating).bind(feedback)
        .fetch_one(self.pool).await
    }

    pub async fn get_my_assignments(&self, assigned_to: i64, page: i64, per_page: i64) -> Result<(Vec<Ticket>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM tickets WHERE assigned_to = $1 AND status NOT IN ('resolved', 'closed')",
        )
        .bind(assigned_to).fetch_one(self.pool).await?;

        let tickets: Vec<Ticket> = sqlx::query_as(
            "SELECT * FROM tickets WHERE assigned_to = $1 AND status NOT IN ('resolved', 'closed')
             ORDER BY CASE priority WHEN 'critical' THEN 0 WHEN 'high' THEN 1 WHEN 'medium' THEN 2 ELSE 3 END, created_at ASC
             LIMIT $2 OFFSET $3",
        )
        .bind(assigned_to).bind(per_page).bind(offset)
        .fetch_all(self.pool).await?;

        Ok((tickets, count_row.0))
    }

    pub async fn get_dashboard_stats(&self) -> Result<(i64, i64, i64, i64), sqlx::Error> {
        sqlx::query_as(
            "SELECT
                COUNT(*) FILTER (WHERE status IN ('open', 'assigned')),
                COUNT(*) FILTER (WHERE status IN ('in_progress', 'waiting_customer', 'escalated')),
                COUNT(*) FILTER (WHERE status = 'resolved' AND resolved_at::date = CURRENT_DATE),
                COUNT(*) FILTER (WHERE status IN ('open', 'assigned', 'in_progress') AND sla_resolution_at < NOW())
             FROM tickets",
        )
        .fetch_one(self.pool).await
    }

    pub async fn get_priority_counts(&self) -> Result<Vec<(String, i64)>, sqlx::Error> {
        sqlx::query_as(
            "SELECT priority, COUNT(*) FROM tickets WHERE status NOT IN ('resolved', 'closed') GROUP BY priority ORDER BY count DESC",
        )
        .fetch_all(self.pool).await
    }

    pub async fn get_category_counts(&self) -> Result<Vec<(String, i64)>, sqlx::Error> {
        sqlx::query_as(
            "SELECT category, COUNT(*) FROM tickets WHERE status NOT IN ('resolved', 'closed') GROUP BY category ORDER BY count DESC",
        )
        .fetch_all(self.pool).await
    }

    pub async fn delete(&self, id: i64) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM tickets WHERE id = $1").bind(id).execute(self.pool).await?;
        Ok(result.rows_affected() > 0)
    }

    // ──── Comments ────

    pub async fn list_comments(&self, ticket_id: i64) -> Result<Vec<TicketComment>, sqlx::Error> {
        sqlx::query_as::<_, TicketComment>(
            "SELECT * FROM ticket_comments WHERE ticket_id = $1 ORDER BY created_at ASC",
        )
        .bind(ticket_id).fetch_all(self.pool).await
    }

    pub async fn add_comment(
        &self, ticket_id: i64, user_id: Option<i64>, is_customer: bool,
        comment: &str, is_internal: bool, attachments: Option<serde_json::Value>,
    ) -> Result<TicketComment, sqlx::Error> {
        sqlx::query_as::<_, TicketComment>(
            "INSERT INTO ticket_comments (ticket_id, user_id, is_customer, comment, is_internal, attachments)
             VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        )
        .bind(ticket_id).bind(user_id).bind(is_customer).bind(comment).bind(is_internal).bind(attachments)
        .fetch_one(self.pool).await
    }
}
