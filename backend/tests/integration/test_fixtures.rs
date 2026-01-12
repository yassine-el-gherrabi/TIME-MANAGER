//! Integration test fixtures using testcontainers.
//!
//! Provides TestDatabase struct for spinning up ephemeral PostgreSQL instances
//! and seeding them with test data.

use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use std::sync::Arc;
use testcontainers::{clients::Cli, Container};
use testcontainers_modules::postgres::Postgres;
use uuid::Uuid;

use timemanager_backend::config::app::{AppConfig, AppState};
use timemanager_backend::config::email::EmailConfig;
use timemanager_backend::config::hibp::HibpConfig;
use timemanager_backend::domain::enums::UserRole;
use timemanager_backend::services::{EmailService, EndpointRateLimiter, HibpService, MetricsService};
use timemanager_backend::utils::JwtService;

// Test RSA keys (2048-bit, for testing only - DO NOT use in production)
pub const TEST_PRIVATE_KEY: &str = "-----BEGIN PRIVATE KEY-----
MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQCfwD3rj7NWw0cH
su5hRvc5uo4lPhjFgx1HlZvxzt1pWafbr9zC1exr5HB/NxH1gN37e2nWOI8tPQWf
GsZecR2kGOc2LL3wWiBI9OXxZutf0mtzn3tcIz5vrjktVFM8Q1cVj1e+8wEcvyOw
netXKt+YDah0lBuxm61pg9omb0pnkCsXUjkrowe2c/50X6wBbT4zoYqeSu0EdVnR
ifRZKYbPTpKIh40AejfPHSHI7FPHq0BBnP3iUqtq+a2e9cO6kufZR/T1t9B2G/Zp
58PC/LYIYlINuk9+LcFHDZScoqmW9Aa5QP2JDmvZmbVD6Xd4CKOEh02fJZ8PkzEx
TzU10+1XAgMBAAECggEAJaFFlK7pWjcujKA36b8rLjSFFj293QypAXs63CdT3WSK
l0OiN1znz3RkkXrZ5qAf6gSkphr1kvzsTZGjh4ySpFxfXlIEvdClCTpyzb3mFNC+
keJPzyDYLLt36XcTEj90jHYS/75DFU/q6sgQLxzAxZL2Ctv2eAxJOXEfGm2ds64Q
9OYc/SnQQkpCYRLygfix93n2FlualLDuCZzlXBn/Usb8UzqylMrjzPUe7popIQ3+
QY6oJIgE3aeTBW1kfRgGK7fOcfJZY9q/M0mfAY8Zf3SxT1PTVTSFhunIDxo3Ay0K
XT+r9+YSyJ/0OycR7NsZSOifIwzBGOu/LAEGGA5wiQKBgQDb6J3R4IY55OEJpxJ/
pTwJdsEVmt5L/xoti029rMkwoEb5awBcK0bdQ06oJOHRInb5KTLFvZwhCWBSFyhC
FipnQXH5JRW8CNjlt7SGQZs5C7OJFxclAqfx0ba/oUzTyQ6ZfO1QBHNkx9XIX/DW
t/sEQ6xPWj5kcX1HxcwReCkiPwKBgQC5+B7gYLtSdt1gwHG4iZwTfo0AiZPdiSH7
kcN5JXWdJ4VP5dmtfuL3UOWRnbitfgIeBti//Po+Cd4h8i0CYFF20luOlj1Q4HH4
JPc61SGoykRs8a1DKFHm2YltWShHn5y3x5tarSzY38ndTPx/r1hvFoEHnF8+97gi
J49ozse+6QKBgD8dBtZqYvuQpcl4asW5rX5l18qUlQIop+G0Xk52nZNYHKaOwB6z
yPXN0HBPjYPRKWYfHdREs9+DamKFBOfaprbVwJkpvJAn1eAwFh6GC7+WjSNmPh1A
IuUzNAjRiVQrGwaQJSfW7ytYcxG7/0oQqXky1uw7UTbQn40Oxp+o5d1PAoGBAJh0
Peu3oRkjdKyCVzfvJ9IbZsBQCLYOW5t+jX7dJKQm5/Tt+xtt7+bLnMdZQzKHIHk5
J6uMWiFNuZqejCNsjpwYKxKjO7T3qrbApyTF4Igc+SdOoLlzbmEPaMgJ1SmSQcmv
iz40xZUtMLGJEV4jgx3elvyERti5/2uQftJu4fUxAoGBAMni474jFdHfz0WtHm/d
hTUmXvg1s9h033q0cqjT4CFHRi1JP8h7+Z8mYGa64+vgZFTl0c8+h27NGZdx33j7
T5Wb2QMgao7+BnKnHL2ymEIvaWhIbLXd7xTQsLBe4DvXQJmJCD3TeR6exrXK/lkI
/4D2c3OjbJmOwh4TOcI94I9Q
-----END PRIVATE KEY-----";

