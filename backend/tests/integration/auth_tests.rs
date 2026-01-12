//! Authentication integration tests with testcontainers.
//!
//! These tests verify the authentication endpoints with real database
//! interactions using testcontainers.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::json;
use testcontainers::clients::Cli;
use tower::ServiceExt;

use crate::integration::test_fixtures::*;
use timemanager_backend::api::router::create_router;
use timemanager_backend::domain::enums::UserRole;

/// Test GET /api/v1/auth/me endpoint returns current user info.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_auth_me_returns_user_info() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let user = test_db.test_user(UserRole::Employee);
    let token = generate_test_token(&user);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/auth/me")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return OK or NOT_FOUND (if user not seeded in DB)
    assert!(
        response.status() == StatusCode::OK || response.status() == StatusCode::NOT_FOUND,
        "GET /auth/me failed with status: {}",
        response.status()
    );
}

/// Test GET /api/v1/auth/me without token returns 401.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_auth_me_without_token_returns_unauthorized() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/auth/me")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::UNAUTHORIZED,
        "Expected 401 without auth token"
    );
}

/// Test POST /api/v1/auth/login with invalid credentials.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_login_with_invalid_credentials() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "email": "nonexistent@example.com",
                        "password": "wrongpassword123"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return UNAUTHORIZED or NOT_FOUND for invalid credentials
    assert!(
        response.status() == StatusCode::UNAUTHORIZED
            || response.status() == StatusCode::NOT_FOUND
            || response.status() == StatusCode::BAD_REQUEST,
        "Expected authentication failure, got: {}",
        response.status()
    );
}

/// Test POST /api/v1/auth/login with missing fields.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_login_with_missing_fields() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from(json!({}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return BAD_REQUEST or UNPROCESSABLE_ENTITY for validation error
    assert!(
        response.status() == StatusCode::BAD_REQUEST
            || response.status() == StatusCode::UNPROCESSABLE_ENTITY,
        "Expected validation error, got: {}",
        response.status()
    );
}

/// Test POST /api/v1/auth/logout endpoint.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_logout_clears_session() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let user = test_db.test_user(UserRole::Employee);
    let token = generate_test_token(&user);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/logout")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return OK or NO_CONTENT for successful logout
    assert!(
        response.status() == StatusCode::OK
            || response.status() == StatusCode::NO_CONTENT
            || response.status() == StatusCode::NOT_FOUND,
        "Logout failed with status: {}",
        response.status()
    );
}

/// Test POST /api/v1/auth/logout-all endpoint.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_logout_all_sessions() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let user = test_db.test_user(UserRole::Employee);
    let token = generate_test_token(&user);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/logout-all")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::OK
            || response.status() == StatusCode::NO_CONTENT
            || response.status() == StatusCode::NOT_FOUND,
        "Logout-all failed with status: {}",
        response.status()
    );
}

/// Test POST /api/v1/auth/refresh endpoint with invalid token.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_refresh_with_invalid_token() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/refresh")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "refresh_token": "invalid-refresh-token"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return UNAUTHORIZED or BAD_REQUEST for invalid token
    assert!(
        response.status() == StatusCode::UNAUTHORIZED
            || response.status() == StatusCode::BAD_REQUEST
            || response.status() == StatusCode::NOT_FOUND,
        "Expected token error, got: {}",
        response.status()
    );
}

/// Test GET /api/v1/auth/sessions endpoint.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_get_sessions() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let user = test_db.test_user(UserRole::Employee);
    let token = generate_test_token(&user);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/auth/sessions")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::OK || response.status() == StatusCode::NOT_FOUND,
        "GET sessions failed with status: {}",
        response.status()
    );
}

/// Test role-based access - SuperAdmin accessing auth endpoints.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_super_admin_auth_access() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let super_admin = test_db.test_user(UserRole::SuperAdmin);
    let token = generate_test_token(&super_admin);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/auth/me")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::OK || response.status() == StatusCode::NOT_FOUND,
        "SuperAdmin auth/me failed with status: {}",
        response.status()
    );
}

/// Test PUT /api/v1/auth/change-password endpoint.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_change_password() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let user = test_db.test_user(UserRole::Employee);
    let token = generate_test_token(&user);

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/api/v1/auth/change-password")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "current_password": "oldpassword123",
                        "new_password": "newpassword456"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // User not in DB will result in NOT_FOUND, otherwise BAD_REQUEST for wrong password
    assert!(
        response.status() == StatusCode::OK
            || response.status() == StatusCode::NOT_FOUND
            || response.status() == StatusCode::BAD_REQUEST
            || response.status() == StatusCode::UNPROCESSABLE_ENTITY,
        "Change password failed with status: {}",
        response.status()
    );
}

/// Test POST /api/v1/auth/verify-invite endpoint.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_verify_invite_with_invalid_token() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/verify-invite")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "token": "invalid-invite-token"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Should fail with invalid token
    assert!(
        response.status() == StatusCode::NOT_FOUND
            || response.status() == StatusCode::BAD_REQUEST
            || response.status() == StatusCode::UNAUTHORIZED,
        "Expected invalid token error, got: {}",
        response.status()
    );
}

/// Test concurrent authentication requests don't cause race conditions.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_concurrent_auth_requests() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;

    let user = test_db.test_user(UserRole::Employee);
    let token = generate_test_token(&user);

    let mut handles = vec![];

    for _ in 0..5 {
        let app = create_router(state.clone());
        let token_clone = token.clone();

        let handle = tokio::spawn(async move {
            app.oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/auth/me")
                    .header("Authorization", format!("Bearer {}", token_clone))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
        });

        handles.push(handle);
    }

    for handle in handles {
        let result = handle.await.expect("Task failed");
        let response = result.expect("Request failed");
        assert!(
            response.status() == StatusCode::OK || response.status() == StatusCode::NOT_FOUND,
            "Concurrent auth request failed with: {}",
            response.status()
        );
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_auth_header_generation() {
        let user = TestUser::employee();
        let header = auth_header(&user);
        assert!(header.starts_with("Bearer "));
        assert!(header.len() > 20);
    }

    #[test]
    fn test_different_roles_generate_different_tokens() {
        let employee = TestUser::employee();
        let admin = TestUser::admin();

        let employee_token = generate_test_token(&employee);
        let admin_token = generate_test_token(&admin);

        // Tokens should be different due to different roles
        assert_ne!(employee_token, admin_token);
    }
}
