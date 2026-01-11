use anyhow::{Context, Result};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

pub type DbPool = Pool<AsyncPgConnection>;

/// Creates a new async database connection pool
pub async fn create_pool(database_url: &str) -> Result<DbPool> {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);

    Pool::builder(config)
        .max_size(10)
        .build()
        .context("Failed to create database connection pool")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires actual database
    async fn test_pool_creation() {
        let database_url = "postgres://timemanager:password@localhost/timemanager";
        let _pool = create_pool(database_url).await;
        // Pool creation successful if no panic
    }
}
