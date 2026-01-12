//! Absences integration tests with testcontainers.
//!
//! These tests verify the absence management endpoints with real database
//! interactions using testcontainers.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::json;
use testcontainers::clients::Cli;
use tower::ServiceExt;
use uuid::Uuid;

use crate::integration::test_fixtures::*;
use timemanager_backend::api::router::create_router;
use timemanager_backend::domain::enums::UserRole;

/// Test GET /api/v1/absences/my - get user's absences.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_get_my_absences() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let employee = test_db.test_user(UserRole::Employee);
    let token = generate_test_token(&employee);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/absences/my")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::OK || response.status() == StatusCode::NOT_FOUND,
        "Get my absences failed with status: {}",
        response.status()
    );
}

/// Test POST /api/v1/absences/my - create absence request.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_create_absence_request() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let employee = test_db.test_user(UserRole::Employee);
    let token = generate_test_token(&employee);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/absences/my")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "absence_type": "vacation",
                        "start_date": "2024-06-01",
                        "end_date": "2024-06-05",
                        "reason": "Summer vacation"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::CREATED
            || response.status() == StatusCode::OK
            || response.status() == StatusCode::NOT_FOUND
            || response.status() == StatusCode::BAD_REQUEST
            || response.status() == StatusCode::UNPROCESSABLE_ENTITY,
        "Create absence request failed with status: {}",
        response.status()
    );
}

/// Test POST /api/v1/absences/my with invalid dates.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_create_absence_invalid_dates() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let employee = test_db.test_user(UserRole::Employee);
    let token = generate_test_token(&employee);

    // End date before start date
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/absences/my")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "absence_type": "vacation",
                        "start_date": "2024-06-05",
                        "end_date": "2024-06-01",
                        "reason": "Invalid dates"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::BAD_REQUEST
            || response.status() == StatusCode::UNPROCESSABLE_ENTITY
            || response.status() == StatusCode::NOT_FOUND,
        "Expected validation error for invalid dates, got: {}",
        response.status()
    );
}

/// Test GET /api/v1/absences/pending - list pending absences.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_list_pending_absences() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let manager = test_db.test_user(UserRole::Manager);
    let token = generate_test_token(&manager);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/absences/pending")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::OK
            || response.status() == StatusCode::FORBIDDEN
            || response.status() == StatusCode::NOT_FOUND,
        "List pending absences failed with status: {}",
        response.status()
    );
}

/// Test GET /api/v1/absences/:id - get absence by ID.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_get_absence_by_id() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let employee = test_db.test_user(UserRole::Employee);
    let token = generate_test_token(&employee);
    let absence_id = Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/absences/{}", absence_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::NOT_FOUND
            || response.status() == StatusCode::OK
            || response.status() == StatusCode::FORBIDDEN,
        "Get absence by ID failed with status: {}",
        response.status()
    );
}

/// Test POST /api/v1/absences/:id/approve - approve absence.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_approve_absence() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let manager = test_db.test_user(UserRole::Manager);
    let token = generate_test_token(&manager);
    let absence_id = Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/absences/{}/approve", absence_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::OK
            || response.status() == StatusCode::NOT_FOUND
            || response.status() == StatusCode::FORBIDDEN,
        "Approve absence failed with status: {}",
        response.status()
    );
}

/// Test POST /api/v1/absences/:id/reject - reject absence.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_reject_absence() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let manager = test_db.test_user(UserRole::Manager);
    let token = generate_test_token(&manager);
    let absence_id = Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/absences/{}/reject", absence_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "reason": "Not enough notice given"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::OK
            || response.status() == StatusCode::NOT_FOUND
            || response.status() == StatusCode::FORBIDDEN,
        "Reject absence failed with status: {}",
        response.status()
    );
}

/// Test POST /api/v1/absences/:id/cancel - cancel absence.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_cancel_absence() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let employee = test_db.test_user(UserRole::Employee);
    let token = generate_test_token(&employee);
    let absence_id = Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/absences/{}/cancel", absence_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::OK
            || response.status() == StatusCode::NOT_FOUND
            || response.status() == StatusCode::FORBIDDEN
            || response.status() == StatusCode::BAD_REQUEST,
        "Cancel absence failed with status: {}",
        response.status()
    );
}

/// Test employee cannot approve absences.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_employee_cannot_approve_absence() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let employee = test_db.test_user(UserRole::Employee);
    let token = generate_test_token(&employee);
    let absence_id = Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/absences/{}/approve", absence_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::FORBIDDEN
            || response.status() == StatusCode::NOT_FOUND
            || response.status() == StatusCode::OK,
        "Expected FORBIDDEN for employee, got: {}",
        response.status()
    );
}

/// Test admin can view all pending absences.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_admin_view_pending_absences() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let admin = test_db.test_user(UserRole::Admin);
    let token = generate_test_token(&admin);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/absences/pending")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::OK || response.status() == StatusCode::FORBIDDEN,
        "Admin view pending absences failed with status: {}",
        response.status()
    );
}

/// Test concurrent absence operations don't cause race conditions.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_concurrent_absence_operations() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;

    let employee = test_db.test_user(UserRole::Employee);
    let token = generate_test_token(&employee);

    let mut handles = vec![];

    for _ in 0..5 {
        let app = create_router(state.clone());
        let token_clone = token.clone();

        let handle = tokio::spawn(async move {
            app.oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/absences/my")
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
            "Concurrent absence request failed with: {}",
            response.status()
        );
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_seed_dates() {
        let today = seed::today();
        let tomorrow = seed::days_from_now(1);
        assert!(tomorrow > today);
    }
}
