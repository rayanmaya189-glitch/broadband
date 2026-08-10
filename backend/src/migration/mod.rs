//! AeroXe Backend - Database Migration Runner
//!
//! SeaORM migrations for all 17 database schema changes.
//! Run with: `cargo run -- migrate` or `cargo run -- migrate --apply`

use sea_orm_migration::prelude::*;

mod m001_create_extensions;
mod m002_create_branches;
mod m003_create_rbac;
mod m004_create_customers;
mod m005_create_plans;
mod m006_create_subscriptions;
mod m007_create_billing;
mod m008_create_accounting;
mod m009_create_devices;
mod m010_create_network;
mod m011_create_tickets;
mod m012_create_notifications;
mod m013_create_audit;
mod m014_create_events;
mod m015_create_documents;
mod m016_seed_roles_permissions;
mod m017_seed_initial_plans;
mod m018_add_2fa_backup_codes;
mod m019_create_schemas;
mod m020_move_tables_to_schemas;
mod m021_create_missing_tables;
mod m022_add_fk_constraints_and_indexes;
mod m023_add_gst_tax_breakdown;
mod m024_widen_status_checks;
mod m025_make_payment_invoice_nullable;
mod m026_seed_gateway_scheduler_permissions;
mod m027_create_otp_codes;
mod m028_fix_notification_channel_seeds;
mod m029_seed_users_branch;
mod m030_add_role_is_company_wide;
mod m031_add_missing_runtime_columns;
mod m032_realign_approval_requests;
mod m033_align_column_types;
mod m034_seed_missing_permissions;
mod m035_grant_read_permissions;
mod m036_remove_unenforced_permissions;
mod m037_widen_subscription_review_status;

pub struct Migrator;

/// Split raw SQL text into individual statements, respecting single-quoted
/// string literals (with `''` escaping), `--` line comments, and `$tag$`-style
/// dollar-quoted blocks (e.g. `DO $$ ... $$`).
pub fn split_sql_statements(sql: &str) -> Vec<String> {
    fn dollar_delim(chars: &[char], i: usize) -> Option<(String, usize)> {
        if chars.get(i) != Some(&'$') {
            return None;
        }
        let mut j = i + 1;
        let mut tag = String::new();
        while j < chars.len() {
            match chars[j] {
                '$' => {
                    let delim = if tag.is_empty() {
                        "$$".to_string()
                    } else {
                        format!("${}$", tag)
                    };
                    return Some((delim, j + 1));
                }
                c if c.is_ascii_alphanumeric() || c == '_' => {
                    tag.push(c);
                    j += 1;
                }
                _ => return None,
            }
        }
        None
    }

    let mut statements = Vec::new();
    let mut current = String::new();
    let chars: Vec<char> = sql.chars().collect();
    let n = chars.len();
    let mut i = 0;

    while i < n {
        let c = chars[i];

        // `--` line comment
        if c == '-' && chars.get(i + 1) == Some(&'-') {
            while i < n && chars[i] != '\n' {
                i += 1;
            }
            if i < n {
                current.push('\n');
                i += 1;
            }
            continue;
        }

        // Single-quoted string literal with '' escaping
        if c == '\'' {
            current.push(c);
            i += 1;
            while i < n {
                current.push(chars[i]);
                if chars[i] == '\'' {
                    if chars.get(i + 1) == Some(&'\'') {
                        current.push(chars[i + 1]);
                        i += 2;
                        continue;
                    }
                    i += 1;
                    break;
                }
                i += 1;
            }
            continue;
        }

        // Dollar-quoted block (e.g. DO $$ ... END $$)
        if c == '$' {
            if let Some((delim, end)) = dollar_delim(&chars, i) {
                current.push_str(&delim);
                i = end;
                while i < n {
                    if let Some((end_delim, end_idx)) = dollar_delim(&chars, i) {
                        if end_delim == delim {
                            current.push_str(&delim);
                            i = end_idx;
                            break;
                        }
                    }
                    current.push(chars[i]);
                    i += 1;
                }
                continue;
            }
        }

        // Statement separator
        if c == ';' {
            let stmt = current.trim();
            if !stmt.is_empty() {
                statements.push(stmt.to_string());
            }
            current.clear();
            i += 1;
            continue;
        }

        current.push(c);
        i += 1;
    }

    let stmt = current.trim();
    if !stmt.is_empty() {
        statements.push(stmt.to_string());
    }

    statements
}

/// Helper to execute raw SQL from a migration file
pub async fn exec_sql_file(manager: &SchemaManager<'_>, sql: &str) -> Result<(), DbErr> {
    let conn = manager.get_connection();
    for stmt in split_sql_statements(sql) {
        let stmt = stmt.trim();
        if !stmt.is_empty() && !stmt.starts_with("--") {
            conn.execute_unprepared(stmt).await?;
        }
    }
    Ok(())
}

