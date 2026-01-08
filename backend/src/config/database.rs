use anyhow::{Context, Result};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Creates a new database connection pool
pub fn create_pool(database_url: &str) -> Result<DbPool> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .max_size(10)
        .min_idle(Some(2))
        .test_on_check_out(true)
        .build(manager)
        .context("Failed to create database connection pool")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Requires actual database
    fn test_pool_creation() {
        let database_url = "postgres://timemanager:password@localhost/timemanager";
        let _pool = create_pool(database_url);
        // Pool creation successful if no panic
    }
}
