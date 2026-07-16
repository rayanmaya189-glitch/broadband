use aeroxe_backend::migration::Migrator;
use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://aeroxe:aeroxe_secret@localhost:5432/aeroxe".to_string());

    let db = sea_orm::Database::connect(&database_url).await?;

    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("up");

    match command {
        "up" => {
            tracing::info!("Running all pending migrations...");
            Migrator::up(&db, None).await?;
            tracing::info!("Migrations completed successfully");
        }
        "down" => {
            tracing::info!("Rolling back last migration...");
            Migrator::down(&db, Some(1)).await?;
            tracing::info!("Rollback completed successfully");
        }
        "status" => {
            tracing::info!("Migration status:");
            let status = Migrator::get_pending_migrations(&db).await?;
            tracing::info!("Pending migrations: {}", status.len());
            // Just print count - Migration type doesn't implement Display or Debug
        }
        "fresh" => {
            tracing::info!("Dropping all tables and re-running migrations...");
            Migrator::fresh(&db).await?;
            tracing::info!("Fresh migration completed successfully");
        }
        _ => {
            eprintln!("Usage: migrate [up|down|status|fresh]");
            std::process::exit(1);
        }
    }

    Ok(())
}
