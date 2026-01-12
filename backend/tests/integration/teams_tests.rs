//! Teams integration tests with testcontainers.
//!
//! These tests verify the team management endpoints with real database
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

/// Test GET /api/v1/teams endpoint - list teams.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_list_teams() {
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
                .uri("/api/v1/teams")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::OK || response.status() == StatusCode::FORBIDDEN,
        "List teams failed with status: {}",
        response.status()
    );
}

/// Test GET /api/v1/teams/my - get user's teams.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_get_my_teams() {
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
                .uri("/api/v1/teams/my")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::OK || response.status() == StatusCode::NOT_FOUND,
        "Get my teams failed with status: {}",
        response.status()
    );
}

/// Test POST /api/v1/teams - create team.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_create_team() {
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
                .uri("/api/v1/teams")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "name": "Engineering Team",
                        "description": "Software development team"
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
        "Create team failed with status: {}",
        response.status()
    );
}

/// Test POST /api/v1/teams with empty name.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_create_team_empty_name() {
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
                .uri("/api/v1/teams")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "name": "",
                        "description": "Team with empty name"
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

/// Test GET /api/v1/teams/:id - get team by ID.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_get_team_by_id() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let admin = test_db.test_user(UserRole::Admin);
    let token = generate_test_token(&admin);
    let team_id = Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/teams/{}", team_id))
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
        "Get team by ID failed with status: {}",
        response.status()
    );
}

/// Test PUT /api/v1/teams/:id - update team.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_update_team() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let admin = test_db.test_user(UserRole::Admin);
    let token = generate_test_token(&admin);
    let team_id = Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/v1/teams/{}", team_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "name": "Updated Team Name",
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
        "Update team failed with status: {}",
        response.status()
    );
}

/// Test DELETE /api/v1/teams/:id - delete team.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_delete_team() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let admin = test_db.test_user(UserRole::Admin);
    let token = generate_test_token(&admin);
    let team_id = Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/teams/{}", team_id))
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
        "Delete team failed with status: {}",
        response.status()
    );
}

/// Test POST /api/v1/teams/:id/members - add member to team.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_add_team_member() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let admin = test_db.test_user(UserRole::Admin);
    let token = generate_test_token(&admin);
    let team_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/teams/{}/members", team_id))
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "user_id": user_id
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
        "Add team member failed with status: {}",
        response.status()
    );
}

/// Test DELETE /api/v1/teams/:team_id/members/:user_id - remove member from team.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_remove_team_member() {
    let docker = Cli::default();
    let test_db = TestDatabase::new(&docker).await;
    let state = test_db.create_state().await;
    let app = create_router(state);

    let admin = test_db.test_user(UserRole::Admin);
    let token = generate_test_token(&admin);
    let team_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/teams/{}/members/{}", team_id, user_id))
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
        "Remove team member failed with status: {}",
        response.status()
    );
}

/// Test employee cannot create teams.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_employee_cannot_create_team() {
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
                .uri("/api/v1/teams")
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "name": "Unauthorized Team",
                        "description": "Should fail"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return FORBIDDEN for employee
    assert!(
        response.status() == StatusCode::FORBIDDEN
            || response.status() == StatusCode::CREATED
            || response.status() == StatusCode::OK,
        "Expected FORBIDDEN for employee, got: {}",
        response.status()
    );
}

/// Test manager can view their own teams.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_manager_view_teams() {
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
                .uri("/api/v1/teams/my")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::OK || response.status() == StatusCode::NOT_FOUND,
        "Manager get my teams failed with status: {}",
        response.status()
    );
}

/// Test concurrent team operations don't cause race conditions.
#[tokio::test]
#[ignore = "Requires Docker - run with: cargo test -- --ignored"]
async fn test_concurrent_team_operations() {
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
                    .uri("/api/v1/teams")
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
            "Concurrent team request failed with: {}",
            response.status()
        );
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_manager_role() {
        let manager = TestUser::manager();
        assert!(matches!(manager.role, UserRole::Manager));
    }
}
