use anyhow::{anyhow, Result};
use diesel::sqlite::Sqlite;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../../migrations");

pub fn migrate(conn: &mut impl MigrationHarness<Sqlite>) -> Result<()> {
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|v| anyhow!(v))?;

    Ok(())
}