/// Helper to drop multiple tables in reverse dependency order
pub async fn drop_tables(manager: &SchemaManager<'_>, tables: Vec<&str>) -> Result<(), DbErr> {
    let conn = manager.get_connection();
    for table in tables {
        conn.execute_unprepared(&format!("DROP TABLE IF EXISTS {} CASCADE", table))
            .await?;
    }
    Ok(())
}

/// Helper to execute a single raw SQL statement
pub async fn exec_stmt_raw(manager: &SchemaManager<'_>, sql: &str) -> Result<(), DbErr> {
    manager.get_connection().execute_unprepared(sql).await?;
    Ok(())
}

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m001_create_extensions::Migration),
            Box::new(m002_create_branches::Migration),
            Box::new(m003_create_rbac::Migration),
            Box::new(m004_create_customers::Migration),
            Box::new(m005_create_plans::Migration),
            Box::new(m006_create_subscriptions::Migration),
            Box::new(m007_create_billing::Migration),
            Box::new(m008_create_accounting::Migration),
            Box::new(m009_create_devices::Migration),
            Box::new(m010_create_network::Migration),
            Box::new(m011_create_tickets::Migration),
            Box::new(m012_create_notifications::Migration),
            Box::new(m013_create_audit::Migration),
            Box::new(m014_create_events::Migration),
            Box::new(m015_create_documents::Migration),
            Box::new(m016_seed_roles_permissions::Migration),
            Box::new(m017_seed_initial_plans::Migration),
            Box::new(m018_add_2fa_backup_codes::Migration),
            Box::new(m019_create_schemas::Migration),
            Box::new(m020_move_tables_to_schemas::Migration),
            Box::new(m021_create_missing_tables::Migration),
            Box::new(m022_add_fk_constraints_and_indexes::Migration),
            Box::new(m023_add_gst_tax_breakdown::Migration),
            Box::new(m024_widen_status_checks::Migration),
            Box::new(m025_make_payment_invoice_nullable::Migration),
            Box::new(m026_seed_gateway_scheduler_permissions::Migration),
            Box::new(m027_create_otp_codes::Migration),
            Box::new(m028_fix_notification_channel_seeds::Migration),
            Box::new(m029_seed_users_branch::Migration),
            Box::new(m030_add_role_is_company_wide::Migration),
            Box::new(m031_add_missing_runtime_columns::Migration),
            Box::new(m032_realign_approval_requests::Migration),
            Box::new(m033_align_column_types::Migration),
            Box::new(m034_seed_missing_permissions::Migration),
            Box::new(m035_grant_read_permissions::Migration),
            Box::new(m036_remove_unenforced_permissions::Migration),
            Box::new(m037_widen_subscription_review_status::Migration),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::split_sql_statements;

    #[test]
    fn splits_on_semicolons() {
        let sql = "CREATE TABLE a (id INT);\nCREATE TABLE b (id INT);";
        let stmts = split_sql_statements(sql);
        assert_eq!(stmts.len(), 2);
        assert!(stmts[0].starts_with("CREATE TABLE a"));
        assert!(stmts[1].starts_with("CREATE TABLE b"));
    }

    #[test]
    fn ignores_line_comments() {
        let sql = "-- header comment\nSELECT 1;\n-- trailing\nSELECT 2;";
        let stmts = split_sql_statements(sql);
        assert_eq!(stmts.len(), 2);
        assert!(!stmts[0].contains("--"));
    }

    #[test]
    fn keeps_escaped_quotes_intact() {
        let sql = "INSERT INTO t (v) VALUES ('it''s fine');";
        let stmts = split_sql_statements(sql);
        assert_eq!(stmts.len(), 1);
        assert!(stmts[0].contains("it''s fine"));
    }

    #[test]
    fn captures_dollar_quoted_do_block_as_one_statement() {
        let sql = "SELECT 1;\nDO $$\nDECLARE\n  x TEXT;\nBEGIN\n  EXECUTE format('GRANT USAGE ON SCHEMA %I TO CURRENT_USER', x);\nEND $$;\nSELECT 2;";
        let stmts = split_sql_statements(sql);
        assert_eq!(stmts.len(), 3);
        assert!(stmts[1].starts_with("DO $$"));
        assert!(stmts[1].contains("GRANT USAGE"));
        assert!(stmts[1].ends_with("$$"));
    }

    #[test]
    fn split_migration_019_do_block() {
        let sql = include_str!("../../migrations/sql/019_move_tables_to_schemas.sql");
        let stmts = split_sql_statements(sql);
        let do_stmts: Vec<&String> = stmts.iter().filter(|s| s.starts_with("DO $$")).collect();
        assert_eq!(do_stmts.len(), 1, "expected exactly one DO block in 019");
        assert!(do_stmts[0].contains("GRANT USAGE ON SCHEMA"));
        let partition_defaults = stmts
            .iter()
            .filter(|s| s.contains("PARTITION OF"))
            .collect::<Vec<_>>();
        assert_eq!(partition_defaults.len(), 3);
    }
}
