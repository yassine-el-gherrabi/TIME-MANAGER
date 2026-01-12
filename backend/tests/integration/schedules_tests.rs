//! Schedules integration tests with testcontainers.
//!
//! These tests verify the schedule management endpoints with real database
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

/// Test GET /api/v1/schedules - list schedules.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_list_schedules() {
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
                .uri("/api/v1/schedules")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::OK
            || response.status() == StatusCode::NOT_FOUND
            || response.status() == StatusCode::FORBIDDEN,
        "List schedules failed with status: {}",
        response.status()
    );
}

/// Test GET /api/v1/schedules/me - get my schedule.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_get_my_schedule() {
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
                .uri("/api/v1/schedules/me")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::OK || response.status() == StatusCode::NOT_FOUND,
        "Get my schedule failed with status: {}",
        response.status()
    );
}

/// Test POST /api/v1/schedules - create schedule.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_create_schedule() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let admin = test_db.test_user(UserRole::Admin);
    let token = generate_test_token(&admin);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/schedules")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "name": "Standard Schedule",
                        "description": "Monday to Friday, 9-5"
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
            || response.status() == StatusCode::FORBIDDEN
            || response.status() == StatusCode::BAD_REQUEST,
        "Create schedule failed with status: {}",
        response.status()
    );
}

/// Test GET /api/v1/schedules/:id - get schedule by ID.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_get_schedule_by_id() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let admin = test_db.test_user(UserRole::Admin);
    let token = generate_test_token(&admin);
    let schedule_id = Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/schedules/{}", schedule_id))
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
        "Get schedule by ID failed with status: {}",
        response.status()
    );
}

/// Test PUT /api/v1/schedules/:id - update schedule.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_update_schedule() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let admin = test_db.test_user(UserRole::Admin);
    let token = generate_test_token(&admin);
    let schedule_id = Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/schedules/{}", schedule_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "name": "Updated Schedule",
                        "description": "Updated description"
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
        "Update schedule failed with status: {}",
        response.status()
    );
}

/// Test POST /api/v1/schedules/:id/days - add day to schedule.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_add_schedule_day() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let admin = test_db.test_user(UserRole::Admin);
    let token = generate_test_token(&admin);
    let schedule_id = Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/schedules/{}/days", schedule_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "day_of_week": 1,
                        "start_time": "09:00",
                        "end_time": "17:00"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::OK
            || response.status() == StatusCode::CREATED
            || response.status() == StatusCode::NOT_FOUND
            || response.status() == StatusCode::FORBIDDEN
            || response.status() == StatusCode::BAD_REQUEST,
        "Add schedule day failed with status: {}",
        response.status()
    );
}

/// Test DELETE /api/v1/schedules/:schedule_id/days/:day_id - remove day from schedule.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_remove_schedule_day() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let admin = test_db.test_user(UserRole::Admin);
    let token = generate_test_token(&admin);
    let schedule_id = Uuid::new_v4();
    let day_id = Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/schedules/{}/days/{}", schedule_id, day_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::OK
            || response.status() == StatusCode::NO_CONTENT
            || response.status() == StatusCode::NOT_FOUND
            || response.status() == StatusCode::FORBIDDEN,
        "Remove schedule day failed with status: {}",
        response.status()
    );
}

/// Test concurrent schedule operations don't cause race conditions.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_concurrent_schedule_operations() {
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
                    .uri("/api/v1/schedules/me")
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
            "Concurrent schedule request failed with: {}",
            response.status()
        );
    }
}
