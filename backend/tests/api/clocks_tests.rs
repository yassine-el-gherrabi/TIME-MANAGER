//! Clock API tests.
//!
//! Tests for clock in, clock out, and clock history endpoints.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::json;
use tower::ServiceExt;

use crate::api::test_helpers::*;

/// Test that clock in requires authentication
#[tokio::test]
#[ignore = "Requires database - run with: cargo test -- --ignored"]
async fn test_clock_in_requires_auth() {
    let db_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/timemanager_test".to_string());
    let pool = create_test_db_pool(&db_url);
    let state = create_test_state(pool).await;
    let app = create_test_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/clocks/in")
                .header("Content-Type", "application/json")
                .body(Body::from(json!({}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

/// Test that clock out requires authentication
#[tokio::test]
#[ignore = "Requires database - run with: cargo test -- --ignored"]
async fn test_clock_out_requires_auth() {
    let db_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/timemanager_test".to_string());
    let pool = create_test_db_pool(&db_url);
    let state = create_test_state(pool).await;
    let app = create_test_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/clocks/out")
                .header("Content-Type", "application/json")
                .body(Body::from(json!({}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

/// Test that clock status requires authentication
#[tokio::test]
#[ignore = "Requires database - run with: cargo test -- --ignored"]
async fn test_clock_status_requires_auth() {
    let db_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/timemanager_test".to_string());
    let pool = create_test_db_pool(&db_url);
    let state = create_test_state(pool).await;
    let app = create_test_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/clocks/status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

/// Test that clock history requires authentication
#[tokio::test]
#[ignore = "Requires database - run with: cargo test -- --ignored"]
async fn test_clock_history_requires_auth() {
    let db_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/timemanager_test".to_string());
    let pool = create_test_db_pool(&db_url);
    let state = create_test_state(pool).await;
    let app = create_test_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/clocks/history")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

/// Test clock in with valid auth but user not in database
#[tokio::test]
#[ignore = "Requires database - run with: cargo test -- --ignored"]
async fn test_clock_in_user_not_found() {
    let db_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/timemanager_test".to_string());
    let pool = create_test_db_pool(&db_url);
    let state = create_test_state(pool).await;
    let app = create_test_router(state);

    let user = TestUser::employee();
    let token = generate_test_token(&user);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/clocks/in")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(json!({}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return 404 if user doesn't exist, or other error
    assert!(
        response.status() == StatusCode::NOT_FOUND
            || response.status() == StatusCode::INTERNAL_SERVER_ERROR
            || response.status() == StatusCode::BAD_REQUEST,
        "Expected NOT_FOUND or error status, got {}",
        response.status()
    );
}

/// Test clock history with pagination parameters
#[tokio::test]
#[ignore = "Requires database with seeded data - run with: cargo test -- --ignored"]
async fn test_clock_history_with_pagination() {
    let db_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/timemanager_test".to_string());
    let pool = create_test_db_pool(&db_url);
    let state = create_test_state(pool).await;
    let app = create_test_router(state);

    let user = TestUser::employee();
    let token = generate_test_token(&user);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/clocks/history?page=1&per_page=10")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should work if user exists, 404 if not
    assert!(
        response.status() == StatusCode::OK || response.status() == StatusCode::NOT_FOUND,
        "Expected OK or NOT_FOUND, got {}",
        response.status()
    );
}

/// Test clock history with date filter
#[tokio::test]
#[ignore = "Requires database with seeded data - run with: cargo test -- --ignored"]
async fn test_clock_history_with_date_filter() {
    let db_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/timemanager_test".to_string());
    let pool = create_test_db_pool(&db_url);
    let state = create_test_state(pool).await;
    let app = create_test_router(state);

    let user = TestUser::employee();
    let token = generate_test_token(&user);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/clocks/history?start_date=2024-01-01&end_date=2024-12-31")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should work if user exists, 404 if not
    assert!(
        response.status() == StatusCode::OK || response.status() == StatusCode::NOT_FOUND,
        "Expected OK or NOT_FOUND, got {}",
        response.status()
    );
}

/// Test that admin can view all clocks
#[tokio::test]
#[ignore = "Requires database with seeded data - run with: cargo test -- --ignored"]
async fn test_admin_list_all_clocks() {
    let db_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/timemanager_test".to_string());
    let pool = create_test_db_pool(&db_url);
    let state = create_test_state(pool).await;
    let app = create_test_router(state);

    let admin = TestUser::admin();
    let token = generate_test_token(&admin);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/clocks")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Admin should be able to list clocks (200) or get user not found (404)
    assert!(
        response.status() == StatusCode::OK || response.status() == StatusCode::NOT_FOUND,
        "Expected OK or NOT_FOUND, got {}",
        response.status()
    );
}

/// Test that employee cannot access admin clock list
#[tokio::test]
#[ignore = "Requires database - run with: cargo test -- --ignored"]
async fn test_employee_cannot_list_all_clocks() {
    let db_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/timemanager_test".to_string());
    let pool = create_test_db_pool(&db_url);
    let state = create_test_state(pool).await;
    let app = create_test_router(state);

    let employee = TestUser::employee();
    let token = generate_test_token(&employee);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/clocks")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Employee should get forbidden (403) or not found (404)
    assert!(
        response.status() == StatusCode::FORBIDDEN || response.status() == StatusCode::NOT_FOUND,
        "Expected FORBIDDEN or NOT_FOUND, got {}",
        response.status()
    );
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use crate::api::test_helpers::fixtures;

    #[test]
    fn test_fixtures() {
        let org_id = fixtures::test_org_id();
        let user_id = fixtures::test_user_id();
        let team_id = fixtures::test_team_id();

        // Fixtures should be deterministic
        assert_eq!(org_id, fixtures::test_org_id());
        assert_eq!(user_id, fixtures::test_user_id());
        assert_eq!(team_id, fixtures::test_team_id());

        // All should be different
        assert_ne!(org_id, user_id);
        assert_ne!(user_id, team_id);
    }

    #[test]
    fn test_date_fixtures() {
        let today = fixtures::today();
        let tomorrow = fixtures::days_from_now(1);
        let yesterday = fixtures::days_from_now(-1);

        assert!(tomorrow > today);
        assert!(yesterday < today);
    }
}