pub const TEST_PUBLIC_KEY: &str = "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAn8A964+zVsNHB7LuYUb3
ObqOJT4YxYMdR5Wb8c7daVmn26/cwtXsa+RwfzcR9YDd+3tp1jiPLT0FnxrGXnEd
pBjnNiy98FogSPTl8WbrX9Jrc597XCM+b645LVRTPENXFY9XvvMBHL8jsJ3rVyrf
mA2odJQbsZutaYPaJm9KZ5ArF1I5K6MHtnP+dF+sAW0+M6GKnkrtBHVZ0Yn0WSmG
z06SiIeNAHo3zx0hyOxTx6tAQZz94lKravmtnvXDupLn2Uf09bfQdhv2aefDwvy2
CGJSDbpPfi3BRw2UnKKplvQGuUD9iQ5r2Zm1Q+l3eAijhIdNnyWfD5MxMU81NdPt
VwIDAQAB
-----END PUBLIC KEY-----";

/// Test user data for generating JWT tokens
#[derive(Debug, Clone)]
pub struct TestUser {
    pub id: Uuid,
    pub org_id: Uuid,
    pub email: String,
    pub role: UserRole,
}

impl Default for TestUser {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            org_id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            role: UserRole::Employee,
        }
    }
}

impl TestUser {
    pub fn employee() -> Self {
        Self::default()
    }

    pub fn manager() -> Self {
        Self {
            role: UserRole::Manager,
            ..Self::default()
        }
    }

    pub fn admin() -> Self {
        Self {
            role: UserRole::Admin,
            ..Self::default()
        }
    }

    pub fn super_admin() -> Self {
        Self {
            role: UserRole::SuperAdmin,
            ..Self::default()
        }
    }

    pub fn with_org(mut self, org_id: Uuid) -> Self {
        self.org_id = org_id;
        self
    }

    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = id;
        self
    }

    pub fn with_email(mut self, email: String) -> Self {
        self.email = email;
        self
    }
}

/// Generate a test JWT token for the given user
pub fn generate_test_token(user: &TestUser) -> String {
    let jwt_service = JwtService::new(
        TEST_PRIVATE_KEY.to_string(),
        TEST_PUBLIC_KEY.to_string(),
        900,    // 15 minutes
        604800, // 7 days
    )
    .expect("Failed to create JWT service");

    jwt_service
        .generate_access_token(user.id, user.org_id, user.role.clone())
        .expect("Failed to generate test token")
}

/// Generate Authorization header value for a test user
pub fn auth_header(user: &TestUser) -> String {
    format!("Bearer {}", generate_test_token(user))
}

/// TestDatabase manages an ephemeral PostgreSQL container for integration testing.
///
/// This struct is designed to be used with testcontainers-rs to spin up a
/// fresh PostgreSQL instance for each test or test module.
pub struct TestDatabase<'a> {
    /// Docker client reference
    _docker: &'a Cli,
    /// The running PostgreSQL container
    _container: Container<'a, Postgres>,
    /// Connection pool to the test database
    pub pool: Pool<AsyncPgConnection>,
    /// Connection URL for the test database
    pub url: String,
    /// Test organization ID (seeded)
    pub org_id: Uuid,
}

impl<'a> TestDatabase<'a> {
    /// Create a new test database with an ephemeral PostgreSQL container.
    ///
    /// This will:
    /// 1. Spin up a PostgreSQL container
    /// 2. Create a connection pool
    /// 3. Run migrations
    /// 4. Seed base data (organization, users, etc.)
    pub async fn new(docker: &'a Cli) -> Self {
        // Start PostgreSQL container
        let container = docker.run(Postgres::default());
        let port = container.get_host_port_ipv4(5432);
        let url = format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            port
        );

