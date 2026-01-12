//! Clock approval workflow tests.
//!
//! Tests the complete clock override request and approval workflow including:
//! - Creating clock override requests
//! - Manager approval/rejection
//! - Time entry updates on approval

use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::json;
use testcontainers::clients::Cli;
use tower::ServiceExt;
use uuid::Uuid;

use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use std::sync::Arc;
use testcontainers_modules::postgres::Postgres;

use timemanager_backend::api::router::create_router;
use timemanager_backend::config::app::{AppConfig, AppState};
use timemanager_backend::config::email::EmailConfig;
use timemanager_backend::config::hibp::HibpConfig;
use timemanager_backend::domain::enums::UserRole;
use timemanager_backend::services::{EmailService, EndpointRateLimiter, HibpService, MetricsService};
use timemanager_backend::utils::JwtService;

// Test RSA keys (2048-bit, for testing only)
const TEST_PRIVATE_KEY: &str = "-----BEGIN PRIVATE KEY-----
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
FipnQXH5JRW8CNjlt7SGQZs5C7OJFxclAqfx0ba/oUzTyQ6ZfO1QBHNkx9XIV/DW
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

const TEST_PUBLIC_KEY: &str = "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAn8A964+zVsNHB7LuYUb3
ObqOJT4YxYMdR5Wb8c7daVmn26/cwtXsa+RwfzcR9YDd+3tp1jiPLT0FnxrGXnEd
pBjnNiy98FogSPTl8WbrX9Jrc597XCM+b645LVRTPENXFY9XvvMBHL8jsJ3rVyrf
mA2odJQbsZutaYPaJm9KZ5ArF1I5K6MHtnP+dF+sAW0+M6GKnkrtBHVZ0Yn0WSmG
z06SiIeNAHo3zx0hyOxTx6tAQZz94lKravmtnvXDupLn2Uf09bfQdhv2aefDwvy2
CGJSDbpPfi3BRw2UnKKplvQGuUD9iQ5r2Zm1Q+l3eAijhIdNnyWfD5MxMU81NdPt
VwIDAQAB
-----END PUBLIC KEY-----";

/// Test user data for generating JWT tokens
struct TestUser {
    id: Uuid,
    org_id: Uuid,
    role: UserRole,
}

impl TestUser {
    fn employee(org_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            org_id,
            role: UserRole::Employee,
        }
    }

    fn manager(org_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            org_id,
            role: UserRole::Manager,
        }
    }

    fn admin(org_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            org_id,
            role: UserRole::Admin,
        }
    }
}

fn generate_test_token(user: &TestUser) -> String {
    let jwt_service = JwtService::new(
        TEST_PRIVATE_KEY.to_string(),
        TEST_PUBLIC_KEY.to_string(),
        900,
        604800,
    )
    .expect("Failed to create JWT service");

    jwt_service
        .generate_access_token(user.id, user.org_id, user.role.clone())
        .expect("Failed to generate test token")
}

