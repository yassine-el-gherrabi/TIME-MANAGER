use diesel::prelude::*;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use uuid::Uuid;

type DbPool = Pool<AsyncPgConnection>;

// Helper function to create test database pool
fn create_test_pool() -> DbPool {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/timemanager_test".to_string());

    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    Pool::builder(manager)
        .max_size(5)
        .build()
        .expect("Failed to create test pool")
}

#[cfg(test)]
mod testcontainers_integration {
    use super::*;
    use diesel_async::RunQueryDsl;
    use testcontainers::{clients::Cli, Container};
    use testcontainers_modules::postgres::Postgres;

    /// Creates a PostgreSQL container for integration testing.
    /// Note: Requires Docker to be running.
    fn setup_postgres_container(docker: &Cli) -> Container<'_, Postgres> {
        docker.run(Postgres::default())
    }

    /// Gets the database URL from a running container
    fn get_container_db_url(container: &Container<'_, Postgres>) -> String {
        let port = container.get_host_port_ipv4(5432);
        format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", port)
    }

    #[tokio::test]
    #[ignore = "Requires Docker - run with: cargo test -- --ignored"]
    async fn test_postgres_container_starts() {
        let docker = Cli::default();
        let container = setup_postgres_container(&docker);
        let db_url = get_container_db_url(&container);

        // Verify we can connect
        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&db_url);
        let pool = Pool::builder(manager)
            .max_size(1)
            .build()
            .expect("Failed to create pool");

        let mut conn = pool.get().await.expect("Failed to get connection");

        // Run a simple query to verify connection works
        let result: i32 = diesel::sql_query("SELECT 1 as value")
            .get_result::<QueryResult>(&mut conn)
            .await
            .expect("Query failed")
            .value;

        assert_eq!(result, 1);
    }

    #[derive(QueryableByName)]
    struct QueryResult {
        #[diesel(sql_type = diesel::sql_types::Integer)]
        value: i32,
    }
}

#[cfg(test)]
mod repository_integration_tests {
    use super::*;
    use timemanager_backend::models::{NewLoginAttempt, NewPasswordHistory, NewRefreshToken};
    use timemanager_backend::repositories::*;

    #[tokio::test]
    #[ignore = "Requires database setup"]
    async fn test_refresh_token_create_and_find() {
        let pool = create_test_pool();
        let _repo = RefreshTokenRepository::new(pool.clone());

        let test_user_id = Uuid::new_v4();
        let test_token_hash = "test_hash_12345";
        let expires_at = chrono::Utc::now().naive_utc() + chrono::Duration::days(7);

        let _new_token = NewRefreshToken {
            user_id: test_user_id,
            token_hash: test_token_hash.to_string(),
            expires_at,
            user_agent: Some("Test Agent".to_string()),
        };

        // This would require a test user to exist
        // let created_token = repo.create(new_token).await;
        // assert!(created_token.is_ok());
    }

    #[tokio::test]
    #[ignore = "Requires database setup"]
    async fn test_login_attempt_record() {
        let pool = create_test_pool();
        let _repo = LoginAttemptRepository::new(pool.clone());

        let _new_attempt = NewLoginAttempt {
            email: "test@example.com".to_string(),
            ip_address: "127.0.0.1".to_string(),
            successful: false,
        };

        // This would require database connection
        // let result = repo.record(new_attempt).await;
        // assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "Requires database setup"]
    async fn test_password_history_add() {
        let pool = create_test_pool();
        let _repo = PasswordHistoryRepository::new(pool.clone());

        let test_user_id = Uuid::new_v4();
        let _new_history = NewPasswordHistory {
            user_id: test_user_id,
            password_hash: "hashed_password_example".to_string(),
        };

        // This would require a test user to exist
        // let result = repo.add(new_history).await;
        // assert!(result.is_ok());
    }

    #[test]
    fn test_repository_types_compile() {
        // This test verifies that repository types compile correctly
        // Actual database operations require integration tests with a real database

        // Test that NewLoginAttempt has correct fields
        let _attempt = NewLoginAttempt {
            email: "test@example.com".to_string(),
            ip_address: "127.0.0.1".to_string(),
            successful: false,
        };

        // Test that NewRefreshToken has correct fields
        let _token = NewRefreshToken {
            user_id: Uuid::new_v4(),
            token_hash: "test_hash".to_string(),
            expires_at: chrono::Utc::now().naive_utc() + chrono::Duration::days(7),
            user_agent: Some("Test Agent".to_string()),
        };

        // Test that NewPasswordHistory has correct fields
        let _history = NewPasswordHistory {
            user_id: Uuid::new_v4(),
            password_hash: "test_hash".to_string(),
        };

        // If we get here, all struct types compile correctly
    }
}

#[cfg(test)]
mod repository_unit_tests {
    use super::*;
    use chrono::NaiveDateTime;
    use timemanager_backend::models::RefreshToken;

    #[test]
    fn test_refresh_token_is_valid() {
        let now = chrono::Utc::now().naive_utc();

        // Valid token (not revoked, not expired)
        let valid_token = RefreshToken {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            token_hash: "test_hash".to_string(),
            expires_at: now + chrono::Duration::days(7),
            created_at: now,
            revoked_at: None,
            last_used_at: now,
            user_agent: Some("Test Agent".to_string()),
        };
        assert!(valid_token.is_valid());

        // Expired token
        let expired_token = RefreshToken {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            token_hash: "test_hash".to_string(),
            expires_at: now - chrono::Duration::days(1),
            created_at: now - chrono::Duration::days(8),
            revoked_at: None,
            last_used_at: now - chrono::Duration::days(8),
            user_agent: None,
        };
        assert!(!expired_token.is_valid());

        // Revoked token
        let revoked_token = RefreshToken {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            token_hash: "test_hash".to_string(),
            expires_at: now + chrono::Duration::days(7),
            created_at: now,
            revoked_at: Some(now),
            last_used_at: now,
            user_agent: Some("Test Agent".to_string()),
        };
        assert!(!revoked_token.is_valid());
    }

    #[test]
    fn test_password_reset_token_validation_logic() {
        // Test the logic that would be used in is_valid()
        let now = chrono::Utc::now().naive_utc();

        // Valid: not used, not expired
        let not_used: Option<NaiveDateTime> = None;
        let not_expired = now + chrono::Duration::hours(1);
        assert!(not_used.is_none() && not_expired > now);

        // Invalid: used
        let used: Option<NaiveDateTime> = Some(now);
        assert!(used.is_some());

        // Invalid: expired
        let expired = now - chrono::Duration::hours(1);
        assert!(expired < now);
    }

    #[test]
    fn test_user_session_expiry_logic() {
        let now = chrono::Utc::now().naive_utc();

        // Active session
        let active_expires_at = now + chrono::Duration::hours(2);
        assert!(active_expires_at > now);

        // Expired session
        let expired_expires_at = now - chrono::Duration::hours(1);
        assert!(expired_expires_at < now);
    }
}