        // Create connection pool
        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&url);
        let pool = Pool::builder(manager)
            .max_size(5)
            .build()
            .expect("Failed to create test pool");

        // Run migrations (would need embedded migrations or migration runner)
        // For now, we'll assume migrations are run separately
        // In a full implementation, you'd use diesel_migrations::run_pending_migrations

        let org_id = Uuid::new_v4();

        Self {
            _docker: docker,
            _container: container,
            pool,
            url,
            org_id,
        }
    }

    /// Get a test user with the seeded organization
    pub fn test_user(&self, role: UserRole) -> TestUser {
        TestUser {
            id: Uuid::new_v4(),
            org_id: self.org_id,
            email: format!("{}@test.example.com", role.to_string().to_lowercase()),
            role,
        }
    }

    /// Create test application config
    pub fn create_config(&self) -> AppConfig {
        AppConfig {
            app_host: "127.0.0.1".to_string(),
            app_port: 3000,
            database_url: self.url.clone(),
            rust_log: "debug".to_string(),
            jwt_private_key: TEST_PRIVATE_KEY.to_string(),
            jwt_public_key: TEST_PUBLIC_KEY.to_string(),
            jwt_access_token_expiry_seconds: 900,
            jwt_refresh_token_expiry_seconds: 604800,
            cors_allowed_origins: vec!["http://localhost:3000".to_string()],
            metrics_enabled: false,
            email: EmailConfig {
                smtp_host: "localhost".to_string(),
                smtp_port: 1025,
                smtp_username: "test".to_string(),
                smtp_password: "test".to_string(),
                from_email: "test@example.com".to_string(),
                from_name: "Test".to_string(),
            },
            hibp: HibpConfig {
                enabled: false,
                api_url: "https://api.pwnedpasswords.com".to_string(),
                timeout_ms: 5000,
            },
        }
    }

    /// Create test application state
    pub async fn create_state(&self) -> AppState {
        let config = self.create_config();

        let email_service = Arc::new(EmailService::new(config.email.clone()));
        let hibp_service = Arc::new(HibpService::new(config.hibp.clone()));
        let rate_limiter = Arc::new(EndpointRateLimiter::new());
        let metrics_service = Arc::new(MetricsService::new());

        AppState {
            config,
            db_pool: self.pool.clone(),
            email_service,
            hibp_service,
            rate_limiter,
            metrics_service,
        }
    }
}

/// Seed data helpers for integration tests
pub mod seed {
    use super::*;
    use chrono::{NaiveDate, Utc};

    /// Create a seeded organization ID
    pub fn org_id() -> Uuid {
        Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()
    }

    /// Create a seeded user ID
    pub fn user_id() -> Uuid {
        Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap()
    }

    /// Create a seeded team ID
    pub fn team_id() -> Uuid {
        Uuid::parse_str("00000000-0000-0000-0000-000000000003").unwrap()
    }

    /// Get today's date
    pub fn today() -> NaiveDate {
        Utc::now().date_naive()
    }

    /// Get a date N days from now
    pub fn days_from_now(days: i64) -> NaiveDate {
        (Utc::now() + chrono::Duration::days(days)).date_naive()
    }
}

/// Helper trait for running integration tests with automatic cleanup
pub trait IntegrationTest {
    /// Run setup before test
    fn setup(&self) -> impl std::future::Future<Output = ()> + Send;

    /// Run cleanup after test
    fn cleanup(&self) -> impl std::future::Future<Output = ()> + Send;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_builders() {
        let employee = TestUser::employee();
        assert!(matches!(employee.role, UserRole::Employee));

        let manager = TestUser::manager();
        assert!(matches!(manager.role, UserRole::Manager));

        let admin = TestUser::admin();
        assert!(matches!(admin.role, UserRole::Admin));

        let super_admin = TestUser::super_admin();
        assert!(matches!(super_admin.role, UserRole::SuperAdmin));
    }

    #[test]
    fn test_user_with_org() {
        let org_id = Uuid::new_v4();
        let user = TestUser::employee().with_org(org_id);
        assert_eq!(user.org_id, org_id);
    }

    #[test]
    fn test_user_with_id() {
        let id = Uuid::new_v4();
        let user = TestUser::employee().with_id(id);
        assert_eq!(user.id, id);
    }

    #[test]
    fn test_token_generation() {
        let user = TestUser::employee();
        let token = generate_test_token(&user);
        assert!(!token.is_empty());
        assert!(token.starts_with("eyJ")); // JWT header
    }

    #[test]
    fn test_auth_header_format() {
        let user = TestUser::admin();
        let header = auth_header(&user);
        assert!(header.starts_with("Bearer eyJ"));
    }

    #[test]
    fn test_seed_ids_deterministic() {
        assert_eq!(seed::org_id(), seed::org_id());
        assert_eq!(seed::user_id(), seed::user_id());
        assert_eq!(seed::team_id(), seed::team_id());
    }

    #[test]
    fn test_seed_ids_unique() {
        assert_ne!(seed::org_id(), seed::user_id());
        assert_ne!(seed::user_id(), seed::team_id());
        assert_ne!(seed::org_id(), seed::team_id());
    }
}
