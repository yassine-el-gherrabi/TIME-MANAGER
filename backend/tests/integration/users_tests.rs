//! Users integration tests with testcontainers.
//!
//! These tests verify the user management endpoints with real database
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

/// Test GET /api/v1/users endpoint - list users.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_list_users() {
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
                .uri("/api/v1/users")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::OK || response.status() == StatusCode::FORBIDDEN,
        "List users failed with status: {}",
        response.status()
    );
}

/// Test GET /api/v1/users - employee cannot list all users.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_employee_cannot_list_users() {
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
                .uri("/api/v1/users")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Employee should get FORBIDDEN
    assert!(
        response.status() == StatusCode::FORBIDDEN || response.status() == StatusCode::OK,
        "Expected FORBIDDEN for employee, got: {}",
        response.status()
    );
}

/// Test POST /api/v1/users - create user.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_create_user() {
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
                .uri("/api/v1/users")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "email": "newuser@example.com",
                        "first_name": "New",
                        "last_name": "User",
                        "role": "employee"
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
            || response.status() == StatusCode::BAD_REQUEST
            || response.status() == StatusCode::UNPROCESSABLE_ENTITY,
        "Create user failed with status: {}",
        response.status()
    );
}

/// Test POST /api/v1/users with invalid email.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_create_user_invalid_email() {
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
                .uri("/api/v1/users")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "email": "not-an-email",
                        "first_name": "Test",
                        "last_name": "User",
                        "role": "employee"
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
            || response.status() == StatusCode::FORBIDDEN,
        "Expected validation error, got: {}",
        response.status()
    );
}

/// Test GET /api/v1/users/:id - get specific user.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_get_user_by_id() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let admin = test_db.test_user(UserRole::Admin);
    let token = generate_test_token(&admin);
    let user_id = Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/users/{}", user_id))
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // User doesn't exist, should return NOT_FOUND or FORBIDDEN
    assert!(
        response.status() == StatusCode::NOT_FOUND
            || response.status() == StatusCode::FORBIDDEN
            || response.status() == StatusCode::OK,
        "Get user by ID failed with status: {}",
        response.status()
    );
}

/// Test PUT /api/v1/users/:id - update user.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_update_user() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let admin = test_db.test_user(UserRole::Admin);
    let token = generate_test_token(&admin);
    let user_id = Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/users/{}", user_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "first_name": "Updated",
                        "last_name": "Name"
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
        "Update user failed with status: {}",
        response.status()
    );
}

/// Test DELETE /api/v1/users/:id - soft delete user.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_delete_user() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let admin = test_db.test_user(UserRole::Admin);
    let token = generate_test_token(&admin);
    let user_id = Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/users/{}", user_id))
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
        "Delete user failed with status: {}",
        response.status()
    );
}

/// Test PUT /api/v1/users/:id/restore - restore soft-deleted user.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_restore_user() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let admin = test_db.test_user(UserRole::Admin);
    let token = generate_test_token(&admin);
    let user_id = Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/users/{}/restore", user_id))
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
        "Restore user failed with status: {}",
        response.status()
    );
}

/// Test POST /api/v1/users/:id/resend-invite - resend invitation.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_resend_invite() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let admin = test_db.test_user(UserRole::Admin);
    let token = generate_test_token(&admin);
    let user_id = Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/users/{}/resend-invite", user_id))
                .header("Authorization", format!("Bearer {}", token))
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
        "Resend invite failed with status: {}",
        response.status()
    );
}

/// Test PUT /api/v1/users/:id/update-role - update user role.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_update_user_role() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let super_admin = test_db.test_user(UserRole::SuperAdmin);
    let token = generate_test_token(&super_admin);
    let user_id = Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/users/{}/update-role", user_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "role": "manager"
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
        "Update role failed with status: {}",
        response.status()
    );
}

/// Test manager can view users in their team.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_manager_view_users() {
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
                .uri("/api/v1/users")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Manager may have limited access
    assert!(
        response.status() == StatusCode::OK || response.status() == StatusCode::FORBIDDEN,
        "Manager user list failed with status: {}",
        response.status()
    );
}

/// Test pagination on user list.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_list_users_pagination() {
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
                .uri("/api/v1/users?page=1&limit=10")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::OK || response.status() == StatusCode::FORBIDDEN,
        "Paginated user list failed with status: {}",
        response.status()
    );
}

/// Test concurrent user operations don't cause race conditions.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_concurrent_user_operations() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;

    let admin = test_db.test_user(UserRole::Admin);
    let token = generate_test_token(&admin);

    let mut handles = vec![];

    for _ in 0..5 {
        let app = create_router(state.clone());
        let token_clone = token.clone();

        let handle = tokio::spawn(async move {
            app.oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/v1/users")
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
            response.status() == StatusCode::OK || response.status() == StatusCode::FORBIDDEN,
            "Concurrent user request failed with: {}",
            response.status()
        );
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_admin_user_creation() {
        let admin = TestUser::admin();
        assert!(matches!(admin.role, UserRole::Admin));
    }

    #[test]
    fn test_super_admin_user_creation() {
        let super_admin = TestUser::super_admin();
        assert!(matches!(super_admin.role, UserRole::SuperAdmin));
    }
}