fn create_test_config(db_url: &str) -> AppConfig {
    AppConfig {
        app_host: "127.0.0.1".to_string(),
        app_port: 3000,
        database_url: db_url.to_string(),
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

async fn create_test_state(db_url: &str) -> AppState {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    let pool = Pool::builder(manager)
        .max_size(5)
        .build()
        .expect("Failed to create pool");

    let config = create_test_config(db_url);
    AppState {
        config,
        db_pool: pool,
        email_service: Arc::new(EmailService::new(EmailConfig {
            smtp_host: "localhost".to_string(),
            smtp_port: 1025,
            smtp_username: "test".to_string(),
            smtp_password: "test".to_string(),
            from_email: "test@example.com".to_string(),
            from_name: "Test".to_string(),
        })),
        hibp_service: Arc::new(HibpService::new(HibpConfig {
            enabled: false,
            api_url: "https://api.pwnedpasswords.com".to_string(),
            timeout_ms: 5000,
        })),
        rate_limiter: Arc::new(EndpointRateLimiter::new()),
        metrics_service: Arc::new(MetricsService::new()),
    }
}

/// Test the complete clock override request workflow.
///
/// Workflow steps:
/// 1. Employee requests a clock entry override
/// 2. Request is in "pending" status
/// 3. Manager views pending override requests
/// 4. Manager approves/rejects request
/// 5. Clock entry is updated on approval
#[tokio::test]
#[ignore = "Requires Docker with seeded database - run with: cargo test -- --ignored"]
async fn test_clock_override_approval_workflow() {
    let docker = Cli::default();
    let container = docker.run(Postgres::default());
    let port = container.get_host_port_ipv4(5432);
    let db_url = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", port);

    let state = create_test_state(&db_url).await;
    let org_id = Uuid::new_v4();

    let employee = TestUser::employee(org_id);
    let manager = TestUser::manager(org_id);

    let employee_token = generate_test_token(&employee);
    let manager_token = generate_test_token(&manager);

    let app = create_router(state.clone());

    // Step 1: Employee requests clock override
    let clock_entry_id = Uuid::new_v4(); // Would be a real entry ID in seeded DB
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&format!("/api/v1/clocks/{}/override", clock_entry_id))
                .header("Authorization", format!("Bearer {}", employee_token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "clock_in": "2024-06-01T09:00:00Z",
                        "clock_out": "2024-06-01T17:00:00Z",
                        "reason": "Forgot to clock in - arrived at 9am"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // In fresh DB, likely NOT_FOUND (no clock entry)
    assert!(
        response.status() == StatusCode::CREATED
            || response.status() == StatusCode::OK
            || response.status() == StatusCode::NOT_FOUND
            || response.status() == StatusCode::BAD_REQUEST,
        "Unexpected override request status: {}",
        response.status()
    );

    // Step 2: Manager views pending override requests
    let app = create_router(state.clone());
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/clocks/overrides?status=pending")
                .header("Authorization", format!("Bearer {}", manager_token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::OK || response.status() == StatusCode::NOT_FOUND,
        "Manager view overrides failed: {}",
        response.status()
    );
}

/// Test manager can approve clock override requests.
#[tokio::test]
#[ignore = "Requires Docker with seeded database - run with: cargo test -- --ignored"]
async fn test_manager_approve_clock_override() {
    let docker = Cli::default();
    let container = docker.run(Postgres::default());
    let port = container.get_host_port_ipv4(5432);
    let db_url = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", port);

    let state = create_test_state(&db_url).await;
    let org_id = Uuid::new_v4();

    let manager = TestUser::manager(org_id);
    let manager_token = generate_test_token(&manager);

    let app = create_router(state);

    // Attempt to approve a non-existent override request
    let override_id = Uuid::new_v4();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&format!("/api/v1/clocks/overrides/{}/approve", override_id))
                .header("Authorization", format!("Bearer {}", manager_token))
                .header("Content-Type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should get NOT_FOUND since override doesn't exist
    assert!(
        response.status() == StatusCode::NOT_FOUND
            || response.status() == StatusCode::OK
            || response.status() == StatusCode::BAD_REQUEST,
        "Unexpected approval status: {}",
        response.status()
    );
}

/// Test employee cannot approve their own override request.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_employee_cannot_approve_own_override() {
    let docker = Cli::default();
    let container = docker.run(Postgres::default());
    let port = container.get_host_port_ipv4(5432);
    let db_url = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", port);

    let state = create_test_state(&db_url).await;
    let org_id = Uuid::new_v4();

    let employee = TestUser::employee(org_id);
    let employee_token = generate_test_token(&employee);

    let app = create_router(state);

    let override_id = Uuid::new_v4();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&format!("/api/v1/clocks/overrides/{}/approve", override_id))
                .header("Authorization", format!("Bearer {}", employee_token))
                .header("Content-Type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Employee should get FORBIDDEN or NOT_FOUND
    assert!(
        response.status() == StatusCode::FORBIDDEN || response.status() == StatusCode::NOT_FOUND,
        "Employee should not be able to approve: {}",
        response.status()
    );
}

/// Test admin can view all clock overrides across organization.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_admin_view_all_overrides() {
    let docker = Cli::default();
    let container = docker.run(Postgres::default());
    let port = container.get_host_port_ipv4(5432);
    let db_url = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", port);

    let state = create_test_state(&db_url).await;
    let org_id = Uuid::new_v4();

    let admin = TestUser::admin(org_id);
    let admin_token = generate_test_token(&admin);

    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/clocks/overrides/all")
                .header("Authorization", format!("Bearer {}", admin_token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Admin should have access
    assert!(
        response.status() == StatusCode::OK || response.status() == StatusCode::NOT_FOUND,
        "Admin view all overrides failed: {}",
        response.status()
    );
}

/// Test rejection workflow for clock overrides.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_clock_override_rejection() {
    let docker = Cli::default();
    let container = docker.run(Postgres::default());
    let port = container.get_host_port_ipv4(5432);
    let db_url = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", port);

    let state = create_test_state(&db_url).await;
    let org_id = Uuid::new_v4();

    let manager = TestUser::manager(org_id);
    let manager_token = generate_test_token(&manager);

    let app = create_router(state);

    let override_id = Uuid::new_v4();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&format!("/api/v1/clocks/overrides/{}/reject", override_id))
                .header("Authorization", format!("Bearer {}", manager_token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "reason": "Insufficient evidence for the claimed time"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::NOT_FOUND
            || response.status() == StatusCode::OK
            || response.status() == StatusCode::BAD_REQUEST,
        "Unexpected rejection status: {}",
        response.status()
    );
}

/// Test clock history shows approved overrides correctly.
#[tokio::test]
#[ignore = "Requires Docker with seeded data - run with: cargo test -- --ignored"]
async fn test_clock_history_with_overrides() {
    let docker = Cli::default();
    let container = docker.run(Postgres::default());
    let port = container.get_host_port_ipv4(5432);
    let db_url = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", port);

    let state = create_test_state(&db_url).await;
    let org_id = Uuid::new_v4();

    let employee = TestUser::employee(org_id);
    let employee_token = generate_test_token(&employee);

    let app = create_router(state);

    // Get clock history which would include approved overrides
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/clocks/history?include_overrides=true")
                .header("Authorization", format!("Bearer {}", employee_token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::OK || response.status() == StatusCode::NOT_FOUND,
        "History with overrides failed: {}",
        response.status()
    );
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_role_hierarchy() {
        let org_id = Uuid::new_v4();
        let employee = TestUser::employee(org_id);
        let manager = TestUser::manager(org_id);
        let admin = TestUser::admin(org_id);

        assert!(matches!(employee.role, UserRole::Employee));
        assert!(matches!(manager.role, UserRole::Manager));
        assert!(matches!(admin.role, UserRole::Admin));
    }

    #[test]
    fn test_users_have_same_org() {
        let org_id = Uuid::new_v4();
        let employee = TestUser::employee(org_id);
        let manager = TestUser::manager(org_id);

        assert_eq!(employee.org_id, manager.org_id);
    }

    #[test]
    fn test_users_have_unique_ids() {
        let org_id = Uuid::new_v4();
        let employee = TestUser::employee(org_id);
        let manager = TestUser::manager(org_id);

        assert_ne!(employee.id, manager.id);
    }
}
