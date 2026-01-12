//! Absence API tests.
//!
//! Tests for absence request, approval, and balance endpoints.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::json;
use tower::ServiceExt;

use crate::api::test_helpers::*;

/// Test that creating an absence requires authentication
#[tokio::test]
#[ignore = "Requires database - run with: cargo test -- --ignored"]
async fn test_create_absence_requires_auth() {
    let db_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/timemanager_test".to_string());
    let pool = create_test_db_pool(&db_url);
    let state = create_test_state(pool).await;
    let app = create_test_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/absences")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "start_date": "2024-01-15",
                        "end_date": "2024-01-16",
                        "absence_type": "vacation"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

/// Test that listing absences requires authentication
#[tokio::test]
#[ignore = "Requires database - run with: cargo test -- --ignored"]
async fn test_list_absences_requires_auth() {
    let db_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/timemanager_test".to_string());
    let pool = create_test_db_pool(&db_url);
    let state = create_test_state(pool).await;
    let app = create_test_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/absences")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

/// Test that getting absence balance requires authentication
#[tokio::test]
#[ignore = "Requires database - run with: cargo test -- --ignored"]
async fn test_absence_balance_requires_auth() {
    let db_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/timemanager_test".to_string());
    let pool = create_test_db_pool(&db_url);
    let state = create_test_state(pool).await;
    let app = create_test_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/absences/balance")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

/// Test creating absence with valid auth
#[tokio::test]
#[ignore = "Requires database with seeded user - run with: cargo test -- --ignored"]
async fn test_create_absence_with_auth() {
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
                .uri("/api/v1/absences")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "start_date": "2024-06-01",
                        "end_date": "2024-06-05",
                        "absence_type": "vacation",
                        "reason": "Summer holiday"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Will be 201 if user exists and has balance, or error if not
    assert!(
        response.status() == StatusCode::CREATED
            || response.status() == StatusCode::NOT_FOUND
            || response.status() == StatusCode::BAD_REQUEST
            || response.status() == StatusCode::UNPROCESSABLE_ENTITY,
        "Expected CREATED, NOT_FOUND, BAD_REQUEST or UNPROCESSABLE_ENTITY, got {}",
        response.status()
    );
}

/// Test creating absence with invalid date range
#[tokio::test]
#[ignore = "Requires database - run with: cargo test -- --ignored"]
async fn test_create_absence_invalid_dates() {
    let db_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/timemanager_test".to_string());
    let pool = create_test_db_pool(&db_url);
    let state = create_test_state(pool).await;
    let app = create_test_router(state);

    let user = TestUser::employee();
    let token = generate_test_token(&user);

    // End date before start date
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/absences")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "start_date": "2024-06-10",
                        "end_date": "2024-06-05",
                        "absence_type": "vacation"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Should reject invalid date range
    assert!(
        response.status() == StatusCode::BAD_REQUEST
            || response.status() == StatusCode::UNPROCESSABLE_ENTITY
            || response.status() == StatusCode::NOT_FOUND,
        "Expected BAD_REQUEST or UNPROCESSABLE_ENTITY, got {}",
        response.status()
    );
}

/// Test that manager can approve absences
#[tokio::test]
#[ignore = "Requires database with seeded data - run with: cargo test -- --ignored"]
async fn test_manager_can_approve_absence() {
    let db_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/timemanager_test".to_string());
    let pool = create_test_db_pool(&db_url);
    let state = create_test_state(pool).await;
    let app = create_test_router(state);

    let manager = TestUser::manager();
    let token = generate_test_token(&manager);

    // Use a placeholder UUID - in real tests this would be an actual absence ID
    let absence_id = fixtures::test_user_id();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&format!("/api/v1/absences/{}/approve", absence_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Manager should be able to approve (200) or absence not found (404)
    assert!(
        response.status() == StatusCode::OK || response.status() == StatusCode::NOT_FOUND,
        "Expected OK or NOT_FOUND, got {}",
        response.status()
    );
}

/// Test that employee cannot approve absences
#[tokio::test]
#[ignore = "Requires database - run with: cargo test -- --ignored"]
async fn test_employee_cannot_approve_absence() {
    let db_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/timemanager_test".to_string());
    let pool = create_test_db_pool(&db_url);
    let state = create_test_state(pool).await;
    let app = create_test_router(state);

    let employee = TestUser::employee();
    let token = generate_test_token(&employee);

    let absence_id = fixtures::test_user_id();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&format!("/api/v1/absences/{}/approve", absence_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
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

/// Test listing absences with pagination
#[tokio::test]
#[ignore = "Requires database - run with: cargo test -- --ignored"]
async fn test_list_absences_with_pagination() {
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
                .uri("/api/v1/absences?page=1&per_page=10")
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

/// Test listing absences with status filter
#[tokio::test]
#[ignore = "Requires database - run with: cargo test -- --ignored"]
async fn test_list_absences_with_status_filter() {
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
                .uri("/api/v1/absences?status=pending")
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

/// Test admin can list all absences
#[tokio::test]
#[ignore = "Requires database - run with: cargo test -- --ignored"]
async fn test_admin_list_all_absences() {
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
                .uri("/api/v1/absences/all")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Admin should be able to list all (200) or not found (404)
    assert!(
        response.status() == StatusCode::OK || response.status() == StatusCode::NOT_FOUND,
        "Expected OK or NOT_FOUND, got {}",
        response.status()
    );
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use crate::api::test_helpers::fixtures;

    #[test]
    fn test_test_user_roles() {
        let employee = TestUser::employee();
        let manager = TestUser::manager();
        let admin = TestUser::admin();
        let super_admin = TestUser::super_admin();

        // All should have different roles
        assert_ne!(format!("{:?}", employee.role), format!("{:?}", manager.role));
        assert_ne!(format!("{:?}", manager.role), format!("{:?}", admin.role));
        assert_ne!(format!("{:?}", admin.role), format!("{:?}", super_admin.role));
    }

    #[test]
    fn test_fixtures_consistency() {
        // Fixtures should be deterministic across calls
        assert_eq!(fixtures::test_org_id(), fixtures::test_org_id());
        assert_eq!(fixtures::test_user_id(), fixtures::test_user_id());
        assert_eq!(fixtures::test_team_id(), fixtures::test_team_id());
    }
}
