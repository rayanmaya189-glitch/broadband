use sea_orm::{ConnectOptions, DatabaseConnection, Database};

/// Create a SeaORM database connection pool.
pub async fn create_database_pool(database_url: &str, max_connections: u32) -> anyhow::Result<DatabaseConnection> {
    let mut opt = ConnectOptions::new(database_url.to_string());
    opt.max_connections(max_connections)
        .min_connections(5)
        .connect_timeout(std::time::Duration::from_secs(30))
        .idle_timeout(std::time::Duration::from_secs(600));

    let db = Database::connect(opt).await?;
    Ok(db)
}

/// Helper to set history context variables for PostgreSQL triggers.
pub async fn set_history_context(
    db: &DatabaseConnection,
    user_id: i64,
    branch_id: Option<i64>,
    ip_address: Option<&str>,
    user_agent: Option<&str>,
) -> Result<(), sea_orm::DbErr> {
    use sea_orm::{ConnectionTrait, Statement};

    db.execute(Statement::from_string(
        db.get_database_backend(),
        format!("SELECT set_config('app.current_user_id', '{}', true)", user_id),
    ))
    .await?;

    if let Some(branch) = branch_id {
        db.execute(Statement::from_string(
            db.get_database_backend(),
            format!("SELECT set_config('app.current_branch_id', '{}', true)", branch),
        ))
        .await?;
    }

    if let Some(ip) = ip_address {
        db.execute(Statement::from_string(
            db.get_database_backend(),
            format!("SELECT set_config('app.current_ip_address', '{}', true)", ip),
        ))
        .await?;
    }

    if let Some(ua) = user_agent {
        db.execute(Statement::from_string(
            db.get_database_backend(),
            format!("SELECT set_config('app.current_user_agent', '{}', true)", ua),
        ))
        .await?;
    }

    Ok(())
}
