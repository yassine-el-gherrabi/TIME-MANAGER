//! Authentication API tests.
//!
//! Tests for login, logout, token refresh, and session management endpoints.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::json;
use tower::ServiceExt;

use crate::api::test_helpers::*;

/// Test that login with missing credentials returns 400 Bad Request
#[tokio::test]
#[ignore = "Requires database - run with: cargo test -- --ignored"]
async fn test_login_missing_credentials() {
    let db_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/timemanager_test".to_string());
    let pool = create_test_db_pool(&db_url);
    let state = create_test_state(pool).await;
    let app = create_test_router(state);

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

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

/// Test that login with invalid email format returns 400 Bad Request
#[tokio::test]
#[ignore = "Requires database - run with: cargo test -- --ignored"]
async fn test_login_invalid_email_format() {
    let db_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/timemanager_test".to_string());
    let pool = create_test_db_pool(&db_url);
    let state = create_test_state(pool).await;
    let app = create_test_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "email": "not-an-email",
                        "password": "password123"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Should fail validation
    assert!(
        response.status() == StatusCode::BAD_REQUEST
            || response.status() == StatusCode::UNAUTHORIZED
    );
}

/// Test that login with wrong credentials returns 401 Unauthorized
#[tokio::test]
#[ignore = "Requires database - run with: cargo test -- --ignored"]
async fn test_login_wrong_credentials() {
    let db_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/timemanager_test".to_string());
    let pool = create_test_db_pool(&db_url);
    let state = create_test_state(pool).await;
    let app = create_test_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "email": "nonexistent@example.com",
                        "password": "wrongpassword"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

/// Test that /me endpoint requires authentication
#[tokio::test]
#[ignore = "Requires database - run with: cargo test -- --ignored"]
async fn test_me_requires_auth() {
    let db_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/timemanager_test".to_string());
    let pool = create_test_db_pool(&db_url);
    let state = create_test_state(pool).await;
    let app = create_test_router(state);

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

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

/// Test that /me endpoint works with valid token
#[tokio::test]
#[ignore = "Requires database with seeded user - run with: cargo test -- --ignored"]
async fn test_me_with_valid_token() {
    let db_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/timemanager_test".to_string());
    let pool = create_test_db_pool(&db_url);
    let state = create_test_state(pool).await;
    let app = create_test_router(state);

    // Note: This test requires a user to exist in the database with matching ID
    let user = TestUser::employee();
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

    // Will be 404 if user doesn't exist, or 200 if user exists
    assert!(
        response.status() == StatusCode::OK || response.status() == StatusCode::NOT_FOUND,
        "Expected OK or NOT_FOUND, got {}",
        response.status()
    );
}

/// Test that logout requires authentication
#[tokio::test]
#[ignore = "Requires database - run with: cargo test -- --ignored"]
async fn test_logout_requires_auth() {
    let db_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/timemanager_test".to_string());
    let pool = create_test_db_pool(&db_url);
    let state = create_test_state(pool).await;
    let app = create_test_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/logout")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

/// Test that refresh token endpoint requires a token
#[tokio::test]
#[ignore = "Requires database - run with: cargo test -- --ignored"]
async fn test_refresh_requires_token() {
    let db_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/timemanager_test".to_string());
    let pool = create_test_db_pool(&db_url);
    let state = create_test_state(pool).await;
    let app = create_test_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/refresh")
                .header("Content-Type", "application/json")
                .body(Body::from(json!({}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

/// Test that refresh with invalid token returns 401
#[tokio::test]
#[ignore = "Requires database - run with: cargo test -- --ignored"]
async fn test_refresh_invalid_token() {
    let db_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/timemanager_test".to_string());
    let pool = create_test_db_pool(&db_url);
    let state = create_test_state(pool).await;
    let app = create_test_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/refresh")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "refresh_token": "invalid-token"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_token_generation() {
        let user = TestUser::employee();
        let token = generate_test_token(&user);

        // Token should be a valid JWT format (3 parts separated by dots)
        let parts: Vec<&str> = token.split('.').collect();
        assert_eq!(parts.len(), 3, "JWT should have 3 parts");

        // Each part should be base64url encoded
        assert!(parts[0].starts_with("eyJ"), "Header should start with eyJ");
    }

    #[test]
    fn test_auth_header_format() {
        let user = TestUser::admin();
        let header = auth_header(&user);

        assert!(header.starts_with("Bearer "), "Should have Bearer prefix");
        let token = header.strip_prefix("Bearer ").unwrap();
        assert!(!token.is_empty(), "Token should not be empty");
    }
}
