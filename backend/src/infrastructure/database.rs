use sea_orm::{ConnectOptions, Database, DatabaseConnection};

/// Create a SeaORM database connection pool.
pub async fn create_database_pool(
    database_url: &str,
    max_connections: u32,
    min_connections: u32,
    connect_timeout_secs: u64,
    idle_timeout_secs: u64,
) -> anyhow::Result<DatabaseConnection> {
    let mut opt = ConnectOptions::new(database_url.to_string());
    opt.max_connections(max_connections)
        .min_connections(min_connections)
        .connect_timeout(std::time::Duration::from_secs(connect_timeout_secs))
        .idle_timeout(std::time::Duration::from_secs(idle_timeout_secs));

    let db = Database::connect(opt).await?;
    Ok(db)
}

/// Helper to set history context variables for PostgreSQL triggers.
/// SECURITY: All values are parameterized to prevent SQL injection.
pub async fn set_history_context(
    db: &DatabaseConnection,
    user_id: i64,
    branch_id: Option<i64>,
    ip_address: Option<&str>,
    user_agent: Option<&str>,
) -> Result<(), sea_orm::DbErr> {
    use sea_orm::{ConnectionTrait, Statement, Value as SeaValue};

    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "SELECT set_config('app.current_user_id', $1, true)",
        vec![SeaValue::BigInt(Some(user_id))],
    ))
    .await?;

    if let Some(branch) = branch_id {
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "SELECT set_config('app.current_branch_id', $1, true)",
            vec![SeaValue::BigInt(Some(branch))],
        ))
        .await?;
    }

    if let Some(ip) = ip_address {
        // Sanitize IP address: only allow valid IPv4/IPv6 patterns
        let sanitized_ip = sanitize_ip_address(ip);
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "SELECT set_config('app.current_ip_address', $1, true)",
            vec![SeaValue::String(Some(Box::new(sanitized_ip)))],
        ))
        .await?;
    }

    if let Some(ua) = user_agent {
        // Truncate user agent to prevent abuse
        let truncated_ua: String = ua.chars().take(512).collect();
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "SELECT set_config('app.current_user_agent', $1, true)",
            vec![SeaValue::String(Some(Box::new(truncated_ua)))],
        ))
        .await?;
    }

    Ok(())
}

/// Sanitize IP address — strip anything that isn't a valid IP character.
/// Returns "unknown" if the input is empty or completely invalid.
fn sanitize_ip_address(ip: &str) -> String {
    let sanitized: String = ip
        .chars()
        .filter(|c| {
            c.is_ascii_digit()
                || *c == '.'
                || *c == ':'
                || *c == 'a'
                || *c == 'f'
                || *c == 'A'
                || *c == 'F'
        })
        .take(45) // IPv6 max length
        .collect();
    if sanitized.is_empty() {
        "unknown".to_string()
    } else {
        sanitized
    }
}
